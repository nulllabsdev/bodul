//! MVP retail-sourcing pipeline (Phase 0).
//!
//! Library crate exposing the pipeline stages; the binaries in `src/bin/` drive
//! them.

// Stage A — retailer data (hardcoded in Phase 0) and sourcing trigger
pub mod retailer_sourcing;

// Stage B/C — sitemap fetching and processing
pub mod sitemap_discovery;

// Stage D — offer page fetching
pub mod offer_discovery;

// Stage E — offer page processing
pub mod offer_processing;

// Stage E — retailer-specific HTML parsing (used by offer_processing)
pub mod html_parser;

// Stage G — product inventory (write side)
pub mod product_information_management;

// Stage H — offer inventory (write side)
pub mod offer_information_management;

// Stage K — customer-facing API and web app
pub mod customer_facing;

// Supporting module for database access
pub mod database;

// Supporting module for fetching stuff
pub mod retailer_data_ingestion;
