-- Add migration script here
-- =========================
-- Qira'a Core Schema
-- =========================

SET client_encoding = 'UTF8';

-- ---------
-- Genres
-- ---------
CREATE TABLE genres (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL
);

-- ---------
-- Books
-- ---------
CREATE TABLE books (
    id TEXT PRIMARY KEY,
    genre_id TEXT NOT NULL REFERENCES genres(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    info TEXT,
    description TEXT,
    historical_context TEXT,
    author_bio TEXT,
    image_url TEXT,
    created_at TIMESTAMPTZ DEFAULT now()
);

CREATE INDEX idx_books_genre_id ON books(genre_id);

-- ---------
-- Chapters
-- ---------
CREATE TABLE chapters (
    id TEXT PRIMARY KEY,
    book_id TEXT NOT NULL REFERENCES books(id) ON DELETE CASCADE,
    order_no INT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    UNIQUE (book_id, order_no)
);

CREATE INDEX idx_chapters_book_id ON chapters(book_id);
CREATE INDEX idx_chapters_order_no ON chapters(order_no);
