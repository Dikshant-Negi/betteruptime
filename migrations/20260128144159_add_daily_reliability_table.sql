-- Add migration script here
CREATE TABLE daily_reliability (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    website_id TEXT NOT NULL REFERENCES websites(id) ON DELETE CASCADE,
    date DATE NOT NULL DEFAULT CURRENT_DATE,
    up_seconds BIGINT DEFAULT 0,
    down_seconds BIGINT DEFAULT 0,
    UNIQUE(website_id, date)
);