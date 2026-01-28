CREATE TABLE pings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    website_id TEXT NOT NULL REFERENCES websites(id) ON DELETE CASCADE,
    latency INTEGER, 
    status website_status NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);


CREATE INDEX idx_pings_website_created ON pings(website_id, created_at DESC);