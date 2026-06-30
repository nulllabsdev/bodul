use super::anker_from_location;
use shared::SitemapConfig;
use shared::link::LinkKind;

pub fn sitemap_config() -> SitemapConfig {
    SitemapConfig {
        sitemap_url: vec!["https://www.ankeritaly.com/sitemap.xml".to_string()],
    }
}

pub fn from_location(url: &str, source: &str) -> LinkKind {
    let matced_by_source = match source {
        "https://www.ankeritaly.com/sitemap-0.xml" => Some(LinkKind::NotInterested),
        "https://www.ankeritaly.com/server-sitemap-index-pages.xml" => Some(LinkKind::NotInterested),
        "https://www.ankeritaly.com/server-sitemap-index-products.xml" => Some(LinkKind::Product),
        "https://www.ankeritaly.com/server-sitemap-index-collections.xml" => Some(LinkKind::Catalog),
        "https://www.ankeritaly.com/server-sitemap-index-blog.xml" => Some(LinkKind::Content),
        _ => None,
    };

    if let Some(y) = matced_by_source {
        return y;
    }

    anker_from_location(&url)
}
