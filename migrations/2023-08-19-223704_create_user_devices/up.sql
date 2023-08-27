-- Your SQL goes here
CREATE TABLE user_devices (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    last_session_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    UNIQUE(user_id, name)
)
