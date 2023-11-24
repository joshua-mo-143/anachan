-- Add up migration script here
CREATE TABLE IF NOT EXISTS stats (
	id SERIAL PRIMARY KEY,
	uri VARCHAR NOT NULL,
	domain VARCHAR NOT NULL,
	session_uuid VARCHAR NOT NULL,
	date_time TIMESTAMP WITH TIME ZONE NOT NULL,
	duration INT NOT NULL
);
