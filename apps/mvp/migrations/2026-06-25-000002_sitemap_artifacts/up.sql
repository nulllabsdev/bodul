CREATE TABLE sitemap_retrievals (
    id UUID PRIMARY KEY,
    retailer_code TEXT NOT NULL,
    status TEXT NOT NULL,
    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retrieved_at TIMESTAMPTZ,
    processed_at TIMESTAMPTZ,
    grouped_at TIMESTAMPTZ,
    error TEXT
);
CREATE INDEX idx_sitemap_retrievals_retailer_requested_at
    ON sitemap_retrievals (retailer_code, requested_at);
CREATE INDEX idx_sitemap_retrievals_status
    ON sitemap_retrievals (status);

CREATE TABLE raw_sitemap_documents (
    id UUID PRIMARY KEY,
    retrieval_id UUID NOT NULL REFERENCES sitemap_retrievals(id) ON DELETE CASCADE,
    url TEXT NOT NULL,
    last_modified TIMESTAMPTZ,
    body TEXT NOT NULL,
    fetched_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (retrieval_id, url)
);
CREATE INDEX idx_raw_sitemap_documents_retrieval_id
    ON raw_sitemap_documents (retrieval_id);

CREATE TABLE processed_sitemaps (
    id UUID PRIMARY KEY,
    retrieval_id UUID NOT NULL REFERENCES sitemap_retrievals(id) ON DELETE CASCADE,
    retailer_code TEXT NOT NULL,
    document JSONB NOT NULL,
    url_count INT NOT NULL,
    processed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (retrieval_id)
);
CREATE INDEX idx_processed_sitemaps_retrieval_id
    ON processed_sitemaps (retrieval_id);

CREATE TABLE grouped_sitemap_contents (
    id UUID PRIMARY KEY,
    processed_sitemap_id UUID NOT NULL REFERENCES processed_sitemaps(id) ON DELETE CASCADE,
    retrieval_id UUID NOT NULL REFERENCES sitemap_retrievals(id) ON DELETE CASCADE,
    retailer_code TEXT NOT NULL,
    content JSONB NOT NULL,
    product_count INT NOT NULL,
    catalog_count INT NOT NULL,
    content_count INT NOT NULL,
    not_interested_count INT NOT NULL,
    unknown_count INT NOT NULL,
    grouped_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (processed_sitemap_id)
);
CREATE INDEX idx_grouped_sitemap_contents_retrieval_id
    ON grouped_sitemap_contents (retrieval_id);
