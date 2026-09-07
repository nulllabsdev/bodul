-- Restore FK from offers.grouped_content_id -> grouped_sitemap_contents(id)
ALTER TABLE offers
    ADD CONSTRAINT offers_grouped_content_id_fkey
    FOREIGN KEY (grouped_content_id) REFERENCES grouped_sitemap_contents(id) ON DELETE CASCADE;

-- Restore FK from raw_offers.offer_id -> offers(id)
ALTER TABLE raw_offers
    ADD CONSTRAINT raw_offers_offer_id_fkey
    FOREIGN KEY (offer_id) REFERENCES offers(id) ON DELETE CASCADE;

-- Restore index on raw_offers.offer_id
CREATE INDEX idx_raw_offers_offer_id ON raw_offers (offer_id);
