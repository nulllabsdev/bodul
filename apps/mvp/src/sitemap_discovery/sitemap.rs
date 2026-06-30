use std::fmt;
use std::str::FromStr;

use chrono::{DateTime, Utc};

/// A parsed sitemap as a composite tree.
///
/// A document holds its own page entries (`urls`) and any child documents
/// (`children`), so a sitemap index and its sub-sitemaps form one tree that
/// behaves like a single node: the accessors below recurse through `children`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SitemapDocument {
    /// This document's own URL, when known (the resolved root or an index entry).
    pub location: Option<String>,
    /// `<lastmod>` for this document, when listed in a parent index.
    pub last_modified: Option<DateTime<Utc>>,
    /// `<url>` entries declared directly in this document.
    pub urls: Vec<SitemapUrl>,
    /// Child documents (from a `<sitemapindex>`), already fetched and parsed.
    pub children: Vec<SitemapDocument>,
}

impl SitemapDocument {
    /// An empty document with no location.
    pub fn new() -> Self {
        Self::default()
    }

    /// An empty document located at `location`.
    pub fn at(location: impl Into<String>) -> Self {
        let location = location.into();
        debug_assert!(!location.is_empty(), "sitemap location must not be empty");
        Self {
            location: Some(location),
            ..Self::default()
        }
    }

    /// Infers the sitemap kind from this document's URL location.
    /// A document with no location is [`SitemapKind::Other`].
    pub fn kind(&self) -> SitemapKind {
        match &self.location {
            Some(location) => SitemapKind::from_location(location),
            None => SitemapKind::Other,
        }
    }

    /// Every `<url>` entry in this document and all of its descendants.
    pub fn all_urls(&self, source: &str) -> impl Iterator<Item = SitemapUrl> {
        let mut urls = Vec::new();
        self.collect_urls(&mut urls, source);
        urls.into_iter()
    }

    /// Every `<url>` entry that lives in a document of the given `kind`, across
    /// this document and all of its descendants.
    pub fn urls_of_kind(&self, kind: SitemapKind) -> Vec<&SitemapUrl> {
        let mut urls = Vec::new();
        self.collect_urls_of_kind(kind, &mut urls);
        urls
    }

    fn collect_urls(&self, urls: &mut Vec<SitemapUrl>, source: &str) {
        let source = self.location.as_deref().unwrap_or(source);
        let sourced_urls = self
            .urls
            .iter()
            .cloned()
            .map(|mut url| {
                url.source = source.to_string();
                url
            })
            .collect::<Vec<_>>();
        urls.extend(sourced_urls);
        for child in &self.children {
            child.collect_urls(urls, source);
        }
    }

    fn collect_urls_of_kind<'a>(&'a self, kind: SitemapKind, urls: &mut Vec<&'a SitemapUrl>) {
        if self.kind() == kind {
            urls.extend(self.urls.iter());
        }
        for child in &self.children {
            child.collect_urls_of_kind(kind, urls);
        }
    }
}

/// Sitemap kind inferred from the URL location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SitemapKind {
    Product,
    Collection,
    Catalog,
    Other,
}

impl SitemapKind {
    /// Infers the kind from the filename portion of the URL.
    ///
    /// Strips query parameters, extracts the last path segment, lowercases it,
    /// and checks for keywords.
    pub fn from_location(location: &str) -> Self {
        let path = location.split_once('?').map_or(location, |(path, _query)| path);

        let filename = path.rsplit('/').next().unwrap_or(path).to_lowercase();

        if filename.contains("product") {
            Self::Product
        } else if filename.contains("catalog") {
            Self::Catalog
        } else if filename.contains("collection") {
            Self::Collection
        } else {
            Self::Other
        }
    }
}

/// One URL entry in a standard sitemap URL set.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SitemapUrl {
    pub location: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub source: String,
    pub last_modified: Option<DateTime<Utc>>,
    pub change_frequency: Option<ChangeFrequency>,
    /// Priority in `[0.0, 1.0]`.  Use [`with_priority`](Self::with_priority)
    /// to set a validated value; the field is `pub` for deserialization but
    /// callers are responsible for honouring the range.
    pub priority: Option<f32>,
    pub images: Vec<SitemapImage>,
}

impl SitemapUrl {
    pub fn new(location: impl Into<String>, source: impl Into<String>) -> Self {
        let location = location.into();
        let source = source.into();
        debug_assert!(!location.is_empty(), "sitemap URL location must not be empty");
        debug_assert!(!source.is_empty(), "sitemap URL source must not be empty");
        Self {
            location,
            source,
            last_modified: None,
            change_frequency: None,
            priority: None,
            images: Vec::new(),
        }
    }

