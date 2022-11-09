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

CREATE TABLE IF NOT EXISTS users (
    _id UUID UNIQUE DEFAULT gen_random_uuid(),
    id serial PRIMARY KEY,
    created_at TIMESTAMP DEFAULT NOW(),
	email VARCHAR(64) UNIQUE NOT NULL,
	password TEXT NOT NULL,
	first_name VARCHAR(64) NOT NULL,
    last_name VARCHAR(64) NOT NULL,
	is_staff BOOLEAN DEFAULT FALSE,
	is_superuser BOOLEAN DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS questions (
    _id UUID UNIQUE DEFAULT gen_random_uuid(),
    id serial PRIMARY KEY,
    created_at TIMESTAMP DEFAULT NOW(),
    title VARCHAR (255) NOT NULL,
    content TEXT NOT NULL,
    tags TEXT [],
    status QUESTION_STATUS DEFAULT 'Pending',
    author INTEGER REFERENCES users (id)
);