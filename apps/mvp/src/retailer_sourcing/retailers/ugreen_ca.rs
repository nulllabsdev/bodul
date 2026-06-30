use super::shopify_from_location;
use shared::SitemapConfig;
use shared::link::LinkKind;

pub fn sitemap_config() -> SitemapConfig {
    SitemapConfig {
        sitemap_url: vec!["https://ca.ugreen.com/sitemap.xml".to_string()],
    }
}

pub fn from_location(url: &str) -> LinkKind {
    shopify_from_location(url)
}
