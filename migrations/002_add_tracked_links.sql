-- Add tracked_links table for click tracking
CREATE TABLE tracked_links (
    id SERIAL PRIMARY KEY,
    sent_email_id UUID REFERENCES sent_emails(id) ON DELETE CASCADE,
    link_index INT NOT NULL,
    original_url TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_tracked_links_sent_email ON tracked_links(sent_email_id);
CREATE INDEX idx_tracked_links_lookup ON tracked_links(sent_email_id, link_index);
