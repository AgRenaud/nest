-- Add up migration script here

CREATE TABLE users (
    username CITEXT NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    password VARCHAR(128) NOT NULL,

    CONSTRAINT users_len_username CHECK (length(username) <= 50),
    CONSTRAINT users_valid_username CHECK (username ~* '^([A-Z0-9]|[A-Z0-9][A-Z0-9._-]*[A-Z0-9])$')
);
