-- Add up migration script here

CREATE TABLE projects (
    id serial PRIMARY KEY, 
    name TEXT UNIQUE NOT NULL,
    normalized_name TEXT UNIQUE NOT NULL,
    created DATE DEFAULT CURRENT_DATE,
    has_docs BOOL DEFAULT False
);
