use booking_a_stay::application_state::AppState;
use booking_a_stay::capabilities::book_stay::process::{BookStayResponse, process};
use booking_a_stay::capabilities::book_stay::request::{BookStay, Stay};
use booking_a_stay::capabilities::book_stay::result::BookingRejected;
use booking_a_stay::providers::Providers;
use chrono::NaiveDate;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::{Executor, Row};
use tokio::sync::Mutex;
use uuid::Uuid;

static TEST_DATABASE_LOCK: Mutex<()> = Mutex::const_new(());

#[tokio::test]
async fn confirms_reservation_and_records_listing_unavailable_nights() {
    let _guard = TEST_DATABASE_LOCK.lock().await;
    let pool = prepare_database().await;
    let fixture = seed_bookable_listing(&pool, "eligible", "bookable").await;
    let state = app_state(pool.clone());

    let request = book_stay_request(fixture.guest_id, fixture.listing_id);
    let response = process(request, &state)
        .await
        .expect("process should not fail technically");

    let reservation_id = match response {
        BookStayResponse::Confirmed { reservation_id } => reservation_id,
        BookStayResponse::Rejected(rejection) => {
            panic!("expected confirmation, got rejection: {rejection:?}");
        }
    };

    let reservation_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM reservations")
        .fetch_one(&pool)
        .await
        .expect("reservations count should be readable");
    assert_eq!(reservation_count, 1);
    assert_eq!(
        stored_reservation_id(&pool).await,
        reservation_id,
        "RPU-generated reservation id should be recorded"
    );

    let unavailable_nights = unavailable_nights(&pool, fixture.listing_id).await;
    assert_eq!(
        unavailable_nights,
        vec![date(2026, 8, 1), date(2026, 8, 2), date(2026, 8, 3)]
    );
}

#[tokio::test]
async fn does_not_occupy_the_check_out_date() {
    let _guard = TEST_DATABASE_LOCK.lock().await;
    let pool = prepare_database().await;
    let fixture = seed_bookable_listing(&pool, "eligible", "bookable").await;
    let state = app_state(pool.clone());

    let request = book_stay_request(fixture.guest_id, fixture.listing_id);
    process(request, &state)
        .await
        .expect("process should not fail technically");

    let check_out_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM listing_unavailable_nights WHERE night = $1")
            .bind(date(2026, 8, 4))
            .fetch_one(&pool)
            .await
            .expect("check-out count should be readable");

    assert_eq!(check_out_count, 0);
}

#[tokio::test]
async fn rejects_when_guest_is_blocked() {
    let _guard = TEST_DATABASE_LOCK.lock().await;
    let pool = prepare_database().await;
    let fixture = seed_bookable_listing(&pool, "blocked", "bookable").await;
    let state = app_state(pool.clone());

    let request = book_stay_request(fixture.guest_id, fixture.listing_id);
    let response = process(request, &state)
        .await
        .expect("process should not fail technically");

    assert_eq!(
        response,
        BookStayResponse::Rejected(BookingRejected::GuestBlocked)
    );
    assert_no_reservation_or_unavailable_nights(&pool).await;
}

#[tokio::test]
async fn rejects_when_listing_is_disabled() {
    let _guard = TEST_DATABASE_LOCK.lock().await;
    let pool = prepare_database().await;
    let fixture = seed_bookable_listing(&pool, "eligible", "disabled").await;
    let state = app_state(pool.clone());

    let request = book_stay_request(fixture.guest_id, fixture.listing_id);
    let response = process(request, &state)
        .await
        .expect("process should not fail technically");

    assert_eq!(
        response,
        BookStayResponse::Rejected(BookingRejected::ListingDisabled)
    );
    assert_no_reservation_or_unavailable_nights(&pool).await;
}

#[tokio::test]
async fn rejects_when_requested_night_is_already_unavailable() {
    let _guard = TEST_DATABASE_LOCK.lock().await;
    let pool = prepare_database().await;
    let fixture = seed_bookable_listing(&pool, "eligible", "bookable").await;
    let state = app_state(pool.clone());

    sqlx::query(
        r#"
        INSERT INTO listing_unavailable_nights (
            listing_id,
            night,
            unavailability_type,
            reason
        )
        VALUES ($1, $2, 'host_block', 'maintenance')
        "#,
    )
    .bind(fixture.listing_id)
    .bind(date(2026, 8, 2))
    .execute(&pool)
    .await
    .expect("host block should be inserted");

    let request = book_stay_request(fixture.guest_id, fixture.listing_id);
    let response = process(request, &state)
        .await
        .expect("process should not fail technically");

    assert_eq!(
        response,
        BookStayResponse::Rejected(BookingRejected::ListingUnavailable)
    );

    let reservation_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM reservations")
        .fetch_one(&pool)
        .await
        .expect("reservations count should be readable");
    assert_eq!(reservation_count, 0);
}