    /// Set `priority`, validating that the value is in `[0.0, 1.0]`.
    ///
    /// # Panics
    ///
    /// Panics if `priority` is not in range.
    pub fn with_priority(mut self, priority: f32) -> Self {
        assert!(
            (0.0..=1.0).contains(&priority),
            "sitemap priority must be in [0.0, 1.0], got {priority}"
        );
        self.priority = Some(priority);
        self
    }
}

/// Image metadata nested under a sitemap URL entry.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SitemapImage {
    pub location: String,
    pub title: Option<String>,
    pub caption: Option<String>,
}

impl SitemapImage {
    pub fn new(location: impl Into<String>) -> Self {
        let location = location.into();
        debug_assert!(!location.is_empty(), "sitemap image location must not be empty");
        Self {
            location,
            title: None,
            caption: None,
        }
    }
}

/// Values accepted by the standard `<changefreq>` sitemap element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChangeFrequency {
    Always,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Never,
}

impl ChangeFrequency {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::Hourly => "hourly",
            Self::Daily => "daily",
            Self::Weekly => "weekly",
            Self::Monthly => "monthly",
            Self::Yearly => "yearly",
            Self::Never => "never",
        }
    }
}

impl fmt::Display for ChangeFrequency {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseChangeFrequencyError {
    pub value: String,
}

impl fmt::Display for ParseChangeFrequencyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "unsupported sitemap change frequency: {}", self.value)
    }
}

impl std::error::Error for ParseChangeFrequencyError {}

