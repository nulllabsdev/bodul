pub mod link;
pub mod retailer;
pub mod retailers;

#[derive(Debug, Clone)]
pub struct SitemapConfig {
    pub sitemap_url: Vec<String>,
}
