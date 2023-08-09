-- Add up migration script here

CREATE FUNCTION pep440_is_prerelease(text) 
    RETURNS boolean as 
    $$
        SELECT lower($1) ~* '(a|b|rc|dev|alpha|beta|c|pre|preview)'
    $$
    LANGUAGE SQL
    IMMUTABLE
    RETURNS NULL ON NULL INPUT;
