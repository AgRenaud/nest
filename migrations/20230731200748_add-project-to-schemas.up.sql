-- Add up migration script here

CREATE TABLE projects (
    id serial PRIMARY KEY, 
    name TEXT UNIQUE NOT NULL,
    normalized_name TEXT UNIQUE NOT NULL,
    created DATE,
    has_docs BOOL
)