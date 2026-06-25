//! Retailer data ingestion.
//!
//! Ingests scraped retailer data into the pipeline. Retailer data is hardcoded
//! in Phase 0.

use std::fmt;
use std::io::Read;

use flate2::read::GzDecoder;

/// An error fetching a remote resource.
#[derive(Debug, Clone)]
pub struct FetchError {
    pub message: String,
}

impl fmt::Display for FetchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for FetchError {}

/// A browser-like User-Agent. Many storefronts reject the default `reqwest/x.y`
/// agent with `403 Forbidden`; sending a common one is enough for basic fetching.
/// (Rotating agents / proxies for anti-scraping is deferred — roadmap Stage D.)
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

/// A minimal blocking HTTP client for fetching retailer resources.
pub struct Client {}

/// The first two bytes of every gzip stream (RFC 1952).
const GZIP_MAGIC: [u8; 2] = [0x1f, 0x8b];

impl Client {
    /// Fetches `url` and returns the response body as text.
    ///
    /// Some sitemaps are served gzip-compressed at the content level (e.g.
    /// `sitemap.xml.gz`); these are transparently decompressed here so callers
    /// always receive plain XML. Detection is by gzip magic bytes, so it works
    /// regardless of the URL suffix or a mislabelled `Content-Type`.
    pub fn get(url: &str) -> Result<String, FetchError> {
        let client = reqwest::blocking::Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .map_err(|error| FetchError {
                message: error.to_string(),
            })?;

        let bytes = client
            .get(url)
            .send()
            .and_then(|response| response.error_for_status())
            .and_then(|response| response.bytes())
            .map_err(|error| FetchError {
                message: error.to_string(),
            })?;

        decode_body(&bytes)
    }
}

/// Decodes a response body to text, gunzipping it first when it is a gzip stream.
fn decode_body(bytes: &[u8]) -> Result<String, FetchError> {
    if bytes.starts_with(&GZIP_MAGIC) {
        let mut text = String::new();
        GzDecoder::new(bytes)
            .read_to_string(&mut text)
            .map_err(|error| FetchError {
                message: format!("gzip decode failed: {error}"),
            })?;
        Ok(text)
    } else {
        // Sitemaps are required to be UTF-8; lossy keeps a stray byte from
        // failing the whole fetch.
        Ok(String::from_utf8_lossy(bytes).into_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::Compression;
    use flate2::write::GzEncoder;
    use std::io::Write;

    fn gzip(text: &str) -> Vec<u8> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(text.as_bytes()).unwrap();
        encoder.finish().unwrap()
    }

    #[test]
    fn decodes_gzipped_body() {
        let xml = "<urlset><url><loc>https://example.com/a</loc></url></urlset>";
        assert_eq!(decode_body(&gzip(xml)).unwrap(), xml);
    }

    #[test]
    fn passes_plain_body_through() {
        let xml = "<urlset></urlset>";
        assert_eq!(decode_body(xml.as_bytes()).unwrap(), xml);
    }
}
