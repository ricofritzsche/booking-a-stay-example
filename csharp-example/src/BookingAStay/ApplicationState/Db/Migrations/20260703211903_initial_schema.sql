-- =====================================================
-- Migration: initial_schema
-- Description: Initial schema for the Stay Booking Example
-- =====================================================

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- =====================================================
-- GUESTS
-- =====================================================
CREATE TABLE guests (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email               TEXT NOT NULL,
    full_name           TEXT NOT NULL,
    booking_eligibility TEXT NOT NULL DEFAULT 'eligible'
                        CHECK (booking_eligibility IN ('eligible', 'blocked')),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX guests_email_unique
    ON guests (lower(email));

-- =====================================================
-- LISTINGS
-- =====================================================
CREATE TABLE listings (
    id             UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    host_id        UUID NOT NULL,
    title          TEXT NOT NULL,
    max_guests     INTEGER NOT NULL CHECK (max_guests > 0),
    min_nights     INTEGER NOT NULL DEFAULT 1 CHECK (min_nights > 0),
    max_nights     INTEGER CHECK (max_nights IS NULL OR max_nights >= min_nights),
    booking_status TEXT NOT NULL DEFAULT 'bookable'
                   CHECK (booking_status IN ('bookable', 'disabled')),
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX listings_host_idx
    ON listings (host_id);

-- =====================================================
-- RESERVATIONS
-- =====================================================
CREATE TABLE reservations (
    id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    listing_id   UUID NOT NULL REFERENCES listings(id),
    guest_id     UUID NOT NULL REFERENCES guests(id),
    check_in     DATE NOT NULL,
    check_out    DATE NOT NULL,
    guest_count  INTEGER NOT NULL CHECK (guest_count > 0),
    status       TEXT NOT NULL DEFAULT 'confirmed'
                 CHECK (status IN ('confirmed', 'cancelled')),
    confirmed_at TIMESTAMPTZ NOT NULL,
    cancelled_at TIMESTAMPTZ,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT reservations_valid_dates
        CHECK (check_out > check_in),

    CONSTRAINT reservations_cancelled_at_matches_status
        CHECK (
            (status = 'confirmed' AND cancelled_at IS NULL)
            OR
            (status = 'cancelled' AND cancelled_at IS NOT NULL)
        ),

    CONSTRAINT reservations_id_listing_unique
        UNIQUE (id, listing_id)
);

CREATE INDEX reservations_listing_dates_idx
    ON reservations (listing_id, check_in, check_out);

CREATE INDEX reservations_guest_idx
    ON reservations (guest_id);

-- =====================================================
-- LISTING UNAVAILABLE NIGHTS
--
-- Final database guard for listing-night availability.
--
-- A stay from July 1 to July 4 occupies:
-- July 1, July 2, July 3.
-- The check-out day is not occupied.
--
-- A listing night can be unavailable because of:
-- - a confirmed reservation
-- - a host block
--
-- The primary key prevents two reasons from occupying
-- the same listing night at the same time.
-- =====================================================
CREATE TABLE listing_unavailable_nights (
    listing_id          UUID NOT NULL REFERENCES listings(id),
    night               DATE NOT NULL,
    unavailability_type TEXT NOT NULL
                        CHECK (unavailability_type IN ('reservation', 'host_block')),
    reservation_id      UUID,
    reason              TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY (listing_id, night),

    CONSTRAINT listing_unavailable_nights_reservation_fk
        FOREIGN KEY (reservation_id, listing_id)
        REFERENCES reservations(id, listing_id)
        ON DELETE CASCADE,

    CONSTRAINT listing_unavailable_nights_valid_source
        CHECK (
            (unavailability_type = 'reservation' AND reservation_id IS NOT NULL)
            OR
            (unavailability_type = 'host_block' AND reservation_id IS NULL)
        )
);

CREATE INDEX listing_unavailable_nights_reservation_idx
    ON listing_unavailable_nights (reservation_id);
