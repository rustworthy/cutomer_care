-- Add up migration script here

CREATE OR REPLACE FUNCTION create_types() RETURNS integer AS $$
	DECLARE type_already_exists INTEGER;
		BEGIN
			SELECT into type_already_exists (SELECT 1 FROM pg_type WHERE typname = 'status');
			IF type_already_exists IS NULL THEN
				CREATE TYPE question_status AS ENUM ('Resolved', 'Unresolved', 'Pending', 'Canceled');
			END IF;
			SELECT into type_already_exists (SELECT 1 FROM pg_type WHERE typname = 'msg_type');
			IF type_already_exists IS NULL THEN
				CREATE TYPE msg_type AS ENUM ('request', 'response', 'other');
			END IF;
			RETURN type_already_exists;
		END;
		$$ LANGUAGE plpgsql;
	SELECT create_types();

CREATE TABLE IF NOT EXISTS questions (
    _id UUID UNIQUE NOT NULL DEFAULT gen_random_uuid(),
    id serial PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    title VARCHAR (255) NOT NULL,
    content TEXT NOT NULL,
    tags TEXT [],
    status QUESTION_STATUS NOT NULL DEFAULT 'Pending'
);