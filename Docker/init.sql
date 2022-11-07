CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE OR REPLACE FUNCTION uuid_or_null(str text) RETURNS uuid AS $$
    BEGIN
        RETURN str::uuid;
    EXCEPTION WHEN invalid_text_representation THEN
        RETURN NULL;
    END;
    $$ LANGUAGE plpgsql;