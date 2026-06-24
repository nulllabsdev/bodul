//! Retailer data ingestion.
//!
//! Ingests scraped retailer data into the pipeline. Retailer data is hardcoded
//! in Phase 0.

use std::fmt;

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

impl Client {
    /// Fetches `url` and returns the response body as text.
    pub fn get(url: &str) -> Result<String, FetchError> {
        let client = reqwest::blocking::Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .map_err(|error| FetchError {
                message: error.to_string(),
            })?;

        client
            .get(url)
            .send()
            .and_then(|response| response.error_for_status())
            .and_then(|response| response.text())
            .map_err(|error| FetchError {
                message: error.to_string(),
            })
    }
}
