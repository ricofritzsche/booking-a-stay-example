-- =====================================================
-- Migration: add_reservation_booking_terms
-- Description: Record listing booking terms used when a reservation is confirmed
-- =====================================================

ALTER TABLE reservations
    ADD COLUMN max_guests_at_confirmation INTEGER,
    ADD COLUMN min_nights_at_confirmation INTEGER,
    ADD COLUMN max_nights_at_confirmation INTEGER;

UPDATE reservations r
SET
    max_guests_at_confirmation = l.max_guests,
    min_nights_at_confirmation = l.min_nights,
    max_nights_at_confirmation = l.max_nights
FROM listings l
WHERE r.listing_id = l.id;

ALTER TABLE reservations
    ALTER COLUMN max_guests_at_confirmation SET NOT NULL,
    ALTER COLUMN min_nights_at_confirmation SET NOT NULL;

ALTER TABLE reservations
    ADD CONSTRAINT reservations_booking_terms_at_confirmation_valid
    CHECK (
        max_guests_at_confirmation > 0
        AND min_nights_at_confirmation > 0
        AND (
            max_nights_at_confirmation IS NULL
            OR max_nights_at_confirmation >= min_nights_at_confirmation
        )
    );
