//! Insert records for the `offers` and `raw_offers` tables.
//!
//! An offer is a product page discovered from grouped sitemap content; a raw offer
//! holds the downloaded page body for one offer. `body_size` is the body length in
//! bytes, stored alongside the body so size reports need not read it.

use crate::schema::{offers, raw_offers};
use chrono::{DateTime, Utc};
use diesel::Insertable;
use uuid::Uuid;

#[derive(Insertable)]
#[diesel(table_name = offers)]
pub struct NewOfferRecord {
    pub id: Uuid,
    pub grouped_content_id: Uuid,
    pub retailer_code: String,
    pub url: String,
    pub status: &'static str,
    pub notes: Option<String>,
    pub discovered_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = raw_offers)]
pub struct NewRawOfferRecord {
    pub id: Uuid,
    pub offer_id: Uuid,
    pub url: String,
    pub body: String,
    pub body_size: i32,
}
