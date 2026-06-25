use crate::SitemapConfig;

pub fn sitemap_config() -> SitemapConfig {
    SitemapConfig {
        sitemap_url: vec!["https://minisforumpc.eu/sitemap.xml".to_string()],
    }
}

/// MinisForum runs Shopify; classification uses the shared Shopify rule.
pub fn from_location(url: &str) -> crate::link::LinkKind {
    crate::link::shopify_from_location(url)
}
