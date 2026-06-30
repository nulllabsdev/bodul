pub mod io {

    pub use super::models::{
        ChangeFrequency, RawSitemapDocument, SitemapDocument, SitemapImage, SitemapKind, SitemapUrl,
    };
}

mod models {
    use chrono::{DateTime, Utc};

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct RawSitemapDocument {
        pub url: String,
        pub last_modified: Option<DateTime<Utc>>,
        pub body: String,
        pub body_size: usize,
    }

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
        pub fn build(
            location: Option<String>,
            last_modified: Option<DateTime<Utc>>,
            urls: Vec<SitemapUrl>,
            children: Vec<SitemapDocument>,
        ) -> Self {
            SitemapDocument {
                location,
                last_modified,
                urls,
                children,
            }
        }

        /// Every `<url>` entry in this document and all of its descendants.
        pub fn all_urls(&self, source: &str) -> impl Iterator<Item = SitemapUrl> {
            let mut urls = Vec::new();
            self.collect_urls(&mut urls, source);
            urls.into_iter()
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
    }

    /// Sitemap kind inferred from the URL location.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum SitemapKind {
        Product,
        Collection,
        Catalog,
        Other,
    }

    /// One URL entry in a standard sitemap URL set.
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct SitemapUrl {
        pub location: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        pub source: String,
        pub last_modified: Option<DateTime<Utc>>,
        pub change_frequency: Option<ChangeFrequency>,
        /// Priority in `[0.0, 1.0]`.
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

    #[derive(serde::Serialize, serde::Deserialize, strum::EnumString, strum::IntoStaticStr)]
    #[serde(rename_all = "lowercase")]
    #[strum(serialize_all = "lowercase")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
        pub fn as_str(self) -> &'static str {
            self.into()
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn all_urls_sets_source_from_owning_document() {
        let products = SitemapDocument {
            location: Some("https://example.com/sitemap_products.xml".into()),
            urls: vec![
                SitemapUrl {
                    location: "https://example.com/products/a".to_string(),
                    source: "stale".to_string(),
                    last_modified: None,
                    change_frequency: None,
                    priority: None,
                    images: Vec::new(),
                },
                SitemapUrl {
                    location: "https://example.com/products/b".to_string(),
                    source: "stale".to_string(),
                    last_modified: None,
                    change_frequency: None,
                    priority: None,
                    images: Vec::new(),
                },
            ],
            ..SitemapDocument::default()
        };

        let root = SitemapDocument {
            location: Some("https://example.com/sitemap.xml".into()),
            urls: vec![SitemapUrl {
                location: "https://example.com/".to_string(),
                source: "stale".to_string(),
                last_modified: None,
                change_frequency: None,
                priority: None,
                images: Vec::new(),
            }],
            children: vec![products],
            ..SitemapDocument::default()
        };

        let urls = root
            .all_urls("https://example.com/fallback-sitemap.xml")
            .collect::<Vec<_>>();

        assert_eq!(urls[0].source, "https://example.com/sitemap.xml");
        assert_eq!(urls[1].source, "https://example.com/sitemap_products.xml");
    }

    use crate::lib_sitemap::io::ChangeFrequency;
    use crate::lib_sitemap::models::{SitemapDocument, SitemapUrl};
    use std::str::FromStr;

    #[test]
    fn parses_and_formats_change_frequency() {
        let value = ChangeFrequency::from_str("daily").expect("daily is valid");

        assert_eq!(value, ChangeFrequency::Daily);
        assert_eq!(value.as_str(), "daily");
    }

    #[test]
    fn rejects_unknown_change_frequency() {
        let error = ChangeFrequency::from_str("sometimes").expect_err("must fail");

        assert_eq!(error.to_string(), "Matching variant not found");
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
            assert_eq!(parsed.as_str(), input, "display mismatch for {input}");
        }
    }
}
