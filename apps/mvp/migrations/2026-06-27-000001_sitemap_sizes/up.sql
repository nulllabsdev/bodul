ALTER TABLE raw_sitemap_documents ADD COLUMN body_size INT NOT NULL DEFAULT 0;
ALTER TABLE processed_sitemaps ADD COLUMN document_size INT NOT NULL DEFAULT 0;
ALTER TABLE grouped_sitemap_contents ADD COLUMN content_size INT NOT NULL DEFAULT 0;
