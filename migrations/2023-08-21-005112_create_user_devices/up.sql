-- Your SQL goes here
ALTER TABLE user_devices ADD COLUMN access_token TEXT NOT NULL UNIQUE default '';
