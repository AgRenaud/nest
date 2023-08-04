-- Add down migration script here

DROP TABLE releases CASCADE;

DROP TABLE release_files CASCADE;

DROP TABLE release_descriptions CASCADE;

DROP TYPE packagetype;