use super::shopify_from_location;
use shared::SitemapConfig;
use shared::link::LinkKind;

pub fn sitemap_config() -> SitemapConfig {
    SitemapConfig {
        sitemap_url: vec!["https://www.ugreen.com/sitemap.xml".to_string()],
    }
}

pub fn from_location(url: &str) -> LinkKind {
    let path = url.to_lowercase();

    if path.starts_with("https://www.ugreen.com/products/") {
        LinkKind::Product
    } else if path.contains("https://www.ugreen.com/collections/") {
        LinkKind::Catalog
    } else if path.contains("https://www.ugreen.com/pages/")
        || path.contains("https://www.ugreen.com/blogs/")
        || path.contains("https://www.ugreen.com/blog/")
    {
        LinkKind::Content
    } else {
        let v = vec![
            "/ar-ae/", "/ar-sa/", "/de-de/", "/en-ae/", "/en-au/", "/en-ca/", "/en-de/", "/en-es/", "/en-eu/",
            "/en-fr/", "/en-gb/", "/en-id/", "/en-it/", "/en-my/", "/en-nl/", "/en-ph/", "/en-sa/", "/en-sg/",
            "/en-th/", "/en-tr/", "/en-vn/", "/es-es/", "/fr-fr/", "/id-id/", "/it-it/", "/nl-nl/", "/th-th/",
            "/tr-tr/", "/vi-vn/", "/nl/", "/tr/", "/vi/",
        ];

        if v.iter().any(|p| path.contains(p)) {
            return LinkKind::NotInterested;
        }

        LinkKind::Unknown
    }
}
