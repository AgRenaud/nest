-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TYPE user_roles AS ENUM ('admin','contributor');


CREATE TABLE users (
    id uuid PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role user_roles NOT NULL DEFAULT 'contributor',

    CONSTRAINT users_len_username CHECK (length(username) <= 50),
    CONSTRAINT users_valid_username CHECK (username ~* '^([A-Z0-9]|[A-Z0-9][A-Z0-9._-]*[A-Z0-9])$')
);
