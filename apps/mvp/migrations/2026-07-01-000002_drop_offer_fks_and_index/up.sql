-- Drop redundant index on raw_offers.offer_id (already covered by the UNIQUE (offer_id) constraint)
DROP INDEX IF EXISTS idx_raw_offers_offer_id;

-- Drop FK from raw_offers.offer_id -> offers(id)
ALTER TABLE raw_offers DROP CONSTRAINT IF EXISTS raw_offers_offer_id_fkey;

-- Drop FK from offers.grouped_content_id -> grouped_sitemap_contents(id)
ALTER TABLE offers DROP CONSTRAINT IF EXISTS offers_grouped_content_id_fkey;
