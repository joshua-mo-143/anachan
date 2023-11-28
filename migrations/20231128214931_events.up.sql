-- Add up migration script here
CREATE TABLE IF NOT EXISTS events (
	id SERIAL PRIMARY KEY,
	uri VARCHAR NOT NULL,
	session_uuid VARCHAR NOT NULL,
	event_id VARCHAR NOT NULL,
	date_time TIMESTAMP WITH TIME ZONE NOT NULL 
);