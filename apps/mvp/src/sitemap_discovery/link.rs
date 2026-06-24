//! Link (page-URL) classification used by the `detect` command.
//!
//! Distinct from [`super::sitemap::SitemapKind`], which classifies sitemap
//! *files*; this classifies the individual page links found inside them.

/// The type of a page link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LinkKind {
    Product,
    Catalog,
    Content,
    Unknown,
}

impl LinkKind {
    /// Lowercase label.
    pub const fn as_str(self) -> &'static str {
        match self {
            LinkKind::Product => "product",
            LinkKind::Catalog => "catalog",
            LinkKind::Content => "content",
            LinkKind::Unknown => "unknown",
        }
    }

    /// Classifies a page URL by its path (Shopify conventions): `/products/` is a
    /// product, `/collections/` a catalog, `/pages/` and `/blogs/` are content,
    /// and anything else is unknown. Case-insensitive.
    pub fn from_location(location: &str) -> Self {
        let path = location.to_lowercase();
        if path.contains("/products/") {
            Self::Product
        } else if path.contains("/collections/") {
            Self::Catalog
        } else if path.contains("/pages/") || path.contains("/blogs/") {
            Self::Content
        } else {
            Self::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkKind;

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
            assert_eq!(LinkKind::from_location(url), expected, "for {url}");
        }
    }

    #[test]
    fn classification_is_case_insensitive() {
        assert_eq!(
            LinkKind::from_location("https://minisforumpc.eu/Products/UM890"),
            LinkKind::Product
        );
    }
}
