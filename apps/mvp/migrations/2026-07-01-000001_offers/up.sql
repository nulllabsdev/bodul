CREATE TABLE offers (
    id UUID PRIMARY KEY,
    grouped_content_id UUID NOT NULL REFERENCES grouped_sitemap_contents(id) ON DELETE CASCADE,
    retailer_code TEXT NOT NULL,
    url TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'discovered',
    notes TEXT,
    discovered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (retailer_code, url)
);

CREATE INDEX idx_offers_grouped_content_id ON offers (grouped_content_id);

CREATE TABLE raw_offers (
    id UUID PRIMARY KEY,
    offer_id UUID NOT NULL REFERENCES offers(id) ON DELETE CASCADE,
    url TEXT NOT NULL,
    body TEXT NOT NULL,
    body_size INT NOT NULL DEFAULT 0,
    fetched_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (offer_id)
);

CREATE INDEX idx_raw_offers_offer_id ON raw_offers (offer_id);
CREATE INDEX idx_raw_offers_url_fetched_at ON raw_offers (url, fetched_at);
