//! Sitemap discovery.
//!
//! Locates a retailer's sitemap entry points (e.g. via `robots.txt` or the
//! well-known `/sitemap.xml` location) as the first step of sitemap sourcing
//! (roadmap Stage B).

pub mod model;

pub use model::{
    ChangeFrequency, ParseChangeFrequencyError, SitemapDocument, SitemapImage,
    SitemapIndex, SitemapKind, SitemapReference, SitemapUrl, UrlSet,
};