impl FromStr for ChangeFrequency {
    type Err = ParseChangeFrequencyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "always" => Ok(Self::Always),
            "hourly" => Ok(Self::Hourly),
            "daily" => Ok(Self::Daily),
            "weekly" => Ok(Self::Weekly),
            "monthly" => Ok(Self::Monthly),
            "yearly" => Ok(Self::Yearly),
            "never" => Ok(Self::Never),
            value => Err(ParseChangeFrequencyError {
                value: value.to_owned(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ChangeFrequency, SitemapDocument, SitemapKind, SitemapUrl};
    use std::str::FromStr;

    #[test]
    fn infers_product_kind_from_shopify_url() {
        let sitemap = SitemapDocument::at("https://minisforumpc.eu/sitemap_products_1.xml?from=1&to=2");
        assert_eq!(sitemap.kind(), SitemapKind::Product);
    }

    #[test]
    fn infers_collection_kind_from_shopify_url() {
        let sitemap = SitemapDocument::at("https://minisforumpc.eu/sitemap_collections_1.xml");
        assert_eq!(sitemap.kind(), SitemapKind::Collection);
    }

    #[test]
    fn infers_product_kind_from_generic_url() {
        let sitemap = SitemapDocument::at("https://example.com/products-sitemap.xml");
        assert_eq!(sitemap.kind(), SitemapKind::Product);
    }

    #[test]
    fn infers_catalog_kind_from_generic_url() {
        let sitemap = SitemapDocument::at("https://example.com/catalog-sitemap.xml");
        assert_eq!(sitemap.kind(), SitemapKind::Catalog);
    }

    #[test]
    fn infers_collection_kind_from_generic_url() {
        let sitemap = SitemapDocument::at("https://example.com/collections.xml");
        assert_eq!(sitemap.kind(), SitemapKind::Collection);
    }

    #[test]
    fn leaves_non_matching_urls_as_other() {
        let sitemap = SitemapDocument::at("https://example.com/sitemap.xml");
        assert_eq!(sitemap.kind(), SitemapKind::Other);
    }

    #[test]
    fn kind_matching_is_case_insensitive() {
        let cases = [
            ("https://example.com/Sitemap_Products_1.xml", SitemapKind::Product),
            ("https://example.com/SITEMAP_COLLECTIONS_1.XML", SitemapKind::Collection),
            ("https://example.com/Catalog.xml", SitemapKind::Catalog),
        ];

        for (url, expected) in cases {
            assert_eq!(SitemapDocument::at(url).kind(), expected);
        }
    }

    #[test]
    fn kind_detection_ignores_query_params() {
        let sitemap = SitemapDocument::at("https://example.com/sitemap_products_1.xml?from=1&to=2&nocache=12345");
        assert_eq!(sitemap.kind(), SitemapKind::Product);
    }

    #[test]
    fn product_kind_takes_precedence_over_catalog() {
        // "catalog_product" contains both keywords — "product" is checked first.
        let sitemap = SitemapDocument::at("https://example.com/catalog_product_sitemap.xml");
        assert_eq!(sitemap.kind(), SitemapKind::Product);
    }

    #[test]
    fn queries_aggregate_across_the_tree() {
        let mut products = SitemapDocument::at("https://minisforumpc.eu/sitemap_products_1.xml");
        products.urls = vec![
            SitemapUrl::new(
                "https://minisforumpc.eu/products/um890",
                "https://minisforumpc.eu/sitemap.xml",
            ),
            SitemapUrl::new(
                "https://minisforumpc.eu/products/ms01",
                "https://minisforumpc.eu/sitemap.xml",
            ),
        ];

        let mut collections = SitemapDocument::at("https://minisforumpc.eu/sitemap_collections_1.xml");
        collections.urls = vec![SitemapUrl::new(
            "https://minisforumpc.eu/collections/all",
            "https://minisforumpc.eu/sitemap.xml",
        )];

        let mut root = SitemapDocument::at("https://minisforumpc.eu/sitemap.xml");
        root.children = vec![products, collections];

        // The tree answers as one node, recursing into children.
        assert_eq!(root.all_urls("https://minisforumpc.eu/sitemap.xml").count(), 3);
        assert_eq!(root.urls_of_kind(SitemapKind::Product).len(), 2);
        assert_eq!(root.urls_of_kind(SitemapKind::Collection).len(), 1);
        assert_eq!(root.urls_of_kind(SitemapKind::Catalog).len(), 0);
    }

    #[test]
    fn all_urls_sets_source_from_owning_document() {
        let mut products = SitemapDocument::at("https://example.com/sitemap_products.xml");
        products.urls = vec![SitemapUrl::new("https://example.com/products/a", "stale")];

        let mut root = SitemapDocument::at("https://example.com/sitemap.xml");
        root.urls = vec![SitemapUrl::new("https://example.com/", "stale")];
        root.children = vec![products];

        let urls = root
            .all_urls("https://example.com/fallback-sitemap.xml")
            .collect::<Vec<_>>();

        assert_eq!(urls[0].source, "https://example.com/sitemap.xml");
        assert_eq!(urls[1].source, "https://example.com/sitemap_products.xml");
    }

    #[test]
    fn parses_and_formats_change_frequency() {
        let value = ChangeFrequency::from_str("daily").expect("daily is valid");

        assert_eq!(value, ChangeFrequency::Daily);
        assert_eq!(value.to_string(), "daily");
    }

    #[test]
    fn rejects_unknown_change_frequency() {
        let error = ChangeFrequency::from_str("sometimes").expect_err("must fail");

        assert_eq!(error.to_string(), "unsupported sitemap change frequency: sometimes");
        assert_eq!(error.value, "sometimes");
    }

    #[test]
    fn parses_all_change_frequency_variants() {
        let cases = [
            ("always", ChangeFrequency::Always),
            ("hourly", ChangeFrequency::Hourly),
            ("daily", ChangeFrequency::Daily),
            ("weekly", ChangeFrequency::Weekly),
            ("monthly", ChangeFrequency::Monthly),
            ("yearly", ChangeFrequency::Yearly),
            ("never", ChangeFrequency::Never),
        ];

        for (input, expected) in cases {
            let parsed = ChangeFrequency::from_str(input).expect("must parse");
            assert_eq!(parsed, expected, "failed roundtrip for {input}");
            assert_eq!(parsed.to_string(), input, "display mismatch for {input}");
        }
    }

    #[test]
    fn sets_priority_in_range() {
        let url = SitemapUrl::new("https://example.com/product", "https://example.com/sitemap.xml").with_priority(0.5);
        assert_eq!(url.priority, Some(0.5));
    }

    #[test]
    fn sets_priority_at_bounds() {
        let url = SitemapUrl::new("https://example.com/product", "https://example.com/sitemap.xml").with_priority(0.0);
        assert_eq!(url.priority, Some(0.0));

        let url = SitemapUrl::new("https://example.com/product", "https://example.com/sitemap.xml").with_priority(1.0);
        assert_eq!(url.priority, Some(1.0));
    }

    #[test]
    #[should_panic(expected = "sitemap priority must be in [0.0, 1.0]")]
    fn panics_on_priority_out_of_range() {
        SitemapUrl::new("https://example.com/product", "https://example.com/sitemap.xml").with_priority(1.5);
    }
}