#[tokio::test]
async fn two_overlapping_booking_requests_cannot_both_confirm() {
    let _guard = TEST_DATABASE_LOCK.lock().await;
    let pool = prepare_database().await;
    let fixture = seed_bookable_listing(&pool, "eligible", "bookable").await;
    let state = app_state(pool.clone());

    let first_request = book_stay_request(fixture.guest_id, fixture.listing_id);
    let second_request = book_stay_request(fixture.guest_id, fixture.listing_id);

    let first_state = state.clone();
    let second_state = state.clone();

    let (first, second) = tokio::join!(
        process(first_request, &first_state),
        process(second_request, &second_state)
    );

    let responses = vec![
        first.expect("first process should not fail technically"),
        second.expect("second process should not fail technically"),
    ];

    let confirmed_count = responses
        .iter()
        .filter(|response| matches!(response, BookStayResponse::Confirmed { .. }))
        .count();
    let unavailable_rejection_count = responses
        .iter()
        .filter(|response| {
            matches!(
                response,
                BookStayResponse::Rejected(BookingRejected::ListingUnavailable)
            )
        })
        .count();

    assert_eq!(confirmed_count, 1);
    assert_eq!(unavailable_rejection_count, 1);

    let unavailable_row_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM listing_unavailable_nights")
            .fetch_one(&pool)
            .await
            .expect("unavailable night count should be readable");
    let distinct_unavailable_row_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT (listing_id, night)) FROM listing_unavailable_nights",
    )
    .fetch_one(&pool)
    .await
    .expect("distinct unavailable night count should be readable");

    assert_eq!(unavailable_row_count, 3);
    assert_eq!(distinct_unavailable_row_count, 3);
}

async fn prepare_database() -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url())
        .await
        .expect("test database should be reachable");

    pool.execute(
        r#"
        DROP SCHEMA public CASCADE;
        CREATE SCHEMA public;
        GRANT ALL ON SCHEMA public TO PUBLIC;
        "#,
    )
    .await
    .expect("test database schema should reset");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("migrations should run");

    pool
}

fn database_url() -> String {
    std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/booking_a_stay".to_owned())
}

#[derive(Debug, Clone, Copy)]
struct BookingFixture {
    guest_id: Uuid,
    listing_id: Uuid,
}

async fn seed_bookable_listing(
    pool: &PgPool,
    guest_eligibility: &str,
    listing_status: &str,
) -> BookingFixture {
    let fixture = BookingFixture {
        guest_id: uuid(1),
        listing_id: uuid(2),
    };

    sqlx::query(
        r#"
        INSERT INTO guests (id, email, full_name, booking_eligibility)
        VALUES ($1, 'guest@example.test', 'Test Guest', $2)
        "#,
    )
    .bind(fixture.guest_id)
    .bind(guest_eligibility)
    .execute(pool)
    .await
    .expect("guest should be inserted");

    sqlx::query(
        r#"
        INSERT INTO listings (
            id,
            host_id,
            title,
            max_guests,
            min_nights,
            max_nights,
            booking_status
        )
        VALUES ($1, $2, 'Test Listing', 4, 2, 7, $3)
        "#,
    )
    .bind(fixture.listing_id)
    .bind(uuid(4))
    .bind(listing_status)
    .execute(pool)
    .await
    .expect("listing should be inserted");

    fixture
}

fn book_stay_request(guest_id: Uuid, listing_id: Uuid) -> BookStay {
    BookStay {
        guest_id,
        listing_id,
        stay: Stay {
            check_in: date(2026, 8, 1),
            check_out: date(2026, 8, 4),
        },
        guest_count: 2,
    }
}

fn app_state(pool: PgPool) -> AppState {
    AppState::new(pool, Providers::new())
}

async fn stored_reservation_id(pool: &PgPool) -> Uuid {
    sqlx::query_scalar("SELECT id FROM reservations")
        .fetch_one(pool)
        .await
        .expect("reservation id should be readable")
}

async fn unavailable_nights(pool: &PgPool, listing_id: Uuid) -> Vec<NaiveDate> {
    sqlx::query(
        r#"
        SELECT night
        FROM listing_unavailable_nights
        WHERE listing_id = $1
        ORDER BY night
        "#,
    )
    .bind(listing_id)
    .fetch_all(pool)
    .await
    .expect("unavailable nights should be readable")
    .into_iter()
    .map(|row| row.get("night"))
    .collect()
}

async fn assert_no_reservation_or_unavailable_nights(pool: &PgPool) {
    let reservation_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM reservations")
        .fetch_one(pool)
        .await
        .expect("reservations count should be readable");
    let unavailable_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM listing_unavailable_nights")
            .fetch_one(pool)
            .await
            .expect("unavailable nights count should be readable");

    assert_eq!(reservation_count, 0);
    assert_eq!(unavailable_count, 0);
}

fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("test date should be valid")
}

fn uuid(value: u128) -> Uuid {
    Uuid::from_u128(value)
}
