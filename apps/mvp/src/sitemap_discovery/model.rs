use std::fmt;
use std::str::FromStr;

/// A parsed sitemap document.
#[derive(Debug, Clone, PartialEq)]
pub enum SitemapDocument {
    Index(SitemapIndex),
    UrlSet(UrlSet),
}

/// A sitemap index that points to one or more child sitemap documents.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SitemapIndex {
    pub sitemaps: Vec<SitemapReference>,
}

impl SitemapIndex {
    pub fn new(sitemaps: Vec<SitemapReference>) -> Self {
        Self { sitemaps }
    }
}

/// A child sitemap referenced by a sitemap index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SitemapReference {
    pub location: String,
    pub last_modified: Option<String>,
}

impl SitemapReference {
    pub fn new(location: impl Into<String>) -> Self {
        Self {
            location: location.into(),
            last_modified: None,
        }
    }

    /// Infers the conventional Shopify sitemap type from the URL path.
    pub fn kind(&self) -> SitemapKind {
        SitemapKind::from_location(&self.location)
    }
}

/// Conventional child sitemap categories used by Shopify storefronts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SitemapKind {
    Products,
    Pages,
    Collections,
    Blogs,
    Unknown,
}

impl SitemapKind {
    pub fn from_location(location: &str) -> Self {
        let path = location
            .split_once('?')
            .map_or(location, |(path, _query)| path);

        if path.contains("sitemap_products_") {
            Self::Products
        } else if path.contains("sitemap_pages_") {
            Self::Pages
        } else if path.contains("sitemap_collections_") {
            Self::Collections
        } else if path.contains("sitemap_blogs_") {
            Self::Blogs
        } else {
            Self::Unknown
        }
    }
}

/// A standard sitemap URL set.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UrlSet {
    pub urls: Vec<SitemapUrl>,
}

impl UrlSet {
    pub fn new(urls: Vec<SitemapUrl>) -> Self {
        Self { urls }
    }
}

/// One URL entry in a standard sitemap URL set.
#[derive(Debug, Clone, PartialEq)]
pub struct SitemapUrl {
    pub location: String,
    pub last_modified: Option<String>,
    pub change_frequency: Option<ChangeFrequency>,
    pub priority: Option<f32>,
    pub images: Vec<SitemapImage>,
}

impl SitemapUrl {
    pub fn new(location: impl Into<String>) -> Self {
        Self {
            location: location.into(),
            last_modified: None,
            change_frequency: None,
            priority: None,
            images: Vec::new(),
        }
    }
}

/// Image metadata nested under a sitemap URL entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SitemapImage {
    pub location: String,
    pub title: Option<String>,
    pub caption: Option<String>,
}

impl SitemapImage {
    pub fn new(location: impl Into<String>) -> Self {
        Self {
            location: location.into(),
            title: None,
            caption: None,
        }
    }
}

/// Values accepted by the standard `<changefreq>` sitemap element.
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
    value: String,
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
    use super::{ChangeFrequency, SitemapKind, SitemapReference};
    use std::str::FromStr;

    #[test]
    fn classifies_minisforum_shopify_child_sitemaps() {
        let cases = [
            (
                "https://minisforumpc.eu/sitemap_products_1.xml?from=1&to=2",
                SitemapKind::Products,
            ),
            (
                "https://minisforumpc.eu/sitemap_pages_1.xml",
                SitemapKind::Pages,
            ),
            (
                "https://minisforumpc.eu/sitemap_collections_1.xml",
                SitemapKind::Collections,
            ),
            (
                "https://minisforumpc.eu/sitemap_blogs_1.xml",
                SitemapKind::Blogs,
            ),
        ];

        for (location, expected) in cases {
            assert_eq!(SitemapReference::new(location).kind(), expected);
        }
    }

    #[test]
    fn leaves_non_shopify_sitemap_names_unclassified() {
        let sitemap = SitemapReference::new("https://example.com/sitemap.xml");

        assert_eq!(sitemap.kind(), SitemapKind::Unknown);
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

        assert_eq!(
            error.to_string(),
            "unsupported sitemap change frequency: sometimes"
        );
    }
}
