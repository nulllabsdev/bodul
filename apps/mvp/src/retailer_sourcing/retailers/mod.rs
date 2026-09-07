use shared::link::LinkKind;



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

/// Classifies Anker storefront URL paths. Most Anker stores use Shopify-style
/// plural paths; Korea also uses singular `/product/` paths.
pub fn anker_from_location(url: &str) -> LinkKind {
    let path = url.to_lowercase();
    if path.contains("/products/") || path.contains("/product/") {
        LinkKind::Product
    } else if path.contains("/collections/") || path.contains("/collection/") {
        LinkKind::Catalog
    } else if path.contains("/pages/") || path.contains("/blogs/") || path.contains("/blog/") {
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
        .map(|last| last.bytes().rev().take_while(|byte| byte.is_ascii_digit()).count())
        .unwrap_or(0)
}

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
            ("https://minisforumpc.eu/de/products/ms01", LinkKind::Product),
            ("https://minisforumpc.eu/collections/all", LinkKind::Catalog),
            ("https://minisforumpc.eu/pages/about", LinkKind::Content),
            ("https://minisforumpc.eu/blogs/news/a-post", LinkKind::Content),
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
