// Stage A — retailer data (hardcoded in Phase 0) and sourcing trigger
mod retailer_sourcing;

// Stage B/C — sitemap fetching and processing
mod sitemap_discovery;

// Stage D — offer page fetching
mod offer_discovery;

// Stage E — offer page processing
mod offer_processing;

// Stage G — product inventory (write side)
mod product_information_management;

// Stage H — offer inventory (write side)
mod offer_information_management;

// Stage K — customer-facing API and web app
mod customer_facing;

// Supporting module for fetching stuff
mod retailer_data_ingestion;

fn main() {
    println!("Hello, world!");
}
