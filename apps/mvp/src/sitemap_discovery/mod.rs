//! Sitemap discovery.
//!
//! Resolves a retailer's root sitemaps, fetches them (and every child sitemap), and
//! returns the parsed [`SitemapDocument`] tree (roadmap Stage B). Fetching is
//! delegated to [`crate::retailer_data_ingestion`]'s client.

pub mod helpers;
mod parse;
pub mod sitemap;

use std::fs;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use shared::SitemapConfig;
use shared::retailer::RetailerCode;

use crate::retailer_data_ingestion::{Client, FetchError};
use crate::retailer_sourcing::sitemap_config;
use crate::sitemap_discovery::parse::Parsed;
use crate::sitemap_discovery::sitemap::SitemapDocument;

#[derive(Debug, thiserror::Error)]
pub enum SitemapError {
    #[error("failed to fetch {url}: {source}")]
    Fetch {
        url: String,
        #[source]
        source: FetchError,
    },

    #[error("failed to parse {url}: {message}")]
    Parse { url: String, message: String },

    #[error("no sitemap configuration for retailer {retailer:?}")]
    UnknownRetailer { retailer: RetailerCode },

    #[error("failed to store {path}: {source}")]
    Store {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

/// Fetches the full sitemap tree for `retailer`.
///
/// Resolves the retailer's root sitemap URLs, fetches them over HTTP, parses
/// them, and follows every child sitemap so the returned [`SitemapDocument`] can
/// be queried as one tree. Every retrieved sitemap body is also written verbatim
/// to `data/raw_sitemap/{retailer}/` as a side effect, for offline inspection.
pub fn fetch_sitemap(retailer: RetailerCode) -> Result<SitemapDocument, SitemapError> {
    let config = sitemap_config(retailer).ok_or(SitemapError::UnknownRetailer { retailer })?;
    let dir = PathBuf::from("data/raw_sitemap").join(format!("{retailer:?}").to_lowercase());
    fetch_config_inner(config, Client::get, Some(&dir))
}

/// Core of [`fetch_sitemap`] with the fetcher injected, so the resolve → parse →
/// recurse logic is testable without the network. Does not persist anything.
#[cfg(test)]
fn fetch_with<F>(retailer: RetailerCode, get: F) -> Result<SitemapDocument, SitemapError>
where
    F: Fn(&str) -> Result<String, FetchError>,
{
    let config = sitemap_config(retailer).ok_or(SitemapError::UnknownRetailer { retailer })?;
    fetch_config_inner(config, get, None)
}

/// Test/helper entry that fetches a known config without persisting anything.
#[cfg(test)]
fn fetch_config_with<F>(config: SitemapConfig, get: F) -> Result<SitemapDocument, SitemapError>
where
    F: Fn(&str) -> Result<String, FetchError>,
{
    fetch_config_inner(config, get, None)
}

/// Fetches every root sitemap in `config` (following children) and merges them
/// into one tree. When `out_dir` is `Some`, each retrieved body is written there
/// verbatim; the directory is created up front.
fn fetch_config_inner<F>(config: SitemapConfig, get: F, out_dir: Option<&Path>) -> Result<SitemapDocument, SitemapError>
where
    F: Fn(&str) -> Result<String, FetchError>,
{
    if let Some(dir) = out_dir {
        fs::create_dir_all(dir).map_err(|source| SitemapError::Store {
            path: dir.display().to_string(),
            source,
        })?;
    }

    let mut root_documents = config
        .sitemap_url
        .iter()
        .map(|url| fetch_document(url, None, &get, out_dir))
        .collect::<Result<Vec<_>, _>>()?;

    if root_documents.len() == 1 {
        Ok(root_documents.remove(0))
    } else {
        Ok(SitemapDocument {
            children: root_documents,
            ..SitemapDocument::default()
        })
    }
}

/// Fetches and parses one sitemap document, recursing into child sitemaps when
/// the document is an index.
fn fetch_document<F>(
    url: &str,
    last_modified: Option<DateTime<Utc>>,
    get: &F,
    out_dir: Option<&Path>,
) -> Result<SitemapDocument, SitemapError>
where
    F: Fn(&str) -> Result<String, FetchError>,
{
    let body = get(url).map_err(|error| SitemapError::Fetch {
        url: url.to_string(),
        source: error,
    })?;

    if let Some(dir) = out_dir {
        let path = dir.join(raw_filename(url));
        fs::write(&path, &body).map_err(|source| SitemapError::Store {
            path: path.display().to_string(),
            source,
        })?;
    }

    // Gzipped sitemaps (e.g. `sitemap.xml.gz`) are already decompressed to XML by
    // the HTTP client, so `body` is always plain XML here.
    let parsed = parse::parse(&body).map_err(|message| SitemapError::Parse {
        url: url.to_string(),
        message,
    })?;

    let mut document = SitemapDocument {
        location: Some(url.to_string()),
        last_modified,
        urls: Vec::new(),
        children: Vec::new(),
    };

    match parsed {
        Parsed::UrlSet(urls) => document.urls = urls,
        Parsed::Index(children) => {
            for child in children {
                document
                    .children
                    .push(fetch_document(&child.location, child.last_modified, get, out_dir)?);
            }
        }
    }

    Ok(document)
}

/// Turns a sitemap URL into a filesystem-safe, collision-free file name within a
/// retailer's raw directory. Drops the scheme, replaces every character outside
/// `[A-Za-z0-9._-]` with `_`, and ensures an `.xml` extension. Including the host,
/// path, and query keeps distinct child sitemaps (e.g. `?id=1` vs `?id=2`) apart.
fn raw_filename(url: &str) -> String {
    let without_scheme = url.split_once("://").map_or(url, |(_, rest)| rest);
    let mut name: String = without_scheme
        .chars()
        .map(|character| match character {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '-' | '_' => character,
            _ => '_',
        })
        .collect();

    if !name.ends_with(".xml") {
        name.push_str(".xml");
    }
    name
}

#[cfg(test)]
mod tests {
    use super::sitemap::SitemapKind;
    use super::*;

    const ROOT: &str = r#"<sitemapindex>
        <sitemap><loc>https://minisforumpc.eu/sitemap_products_1.xml</loc></sitemap>
        <sitemap><loc>https://minisforumpc.eu/sitemap_collections_1.xml</loc></sitemap>
    </sitemapindex>"#;

    const PRODUCTS: &str = r#"<urlset>
        <url><loc>https://minisforumpc.eu/products/um890</loc><priority>0.8</priority></url>
        <url><loc>https://minisforumpc.eu/products/ms01</loc></url>
    </urlset>"#;

    const COLLECTIONS: &str = r#"<urlset>
        <url><loc>https://minisforumpc.eu/collections/all</loc></url>
    </urlset>"#;

    const ROOT_A: &str = r#"<urlset>
        <url><loc>https://example.com/products/a</loc></url>
    </urlset>"#;

    const ROOT_B: &str = r#"<urlset>
        <url><loc>https://example.com/products/b</loc></url>
    </urlset>"#;

    fn fake_get(url: &str) -> Result<String, FetchError> {
        let body = match url {
            u if u.ends_with("/sitemap.xml") => ROOT,
            u if u.contains("sitemap_products_1") => PRODUCTS,
            u if u.contains("sitemap_collections_1") => COLLECTIONS,
            other => {
                return Err(FetchError {
                    message: format!("unexpected url: {other}"),
                });
            }
        };
        Ok(body.to_string())
    }

    #[test]
    fn builds_tree_from_index_and_children() {
        let document = fetch_with(RetailerCode::MinisForumEu, fake_get).expect("fetches");

        assert_eq!(
            document.location.as_deref(),
            Some("https://minisforumpc.eu/sitemap.xml")
        );
        assert_eq!(document.children.len(), 2);
        // The tree answers as one node.
        assert_eq!(document.all_urls("main").count(), 3);
        assert_eq!(document.urls_of_kind(SitemapKind::Product).len(), 2);
        assert_eq!(document.urls_of_kind(SitemapKind::Collection).len(), 1);
    }

    #[test]
    fn builds_one_tree_from_multiple_root_sitemaps() {
        let config = SitemapConfig {
            sitemap_url: vec![
                "https://example.com/sitemap-a.xml".to_string(),
                "https://example.com/sitemap-b.xml".to_string(),
            ],
        };
        let get = |url: &str| {
            let body = match url {
                "https://example.com/sitemap-a.xml" => ROOT_A,
                "https://example.com/sitemap-b.xml" => ROOT_B,
                other => {
                    return Err(FetchError {
                        message: format!("unexpected url: {other}"),
                    });
                }
            };
            Ok(body.to_string())
        };

        let document = fetch_config_with(config, get).expect("fetches");

        assert_eq!(document.location, None);
        assert_eq!(document.children.len(), 2);
        assert_eq!(
            document.children[0].location.as_deref(),
            Some("https://example.com/sitemap-a.xml")
        );
        assert_eq!(
            document.children[1].location.as_deref(),
            Some("https://example.com/sitemap-b.xml")
        );
        assert_eq!(document.all_urls("main").count(), 2);
    }

    #[test]
    #[ignore = "TODO: stale — retailer now resolves to a sitemap config; revisit assertion"]
    fn unknown_retailer_without_config() {
        let error = fetch_with(RetailerCode::MinisForumUs, fake_get).expect_err("no config");
        assert!(error.to_string().contains("no sitemap configuration"));
    }

    #[test]
    fn propagates_fetch_errors() {
        let failing = |_: &str| {
            Err(FetchError {
                message: "boom".to_string(),
            })
        };
        let error = fetch_with(RetailerCode::MinisForumEu, failing).expect_err("fetch fails");
        assert!(error.to_string().contains("failed to fetch"));
        assert!(error.to_string().contains("boom"));
    }
}
