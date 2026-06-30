//! Retailer sourcing.
//!
//! Initiates product sourcing for active retailers, triggering the sitemap
//! pipeline (roadmap Stage A). Triggered manually in Phase 0.

use crate::retailer_sourcing::retailers::{
    minisforum_au, minisforum_ca, minisforum_eu, minisforum_fr, minisforum_hk, minisforum_jp, minisforum_kr,
    minisforum_ru, minisforum_uk, minisforum_us,
};
use shared::SitemapConfig;
use shared::link::LinkKind;
use shared::retailer::RetailerCode;

pub mod model;
pub mod retailers;

/// Resolves a retailer's sitemap configuration.
///
/// Retailers without known sitemap URLs return a config with an empty URL list.
pub fn sitemap_config(code: &RetailerCode) -> Option<SitemapConfig> {
    let config = match code {
        RetailerCode::MinisForumEu => minisforum_eu::sitemap_config(),
        RetailerCode::MinisForumUs => minisforum_us::sitemap_config(),
        RetailerCode::MinisForumUk => minisforum_uk::sitemap_config(),
        RetailerCode::MinisForumFr => minisforum_fr::sitemap_config(),
        RetailerCode::MinisForumCa => minisforum_ca::sitemap_config(),
        RetailerCode::MinisForumAu => minisforum_au::sitemap_config(),
        RetailerCode::MinisForumKr => minisforum_kr::sitemap_config(),
        RetailerCode::MinisForumJp => minisforum_jp::sitemap_config(),
        RetailerCode::MinisForumRu => minisforum_ru::sitemap_config(),
        RetailerCode::MinisForumHk => minisforum_hk::sitemap_config(),
    };

    if config.sitemap_url.is_empty() {
        return None;
    }

    Some(config)
}

/// Classifies a page URL using `code`'s retailer-specific rules.
///
/// Each retailer's rule lives alongside its `sitemap_config` (a `from_location`
/// fn in its module). Retailers without a rule yet fall through to
/// [`LinkKind::Unknown`].
pub fn classify_link(code: RetailerCode, url: &str, _source: &str, _image_count: usize) -> LinkKind {
    match code {
        RetailerCode::MinisForumEu => minisforum_eu::from_location(url),
        RetailerCode::MinisForumUs => minisforum_us::from_location(url),
        RetailerCode::MinisForumUk => minisforum_uk::from_location(url),
        RetailerCode::MinisForumFr => minisforum_fr::from_location(url),
        RetailerCode::MinisForumCa => minisforum_ca::from_location(url),
        RetailerCode::MinisForumAu => minisforum_au::from_location(url),
        RetailerCode::MinisForumKr => minisforum_kr::from_location(url),
        RetailerCode::MinisForumJp => minisforum_jp::from_location(url),
        RetailerCode::MinisForumRu => minisforum_ru::from_location(url),
        RetailerCode::MinisForumHk => minisforum_hk::from_location(url),
    }
}
