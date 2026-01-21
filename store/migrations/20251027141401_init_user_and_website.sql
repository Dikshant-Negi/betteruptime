CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
 

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'website_status') THEN
        CREATE TYPE website_status AS ENUM ('UP', 'DOWN', 'UNKNOWN');
    END IF;
END$$;


CREATE TABLE IF NOT EXISTS websites (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    url TEXT NOT NULL,
    check_interval INTEGER DEFAULT 60,
    status website_status NOT NULL DEFAULT 'UNKNOWN',
    last_checked_at TIMESTAMP WITH TIME ZONE, 
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
