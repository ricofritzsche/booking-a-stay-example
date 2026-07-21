-- Add migration script here
-- =====================================================
-- Migration: seed_demo_data
-- Description: Deterministic demo data for local API usage
-- =====================================================

-- =====================================================
-- GUESTS
-- =====================================================
INSERT INTO guests (
    id,
    email,
    full_name,
    booking_eligibility
)
VALUES
    ('20000000-0000-0000-0000-000000000001', 'manuel.horse@example.invalid', 'Manuel Horse', 'eligible'),
    ('20000000-0000-0000-0000-000000000002', 'clara.biscuit@example.invalid', 'Clara Biscuit', 'eligible'),
    ('20000000-0000-0000-0000-000000000003', 'olivia.moonbeam@example.invalid', 'Olivia Moonbeam', 'eligible'),
    ('20000000-0000-0000-0000-000000000004', 'hugo.marmalade@example.invalid', 'Hugo Marmalade', 'eligible'),
    ('20000000-0000-0000-0000-000000000005', 'greta.waffle@example.invalid', 'Greta Waffle', 'eligible'),
    ('20000000-0000-0000-0000-000000000006', 'felix.sparrow@example.invalid', 'Felix Sparrow', 'eligible'),
    ('20000000-0000-0000-0000-000000000007', 'nora.pickle@example.invalid', 'Nora Pickle', 'eligible'),
    ('20000000-0000-0000-0000-000000000008', 'bruno.button@example.invalid', 'Bruno Button', 'eligible'),
    ('20000000-0000-0000-0000-000000000009', 'lydia.pancake@example.invalid', 'Lydia Pancake', 'blocked'),
    ('20000000-0000-0000-0000-000000000010', 'theo.thunder@example.invalid', 'Theo Thunder', 'blocked')
ON CONFLICT DO NOTHING;

