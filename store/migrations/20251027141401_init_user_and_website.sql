-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'website_status') THEN
        CREATE TYPE website_status AS ENUM ('UP', 'DOWN', 'UNKNOWN');
    END IF;
END$$;

CREATE TABLE IF NOT EXISTS  websites(
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id),
    name TEXT NOT NULL,
    url TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    check_interval INTEGER DEFAULT 60,
    status website_status NOT NULL DEFAULT 'UNKNOWN'
);