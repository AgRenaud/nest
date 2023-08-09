-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS citext;

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

    project_id INTEGER,

    CONSTRAINT fk_project
      FOREIGN KEY(project_id) 
	    REFERENCES projects(id)
);

CREATE TABLE release_files (
    id SERIAL PRIMARY KEY,
    python_version TEXT,
    requires_python TEXT,
    packagetype packagetype,
    comment_text TEXT,
    filename TEXT UNIQUE,
    path TEXT UNIQUE NOT NULL,
    size INT,
    md5_digest TEXT UNIQUE NOT NULL,
    sha256_digest CITEXT UNIQUE NOT NULL,
    blake2_256_digest CITEXT UNIQUE NOT NULL,
    upload_time DATE,
    uploaded_via TEXT,
    metadata_file_sha256_digest CITEXT NOT NULL,
    metadata_file_blake2_256_digest CITEXT NOT NULL,

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
    rendered_by TEXT NOT NULL,

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