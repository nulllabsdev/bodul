//! Link (page-URL) classification.
//!
//! Classifies the individual page links found inside a retailer's sitemaps.
//! Per-retailer rules live alongside each retailer's `sitemap_config` (see
//! [`crate::retailers`]); this module holds the shared [`LinkKind`] type and the
//! default Shopify rule reused by Shopify-based storefronts.

/// The type of a page link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LinkKind {
    Product,
    Catalog,
    Content,
    NotInterested,
    Unknown,
}

impl LinkKind {
    /// Lowercase label.
    pub const fn as_str(self) -> &'static str {
        match self {
            LinkKind::Product => "product",
            LinkKind::Catalog => "catalog",
            LinkKind::Content => "content",
            LinkKind::NotInterested => "not_intersted",
            LinkKind::Unknown => "unknown",
        }
    }
}

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

// ---------------------------------------------------------------------------
// Shared classification helpers for retailer `from_location` rules.
// ---------------------------------------------------------------------------

/// The path portion of `url` (leading `/`, no scheme/host, no query/fragment).
/// Returns `"/"` when there is no path.
pub fn path_of(url: &str) -> &str {
    let after_scheme = url.split_once("://").map_or(url, |(_, rest)| rest);
    let path = match after_scheme.find('/') {
        Some(index) => &after_scheme[index..],
        None => "/",
    };
    let path = path.split(['?', '#']).next().unwrap_or(path);
    if path.is_empty() { "/" } else { path }
}

/// Non-empty path segments, trailing slash trimmed.
pub fn segments(path: &str) -> Vec<&str> {
    path.trim_matches('/')
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect()
}

/// Whether `segment` is a non-empty run of ASCII digits.
pub fn is_numeric(segment: &str) -> bool {
    !segment.is_empty() && segment.bytes().all(|byte| byte.is_ascii_digit())
}

/// Whether the last path segment ends with `-{digits}` (a slug + numeric id).
pub fn ends_with_dash_number(url: &str) -> bool {
    let Some(last) = segments(path_of(url)).into_iter().last() else {
        return false;
    };
    match last.rsplit_once('-') {
        Some((prefix, suffix)) => !prefix.is_empty() && is_numeric(suffix),
        None => false,
    }
}

/// Number of trailing ASCII digits on the last path segment.
pub fn trailing_digit_run(url: &str) -> usize {
    segments(path_of(url))
        .into_iter()
        .last()
        .map(|last| {
            last.bytes()
                .rev()
                .take_while(|byte| byte.is_ascii_digit())
                .count()
        })
        .unwrap_or(0)
}

/// The "MATHEMA" platform family rule, shared by retailers whose sitemap URLs
/// carry explicit page-type suffixes: `/{id}/product` → Product, `/{id}` or
/// `/{id}/l` → Catalog, `/{id}/blog` and `/{id}/page` → Content. `/{id}/manufacture`
/// and `/{id}/offer` are deliberately left Unknown (brand/promo archives).
pub fn numeric_suffix_from_location(url: &str) -> LinkKind {
    let segments = segments(path_of(url));
    let last = segments.last().copied();
    let prev = segments.len().checked_sub(2).map(|index| segments[index]);

    match (last, prev) {
        (Some("product"), Some(id)) if is_numeric(id) => LinkKind::Product,
        (Some("l"), Some(id)) if is_numeric(id) => LinkKind::Catalog,
        (Some("blog"), Some(id)) if is_numeric(id) => LinkKind::Content,
        (Some("page"), Some(id)) if is_numeric(id) => LinkKind::Content,
        (Some("manufacture" | "offer"), Some(id)) if is_numeric(id) => LinkKind::Unknown,
        (Some(id), _) if is_numeric(id) => LinkKind::Catalog,
        _ => LinkKind::Unknown,
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

    #[test]
    fn labels_link_kinds() {
        assert_eq!(LinkKind::Product.as_str(), "product");
        assert_eq!(LinkKind::Catalog.as_str(), "catalog");
        assert_eq!(LinkKind::Content.as_str(), "content");
        assert_eq!(LinkKind::NotInterested.as_str(), "not_intersted");
        assert_eq!(LinkKind::Unknown.as_str(), "unknown");
    }

    #[test]
    fn numeric_suffix_family() {
        let p = |u| numeric_suffix_from_location(u);
        assert_eq!(
            p("https://www.adm.hr/skener-epson/89332/product"),
            LinkKind::Product
        );
        assert_eq!(p("https://www.adm.hr/pos-oprema/26/l"), LinkKind::Catalog);
        assert_eq!(
            p("https://www.hardsoft.hr/ostala-racunala-601/601"),
            LinkKind::Catalog
        );
        assert_eq!(
            p("https://www.adm.hr/politika-privatnosti/15/page"),
            LinkKind::Content
        );
        assert_eq!(p("https://www.adm.hr/nesto/18/blog"), LinkKind::Content);
        // manufacture/offer archives are intentionally not catalog.
        assert_eq!(
            p("https://www.adm.hr/brand/17/manufacture"),
            LinkKind::Unknown
        );
        assert_eq!(p("https://www.adm.hr/o-nama"), LinkKind::Unknown);
    }

    #[test]
    fn path_and_digit_helpers() {
        assert_eq!(path_of("https://x.hr/a/b?c=1#f"), "/a/b");
        assert_eq!(path_of("https://x.hr"), "/");
        assert!(ends_with_dash_number("https://x.hr/foo/bar-123"));
        assert!(!ends_with_dash_number("https://x.hr/foo/bar"));
        assert_eq!(
            trailing_digit_run("https://links.hr/hr/laptop-0101012997"),
            10
        );
        assert_eq!(trailing_digit_run("https://links.hr/hr/informatika-01"), 2);
    }
}
