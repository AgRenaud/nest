-- Add down migration script here

DROP TABLE releases CASCADE;
DROP TABLE release_files CASCADE;
DROP TABLE release_descriptions CASCADE;
DROP TABLE release_dependencies CASCADE;

DROP TYPE packagetype;
DROP TYPE dependency_kind;

DROP TABLE projects CASCADE;

DROP FUNCTION IF EXISTS normalize_pep426_name;

DROP FUNCTION IF EXISTS normalize_pep426_name;
