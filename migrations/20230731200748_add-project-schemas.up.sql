-- Add up migration script here

CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    normalized_name TEXT UNIQUE NOT NULL,
    created DATE DEFAULT CURRENT_DATE,
    has_docs BOOL DEFAULT False
);

CREATE INDEX idx_project_name ON projects (normalized_name);

