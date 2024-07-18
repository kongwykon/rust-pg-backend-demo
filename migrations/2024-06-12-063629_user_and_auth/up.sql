-- Your SQL goes here
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    pw_hash BYTEA NOT NULL,
    username varchar(255) UNIQUE NOT NULL
);