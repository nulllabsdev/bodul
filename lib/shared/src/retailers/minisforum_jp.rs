use crate::SitemapConfig;

pub fn sitemap_config() -> SitemapConfig {
    SitemapConfig {
        sitemap_url: vec!["https://www.minisforum.jp/sitemap.xml".to_string()],
    }
}
