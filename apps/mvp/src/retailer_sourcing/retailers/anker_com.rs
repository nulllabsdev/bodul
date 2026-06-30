use super::anker_from_location;
use shared::SitemapConfig;
use shared::link::LinkKind;

pub fn sitemap_config() -> SitemapConfig {
    SitemapConfig {
        sitemap_url: vec!["https://www.anker.com/sitemap.xml".to_string()],
    }
}

pub fn from_location(url: &str) -> LinkKind {
    let path = url.to_lowercase();

    if path.starts_with("https://www.anker.com/products/") {
        LinkKind::Product
    } else if path.contains("https://www.anker.com/collections/") {
        LinkKind::Catalog
    } else if path.contains("https://www.anker.com/pages/")
        || path.contains("https://www.anker.com/blogs/")
        || path.contains("https://www.anker.com/blog/")
    {
        LinkKind::Content
    } else {
        let v = vec![
            "/ca-fr/", "/eu-de/", "/eu-en/", "/eu-pl/", "/ae/", "/au/", "/ca/", "/fr/", "/my/", "/nz/", "/uk/", "/vn/",
        ];

        if v.iter().any(|p| path.contains(p)) {
            return LinkKind::NotInterested;
        }

        anker_from_location(&url)
    }
}
