-- Add migration script here


CREATE TABLE urls (
    id SERIAL PRIMARY KEY,
    url_key TEXT NOT NULL UNIQUE,
    full_url TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
