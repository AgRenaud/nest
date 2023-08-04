-- Add up migration script here

CREATE OR REPLACE FUNCTION normalize_pep426_name(text)
    RETURNS text AS
    $$
        SELECT lower(regexp_replace($1, '(\.|_)', '-', 'ig'))
    $$
    LANGUAGE SQL
    IMMUTABLE
    RETURNS NULL ON NULL INPUT;
