CREATE TABLE IF NOT EXISTS website_stats (
    website_id TEXT PRIMARY KEY REFERENCES websites(id) ON DELETE CASCADE,
    total_uptime_seconds BIGINT DEFAULT 0,
    total_downtime_seconds BIGINT DEFAULT 0,
    last_status_change TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    current_status website_status DEFAULT 'UNKNOWN'
);

CREATE TABLE IF NOT EXISTS incidents (
    id TEXT PRIMARY KEY, 
    website_id TEXT NOT NULL REFERENCES websites(id) ON DELETE CASCADE,
    start_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    end_time TIMESTAMP WITH TIME ZONE, 
    error_reason TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE OR REPLACE FUNCTION create_website_stats()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO website_stats (website_id)
    VALUES (NEW.id);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_create_website_stats ON websites;
CREATE TRIGGER trigger_create_website_stats
AFTER INSERT ON websites
FOR EACH ROW
EXECUTE FUNCTION create_website_stats();