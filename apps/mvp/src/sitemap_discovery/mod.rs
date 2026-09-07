//! Sitemap discovery.
//!
//! Resolves a retailer's root sitemaps, fetches them (and every child sitemap), and
//! returns the parsed [`SitemapDocument`] tree (roadmap Stage B). Fetching is
//! delegated to [`crate::retailer_data_ingestion`]'s client.

mod model;

mod grouping;
mod processing;
mod retrieval;

pub(crate) const ROOT_SITEMAP_SOURCE: &str = "main";
pub(crate) const MAX_SITEMAP_DEPTH: usize = 32;

pub mod io {
    pub use super::grouping::io::GroupedSitemapContentRepository;
    pub use super::model::RetrievalStatusConversionError;
    pub use super::processing::io::ProcessedSitemapRepository;

    pub use super::grouping::io::{
        GROUP_SITEMAP_CONTENT_COMMAND,
        GroupSitemapContent,
        GroupSitemapContentHandler,
        SITEMAP_CONTENT_GROUPED_EVENT,
        SitemapContentGrouped,
        SitemapContentGroupedSubscriber, //
    };

    pub use super::processing::io::{
        ChildRef,
        PROCESS_SITEMAP_COMMAND,
        Parsed,
        ProcessSitemap,
        ProcessSitemapHandler,
        SITEMAP_PROCESSED_EVENT,
        SitemapProcessed,
        SitemapProcessedSubscriber, //
        parse,
    };

    pub use super::retrieval::io::{
        REQUEST_SITEMAP_RETRIEVAL_COMMAND,
        RawSitemapDocumentRepository,
        RequestSitemapRetrieval,
        RequestSitemapRetrievalError,
        RequestSitemapRetrievalHandler,
        SITEMAP_RETRIEVED_EVENT,
        SitemapRetrieval,
        SitemapRetrievalRepository,
        SitemapRetrieved,
        SitemapRetrievedSubscriber, //
    };
}
