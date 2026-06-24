use crate::SitemapConfig;

pub fn sitemap_config() -> SitemapConfig {
    SitemapConfig {
        sitemap_url: vec!["https://minisforum.kr/sitemap.xml".to_string()],
    }
}
