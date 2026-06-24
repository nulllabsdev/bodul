use crate::SitemapConfig;

pub fn sitemap_config() -> SitemapConfig {
    SitemapConfig {
        sitemap_url: vec!["https://ca.minisforum.com/sitemap.xml".to_string()],
    }
}
