use crate::SitemapConfig;

pub fn sitemap_config() -> SitemapConfig {
    SitemapConfig {
        sitemap_url: vec!["https://store.minisforum.com/sitemap.xml".to_string()],
    }
}
