//! SQLx migration runner for the relational Application State.

use sqlx::{Pool, Postgres};
use tracing::info;

/// Applies pending SQLx migrations to the configured database.
pub(crate) async fn run_migrations(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    info!("starting database migrations");
    sqlx::migrate!("./migrations").run(pool).await?;
    info!("database migrations complete; database is up to date");
    Ok(())
}
