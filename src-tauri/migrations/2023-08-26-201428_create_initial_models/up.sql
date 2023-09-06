-- Your SQL goes here
CREATE TABLE podcasts (
    id INTEGER PRIMARY KEY NOT NULL,
    guid TEXT NOT NULL,
    author TEXT NOT NULL,
    local_image_path TEXT NOT NULL,
    image_url TEXT NOT NULL,
    feed_url TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE episodes (
    id INTEGER PRIMARY KEY NOT NULL,
    guid TEXT NOT NULL,
    podcast_id INTEGER NOT NULL REFERENCES podcasts(id),
    content_local_path TEXT NOT NULL,
    content_url TEXT NOT NULL,
    description TEXT NOT NULL,
    image_local_path TEXT NOT NULL,
    image_url TEXT NOT NULL,
    length INTEGER NOT NULL,
    link TEXT NOT NULL,
    episode_date TIMESTAMP NOT NULL,
    title TEXT NOT NULL
);

CREATE TABLE episode_progresses (
    id INTEGER PRIMARY KEY NOT NULL,
    episode_id INTEGER NOT NULL REFERENCES episodes(id),
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    listened_seconds INTEGER NOT NULL,
    updated_at TIMESTAMP NOT NULL
);
