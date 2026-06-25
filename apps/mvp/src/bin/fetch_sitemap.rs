//! Fetches every configured retailer's sitemap and writes each to disk as pretty
//! JSON. Retailers without a configured storefront are skipped; a failure on one
//! retailer does not stop the others.

use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;
use mvp::sitemap_discovery;
use shared::retailer::RetailerCode;

fn main() {
    let data_dir = PathBuf::from("data");
    if let Err(error) = fs::create_dir_all(&data_dir) {
        eprintln!("error creating {}: {error}", data_dir.display());
        std::process::exit(1);
    }

    // One timestamp for the whole run, so a run's files group together.
    let timestamp = Utc::now().format("%Y%m%dT%H%M%SZ").to_string();

    let mut succeeded = 0usize;
    let mut failed = 0usize;

    for retailer in RetailerCode::ALL {
        // Skip retailers without a configured storefront (e.g. the generic code).
        if shared::retailers::sitemap_config(retailer).is_none() {
            continue;
        }

        match fetch_and_dump(retailer, &data_dir, &timestamp) {
            Ok((path, bytes)) => {
                println!(
                    "ok   {retailer:?}: wrote {} ({bytes} bytes)",
                    path.display()
                );
                succeeded += 1;
            }
            Err(error) => {
                eprintln!("fail {retailer:?}: {error}");
                failed += 1;
            }
        }
    }

    println!("done: {succeeded} succeeded, {failed} failed");
    if succeeded == 0 {
        std::process::exit(1);
    }
}

/// Fetches one retailer's sitemap and writes it; returns the path and byte size.
fn fetch_and_dump(
    retailer: RetailerCode,
    data_dir: &Path,
    timestamp: &str,
) -> Result<(PathBuf, usize), String> {
    let document = sitemap_discovery::fetch_sitemap(retailer).map_err(|error| error.to_string())?;
    let json = serde_json::to_string_pretty(&document).map_err(|error| error.to_string())?;

    let slug = format!("{retailer:?}").to_lowercase();
    let path = data_dir.join(format!(
        "processed-sitemaps/{slug}-sitemap-{timestamp}.json"
    ));
    fs::write(&path, &json).map_err(|error| format!("writing {}: {error}", path.display()))?;

    Ok((path, json.len()))
}
