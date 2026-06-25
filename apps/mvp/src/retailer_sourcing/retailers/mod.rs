use shared::link::LinkKind;

pub mod minisforum_au;
pub mod minisforum_ca;
pub mod minisforum_eu;
pub mod minisforum_fr;
pub mod minisforum_hk;
pub mod minisforum_jp;
pub mod minisforum_kr;
pub mod minisforum_ru;
pub mod minisforum_uk;
pub mod minisforum_us;

// ============================== Separator ==============================

/// Classifies a page URL by Shopify path conventions: `/products/` is a product,
/// `/collections/` a catalog, `/pages/` and `/blogs/` are content, and anything
/// else is unknown. Case-insensitive. Reused by every Shopify-based retailer.
pub fn shopify_from_location(url: &str) -> LinkKind {
    let path = url.to_lowercase();
    if path.contains("/products/") {
        LinkKind::Product
    } else if path.contains("/collections/") {
        LinkKind::Catalog
    } else if path.contains("/pages/") || path.contains("/blogs/") {
        LinkKind::Content
    } else {
        LinkKind::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_shopify_link_paths() {
        let cases = [
            ("https://minisforumpc.eu/products/um890", LinkKind::Product),
            (
                "https://minisforumpc.eu/de/products/ms01",
                LinkKind::Product,
            ),
            ("https://minisforumpc.eu/collections/all", LinkKind::Catalog),
            ("https://minisforumpc.eu/pages/about", LinkKind::Content),
            (
                "https://minisforumpc.eu/blogs/news/a-post",
                LinkKind::Content,
            ),
            ("https://minisforumpc.eu/", LinkKind::Unknown),
            ("https://minisforumpc.eu/agents.md", LinkKind::Unknown),
        ];
        for (url, expected) in cases {
            assert_eq!(shopify_from_location(url), expected, "for {url}");
        }
    }

    #[test]
    fn classification_is_case_insensitive() {
        assert_eq!(
            shopify_from_location("https://minisforumpc.eu/Products/UM890"),
            LinkKind::Product
        );
    }
}
