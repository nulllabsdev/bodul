//! Groups every stored processed sitemap and writes the result to disk.
//!
//! Loads all rows from `processed_sitemaps`, runs the shared grouping logic on each,
//! and writes bare grouped links to `data/grouped-sitemaps/{retailer}/{id}.json`.
//! Also dumps each raw processed sitemap document to
//! `data/processed-sitemaps/{retailer}-sitemap-{id}.json`, in the flat
//! `{slug}-sitemap-...` naming `detect.rs` already expects as its input.

use std::fs;
use std::path::PathBuf;

use mvp::database::{DatabaseConfig, connect};
use mvp::lib_sitemap::io::SitemapDocument;
use mvp::sitemap_discovery::io::{GroupedLinks, ProcessedSitemapRepository};

fn main() {
    dotenvy::dotenv().ok();
    let _guard = mvp::logging::init();
    let config = DatabaseConfig::from_env();
    let pool = connect(&config).expect("DB pool");
    let repo = ProcessedSitemapRepository::new(pool);

    let processed = repo.load_all().expect("load processed sitemaps");
    let total = processed.len();
    let mut written = 0usize;
    let mut failed = 0usize;

    for sitemap in &processed {
        let links = GroupedLinks::from_document(sitemap.retailer_code, &sitemap.document);
        let dir = PathBuf::from("data/grouped-sitemaps").join(sitemap.retailer_code.slug());
        let out = dir.join(format!("{}.json", sitemap.id));

        let processed_dir = PathBuf::from("data/processed-sitemaps");
        let processed_out = processed_dir.join(format!("{}-sitemap-{}.json", sitemap.retailer_code.slug(), sitemap.id));

        match write_grouped(&dir, &out, &links)
            .and_then(|()| write_processed(&processed_dir, &processed_out, &sitemap.document))
        {
            Ok(()) => {
                println!(
                    "ok   {} -> {} + {}",
                    sitemap.retailer_code.slug(),
                    out.display(),
                    processed_out.display()
                );
                written += 1;
            }
            Err(error) => {
                tracing::error!("fail {}: {error}", out.display());
                failed += 1;
            }
        }
    }

    println!("done: {written}/{total} grouped, {failed} failed");
    if total > 0 && written == 0 {
        std::process::exit(1);
    }
}

fn write_grouped(dir: &std::path::Path, out: &std::path::Path, links: &GroupedLinks) -> Result<(), String> {
    fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let json = serde_json::to_string_pretty(links).map_err(|e| e.to_string())?;
    fs::write(out, json).map_err(|e| e.to_string())
}

fn write_processed(dir: &std::path::Path, out: &std::path::Path, document: &SitemapDocument) -> Result<(), String> {
    fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let json = serde_json::to_string_pretty(document).map_err(|e| e.to_string())?;
    fs::write(out, json).map_err(|e| e.to_string())
}
