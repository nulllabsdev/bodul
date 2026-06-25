use shared::SitemapConfig;
use shared::link::{LinkKind, shopify_from_location};

pub fn sitemap_config() -> SitemapConfig {
    SitemapConfig {
        sitemap_url: vec!["https://minisforum.kr/sitemap.xml".to_string()],
    }
}

/// MinisForum runs Shopify; classification uses the shared Shopify rule.
pub fn from_location(url: &str) -> LinkKind {
    shopify_from_location(url)
}
