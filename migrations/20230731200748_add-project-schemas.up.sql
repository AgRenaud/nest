-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS citext;


CREATE OR REPLACE FUNCTION normalize_pep426_name(text)
    RETURNS text AS
    $$
        SELECT lower(regexp_replace($1, '(\.|_)', '-', 'ig'))
    $$
    LANGUAGE SQL
    IMMUTABLE
    RETURNS NULL ON NULL INPUT;

CREATE OR REPLACE FUNCTION pep440_is_prerelease(text) 
    RETURNS boolean as 
    $$
        SELECT lower($1) ~* '(a|b|rc|dev|alpha|beta|c|pre|preview)'
    $$
    LANGUAGE SQL
    IMMUTABLE
    RETURNS NULL ON NULL INPUT;


CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    normalized_name TEXT UNIQUE NOT NULL,
    created DATE DEFAULT CURRENT_DATE,
    has_docs BOOL NOT NULL DEFAULT False
);

CREATE INDEX idx_project_name ON projects (normalized_name);


CREATE TYPE packagetype AS ENUM ('bdist_dmg','bdist_dumb','bdist_egg','bdist_msi','bdist_rpm','bdist_wheel','bdist_wininst','sdist');
CREATE TYPE dependency_kind AS ENUM ('requires','provides','obsoletes','requires_dist','provides_dist','obsoletes_dist','requires_external');

CREATE TABLE releases (
    id SERIAL PRIMARY KEY,
    version TEXT NOT NULL,
    canonical_version TEXT NOT NULL,
    is_prerelease BOOL NOT NULL DEFAULT FALSE,
    author TEXT,
    author_email TEXT,
    maintainer TEXT,
    maintainer_email TEXT,
    home_page TEXT,
    license TEXT,
    summary TEXT,
    keywords TEXT,
    platform TEXT,
    download_url TEXT,
    requires_python TEXT,

    project_id INT,

    CONSTRAINT fk_project
      FOREIGN KEY(project_id)
	    REFERENCES projects(id)
);

CREATE TABLE release_files (
    id SERIAL PRIMARY KEY,
    python_version TEXT,
    requires_python TEXT,
    packagetype packagetype,
    filename TEXT UNIQUE NOT NULL,
    path TEXT UNIQUE NOT NULL,
    size INT,
    md5_digest TEXT UNIQUE NOT NULL,
    sha256_digest CITEXT UNIQUE NOT NULL CHECK (sha256_digest ~* '^[A-F0-9]{64}$'),
    blake2_256_digest CITEXT UNIQUE NOT NULL CHECK (blake2_256_digest ~* '^[A-F0-9]{64}$'),
    upload_time DATE DEFAULT CURRENT_DATE,
    -- TODO: Read whl content and extract metadata to compute the two following fields
    -- metadata_file_sha256_digest CITEXT NOT NULL,
    -- metadata_file_blake2_256_digest CITEXT NOT NULL,

    release_id INT,

    CONSTRAINT fk_release_files
      FOREIGN KEY(release_id)
	    REFERENCES releases(id)
);

CREATE TABLE release_descriptions (
    id SERIAL PRIMARY KEY,
    content_type TEXT,
    raw TEXT NOT NULL,
    html TEXT NOT NULL,

    release_id INT,

    CONSTRAINT fk_release_descriptions
      FOREIGN KEY(release_id)
	    REFERENCES releases(id)
);

CREATE TABLE release_dependencies (
  id SERIAL PRIMARY KEY,
  kind dependency_kind,
  specifier TEXT,
  release_id INT,

  CONSTRAINT fk_release_dependencies
      FOREIGN KEY(release_id)
	    REFERENCES releases(id)
);
