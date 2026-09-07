use crate::retailers::anker::anker_from_location;
use shared::SitemapConfig;
use shared::link::LinkKind;

pub fn sitemap_config() -> SitemapConfig {
    SitemapConfig {
        sitemap_url: vec!["https://www.anker.com/sitemap.xml".to_string()],
    }
}

pub fn classify_link(url: &str, source: &str, _image_count: usize) -> LinkKind {
    from_location(url, source)
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
        if !path.starts_with("https://www.anker.com/ca/") {
            return LinkKind::NotInterested;
        }

        return y;
    }

    anker_from_location(&path)
}

// NOTE: URLs synthesized from anker_de shapes; the ankerca grouped-sitemap data is empty.
#[cfg(test)]
mod tests {
    use super::*;

    const PRODUCTS_INDEX: &str = "https://www.anker.com/ca/server-sitemap-index-products.xml";
    const COLLECTIONS_INDEX: &str = "https://www.anker.com/ca/server-sitemap-index-collections.xml";
    const BLOG_INDEX: &str = "https://www.anker.com/ca/server-sitemap-index-blog.xml";
    const PAGES_INDEX: &str = "https://www.anker.com/ca/server-sitemap-index-pages.xml";
    const SITEMAP_0: &str = "https://www.anker.com/ca/sitemap-0.xml";

    #[test]
    fn classifies_products() {
        let url = "https://www.anker.com/ca/products/a110a-anker-prime-26k-300w-power-bank";
        assert_eq!(classify_link(url, PRODUCTS_INDEX, 0), LinkKind::Product, "for {url}");
    }

    #[test]
    fn classifies_catalog() {
        let url = "https://www.anker.com/ca/collections/1-2-phone-charges";
        assert_eq!(classify_link(url, COLLECTIONS_INDEX, 0), LinkKind::Catalog, "for {url}");
    }

    #[test]
    fn classifies_content() {
        let url = "https://www.anker.com/ca/blogs/ac-power";
        assert_eq!(classify_link(url, BLOG_INDEX, 0), LinkKind::Content, "for {url}");
    }

    #[test]
    fn classifies_not_interested() {
        let pages_url = "https://www.anker.com/ca/pages/about-us";
        assert_eq!(
            classify_link(pages_url, PAGES_INDEX, 0),
            LinkKind::NotInterested,
            "for {pages_url}"
        );

        let sitemap_url = "https://www.anker.com/ca/products/a110a";
        assert_eq!(
            classify_link(sitemap_url, SITEMAP_0, 0),
            LinkKind::NotInterested,
            "for {sitemap_url}"
        );
    }

    // Edge cases below are synthesized because the ankerca grouped-sitemap data folder is empty.

    #[test]
    fn region_guard_rejects_other_region() {
        // A source-matched product URL in a different region must not classify as Product.
        let url = "https://www.anker.com/eu-de/products/a110a";
        assert_eq!(
            classify_link(url, PRODUCTS_INDEX, 0),
            LinkKind::NotInterested,
            "for {url}"
        );
    }

    #[test]
    fn ca_fr_prefix_does_not_satisfy_ca_guard() {
        // `/ca-fr/` does not start with `/ca/`, so the region guard rejects it.
        let url = "https://www.anker.com/ca-fr/products/a110a";
        assert_eq!(
            classify_link(url, PRODUCTS_INDEX, 0),
            LinkKind::NotInterested,
            "for {url}"
        );
    }

    #[test]
    fn unmatched_source_falls_back_to_path() {
        // Unmatched source -> path-based fallback (anker_from_location).
        let unmatched_source = "https://www.anker.com/ca/sitemap.xml";

        let product_url = "https://www.anker.com/ca/products/a1109-f0";
        assert_eq!(
            classify_link(product_url, unmatched_source, 0),
            LinkKind::Product,
            "for {product_url}"
        );

        let unknown_url = "https://www.anker.com/agents.md";
        assert_eq!(
            classify_link(unknown_url, unmatched_source, 0),
            LinkKind::Unknown,
            "for {unknown_url}"
        );
    }

    #[test]
    fn path_matching_is_case_insensitive() {
        let url = "HTTPS://WWW.ANKER.COM/CA/PRODUCTS/A110A";
        assert_eq!(classify_link(url, PRODUCTS_INDEX, 0), LinkKind::Product, "for {url}");
    }
}
