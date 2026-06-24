use crate::SitemapConfig;

pub fn sitemap_config() -> SitemapConfig {
    SitemapConfig {
        sitemap_url: vec!["https://minisforum.ru/sitemap.xml".to_string()],
    }
}
