use super::anker_from_location;
use shared::SitemapConfig;
use shared::link::LinkKind;

pub fn sitemap_config() -> SitemapConfig {
    SitemapConfig {
        sitemap_url: vec!["https://www.anker.com/sitemap.xml".to_string()],
    }
}
pub fn from_location(url: &str, source: &str) -> LinkKind {
    let matced_by_source = match source {
        "https://www.anker.com/sitemap-0.xml" => Some(LinkKind::NotInterested),
        "https://www.anker.com/server-sitemap-index-pages.xml" => Some(LinkKind::NotInterested),
        "https://www.anker.com/server-sitemap-index-products.xml" => Some(LinkKind::Product),
        "https://www.anker.com/server-sitemap-index-collections.xml" => Some(LinkKind::Catalog),
        "https://www.anker.com/server-sitemap-index-blog.xml" => Some(LinkKind::Content),
        _ => None,
    };

    let path = url.to_lowercase();

    if let Some(y) = matced_by_source {
        if !path.starts_with("https://www.anker.com/eu-de/") {
            return LinkKind::NotInterested;
        }

        return y;
    }

    anker_from_location(&path)
}