-- =====================================================
-- LISTINGS
-- =====================================================
INSERT INTO listings (
    id,
    host_id,
    title,
    max_guests,
    min_nights,
    max_nights,
    booking_status
)
VALUES
    ('30000000-0000-0000-0000-000000000001', '10000000-0000-0000-0000-000000000001', 'Seaside Apartment with Morning Balcony', 4, 2, 21, 'bookable'),
    ('30000000-0000-0000-0000-000000000002', '10000000-0000-0000-0000-000000000001', 'Quiet Garden Studio', 2, 1, 14, 'bookable'),
    ('30000000-0000-0000-0000-000000000003', '10000000-0000-0000-0000-000000000001', 'Old Town Loft with Rooftop Views', 3, 2, 14, 'bookable'),
    ('30000000-0000-0000-0000-000000000004', '10000000-0000-0000-0000-000000000001', 'Bright Harbor Apartment', 4, 2, 30, 'bookable'),
    ('30000000-0000-0000-0000-000000000005', '10000000-0000-0000-0000-000000000001', 'Mountain Cabin by the Pines', 5, 3, 21, 'bookable'),

    ('30000000-0000-0000-0000-000000000006', '10000000-0000-0000-0000-000000000002', 'Riverside Guest Suite', 2, 1, 10, 'bookable'),
    ('30000000-0000-0000-0000-000000000007', '10000000-0000-0000-0000-000000000002', 'Modern City Studio', 2, 1, 14, 'bookable'),
    ('30000000-0000-0000-0000-000000000008', '10000000-0000-0000-0000-000000000002', 'Cozy Courtyard Flat', 3, 2, 14, 'bookable'),
    ('30000000-0000-0000-0000-000000000009', '10000000-0000-0000-0000-000000000002', 'Sunny Beach House', 6, 3, 30, 'bookable'),
    ('30000000-0000-0000-0000-000000000010', '10000000-0000-0000-0000-000000000002', 'Minimalist Downtown Apartment', 2, 2, 14, 'bookable'),

    ('30000000-0000-0000-0000-000000000011', '10000000-0000-0000-0000-000000000003', 'Stone Cottage Near the Village Square', 4, 2, 21, 'bookable'),
    ('30000000-0000-0000-0000-000000000012', '10000000-0000-0000-0000-000000000003', 'Family Home with Private Patio', 6, 3, 30, 'bookable'),
    ('30000000-0000-0000-0000-000000000013', '10000000-0000-0000-0000-000000000003', 'Hillside Retreat with Terrace', 4, 2, 21, 'bookable'),
    ('30000000-0000-0000-0000-000000000014', '10000000-0000-0000-0000-000000000003', 'Calm Studio Near the Market', 2, 1, 10, 'bookable'),
    ('30000000-0000-0000-0000-000000000015', '10000000-0000-0000-0000-000000000003', 'Spacious Apartment by the Park', 5, 2, 21, 'bookable'),

    ('30000000-0000-0000-0000-000000000016', '10000000-0000-0000-0000-000000000004', 'Cottage with Fireplace and Garden', 4, 2, 21, 'bookable'),
    ('30000000-0000-0000-0000-000000000017', '10000000-0000-0000-0000-000000000004', 'Light-Filled Loft Near the Station', 3, 1, 14, 'bookable'),
    ('30000000-0000-0000-0000-000000000018', '10000000-0000-0000-0000-000000000004', 'Small House by the Vineyard', 4, 3, 30, 'bookable'),
    ('30000000-0000-0000-0000-000000000019', '10000000-0000-0000-0000-000000000004', 'Elegant Apartment in the Historic Center', 4, 2, 21, 'bookable'),
    ('30000000-0000-0000-0000-000000000020', '10000000-0000-0000-0000-000000000004', 'Quiet Room with Courtyard View', 2, 1, 7, 'bookable'),

    ('30000000-0000-0000-0000-000000000021', '10000000-0000-0000-0000-000000000005', 'Lakeview Cabin with Wooden Deck', 4, 2, 21, 'bookable'),
    ('30000000-0000-0000-0000-000000000022', '10000000-0000-0000-0000-000000000005', 'Urban Flat Near Cafes', 2, 1, 14, 'bookable'),
    ('30000000-0000-0000-0000-000000000023', '10000000-0000-0000-0000-000000000005', 'Traditional House with Sea Breeze', 5, 3, 30, 'bookable'),
    ('30000000-0000-0000-0000-000000000024', '10000000-0000-0000-0000-000000000005', 'Compact Studio for Weekend Stays', 2, 1, 7, 'bookable'),
    ('30000000-0000-0000-0000-000000000025', '10000000-0000-0000-0000-000000000005', 'Bright Apartment Close to the Marina', 4, 2, 21, 'bookable'),

    ('30000000-0000-0000-0000-000000000026', '10000000-0000-0000-0000-000000000001', 'Peaceful Cottage on the Hill', 4, 2, 21, 'bookable'),
    ('30000000-0000-0000-0000-000000000027', '10000000-0000-0000-0000-000000000001', 'Stylish Loft with Work Corner', 2, 1, 14, 'bookable'),
    ('30000000-0000-0000-0000-000000000028', '10000000-0000-0000-0000-000000000001', 'Garden House Near Walking Trails', 5, 3, 30, 'bookable'),
    ('30000000-0000-0000-0000-000000000029', '10000000-0000-0000-0000-000000000001', 'Central Apartment with Balcony', 3, 2, 14, 'bookable'),
    ('30000000-0000-0000-0000-000000000030', '10000000-0000-0000-0000-000000000001', 'Coastal Studio Near the Promenade', 2, 1, 10, 'bookable'),

    ('30000000-0000-0000-0000-000000000031', '10000000-0000-0000-0000-000000000002', 'Warm Cabin for Slow Weekends', 4, 2, 14, 'bookable'),
    ('30000000-0000-0000-0000-000000000032', '10000000-0000-0000-0000-000000000002', 'Stone House with Shaded Terrace', 6, 3, 30, 'bookable'),
    ('30000000-0000-0000-0000-000000000033', '10000000-0000-0000-0000-000000000002', 'Modern Apartment Near the Museum', 3, 2, 14, 'bookable'),
    ('30000000-0000-0000-0000-000000000034', '10000000-0000-0000-0000-000000000002', 'Private Suite in a Quiet Street', 2, 1, 10, 'bookable'),
    ('30000000-0000-0000-0000-000000000035', '10000000-0000-0000-0000-000000000002', 'Beachside Flat with Sunset View', 4, 2, 21, 'bookable'),

    ('30000000-0000-0000-0000-000000000036', '10000000-0000-0000-0000-000000000003', 'Country Home with Wide Garden', 6, 3, 30, 'bookable'),
    ('30000000-0000-0000-0000-000000000037', '10000000-0000-0000-0000-000000000003', 'Loft Apartment Above a Small Square', 3, 2, 14, 'bookable'),
    ('30000000-0000-0000-0000-000000000038', '10000000-0000-0000-0000-000000000003', 'Simple Studio Near Public Transport', 2, 1, 10, 'bookable'),
    ('30000000-0000-0000-0000-000000000039', '10000000-0000-0000-0000-000000000003', 'Spacious Townhouse for Families', 6, 3, 30, 'bookable'),
    ('30000000-0000-0000-0000-000000000040', '10000000-0000-0000-0000-000000000003', 'Light Apartment with Courtyard Access', 4, 2, 21, 'bookable'),

    ('30000000-0000-0000-0000-000000000041', '10000000-0000-0000-0000-000000000004', 'Wooden Cabin Near the Lake', 4, 2, 21, 'bookable'),
    ('30000000-0000-0000-0000-000000000042', '10000000-0000-0000-0000-000000000004', 'Calm Retreat Outside the City', 5, 3, 30, 'bookable'),
    ('30000000-0000-0000-0000-000000000043', '10000000-0000-0000-0000-000000000004', 'Rooftop Apartment with Evening Light', 3, 2, 14, 'bookable'),
    ('30000000-0000-0000-0000-000000000044', '10000000-0000-0000-0000-000000000004', 'Historic House Near the Cathedral', 4, 2, 21, 'bookable'),
    ('30000000-0000-0000-0000-000000000045', '10000000-0000-0000-0000-000000000004', 'Studio with Garden Entrance', 2, 1, 10, 'bookable'),

    ('30000000-0000-0000-0000-000000000046', '10000000-0000-0000-0000-000000000005', 'Apartment Near the Food Market', 3, 2, 14, 'disabled'),
    ('30000000-0000-0000-0000-000000000047', '10000000-0000-0000-0000-000000000005', 'Small Cottage Behind the Orchard', 4, 2, 21, 'disabled'),
    ('30000000-0000-0000-0000-000000000048', '10000000-0000-0000-0000-000000000005', 'City Loft with High Ceilings', 3, 2, 14, 'disabled'),
    ('30000000-0000-0000-0000-000000000049', '10000000-0000-0000-0000-000000000005', 'Secluded House by the Forest', 5, 3, 30, 'disabled'),
    ('30000000-0000-0000-0000-000000000050', '10000000-0000-0000-0000-000000000005', 'Bright Home Near the Old Harbor', 4, 2, 21, 'disabled')
ON CONFLICT DO NOTHING;
