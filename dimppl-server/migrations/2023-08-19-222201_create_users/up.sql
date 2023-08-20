-- Your SQL goes here
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    access_key TEXT NOT NULL UNIQUE
);
