-- Add migration script here
-- =========================
-- Seed: Core Genres
-- =========================

INSERT INTO genres (id, title) VALUES
    ('religious', 'Religious'),
    ('philosophical', 'Philosophical'),
    ('historical', 'Historical')
ON CONFLICT DO NOTHING;
