//! Classifies the links in every sitemap dump under `data/` by type and writes
//! each result, grouped, to `data/detected/` as pretty JSON. A failure on one
//! file does not stop the others.

use std::fs;
use std::path::{Path, PathBuf};

use mvp::sitemap_discovery::link::LinkKind;
use mvp::sitemap_discovery::sitemap::SitemapDocument;
use serde::Serialize;

#[derive(Serialize, Default)]
struct Group {
    product: Vec<String>,
    catalog: Vec<String>,
    content: Vec<String>,
    unknown: Vec<String>,
}

#[derive(Serialize, Clone, Copy)]
struct Counts {
    product: usize,
    catalog: usize,
    content: usize,
    unknown: usize,
}

#[derive(Serialize)]
struct Detection {
    source: String,
    counts: Counts,
    links: Group,
}

fn main() {
    let data_dir = PathBuf::from("data");
    let out_dir = data_dir.join("detected");
    if let Err(error) = fs::create_dir_all(&out_dir) {
        eprintln!("error creating {}: {error}", out_dir.display());
        std::process::exit(1);
    }

    let mut sources = match sitemap_dumps(&data_dir) {
        Ok(sources) => sources,
        Err(error) => {
            eprintln!("error reading {}/: {error}", data_dir.display());
            std::process::exit(1);
        }
    };
    sources.sort();

    if sources.is_empty() {
        eprintln!("no sitemap dumps found in {}/", data_dir.display());
        std::process::exit(1);
    }

    let mut succeeded = 0usize;
    let mut failed = 0usize;

    for source in &sources {
        match detect_file(source, &out_dir) {
            Ok((out_path, counts)) => {
                println!(
                    "ok   {} -> {} (product {}, catalog {}, content {}, unknown {})",
                    source.display(),
                    out_path.display(),
                    counts.product,
                    counts.catalog,
                    counts.content,
                    counts.unknown,
                );
                succeeded += 1;
            }
            Err(error) => {
                eprintln!("fail {}: {error}", source.display());
                failed += 1;
            }
        }
    }

    println!("done: {succeeded} succeeded, {failed} failed");
    if succeeded == 0 {
        std::process::exit(1);
    }
}

/// Reads one sitemap dump, classifies its links, and writes the grouped result.
fn detect_file(source: &Path, out_dir: &Path) -> Result<(PathBuf, Counts), String> {
    let json = fs::read_to_string(source).map_err(|error| error.to_string())?;
    let document: SitemapDocument =
        serde_json::from_str(&json).map_err(|error| error.to_string())?;

    let mut links = Group::default();
    for url in document.all_urls() {
        match LinkKind::from_location(&url.location) {
            LinkKind::Product => links.product.push(url.location.clone()),
            LinkKind::Catalog => links.catalog.push(url.location.clone()),
            LinkKind::Content => links.content.push(url.location.clone()),
            LinkKind::Unknown => links.unknown.push(url.location.clone()),
        }
    }

    let counts = Counts {
        product: links.product.len(),
        catalog: links.catalog.len(),
        content: links.content.len(),
        unknown: links.unknown.len(),
    };

    let stem = source
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("sitemap");
    let out_path = out_dir.join(format!("{stem}-detected.json"));

    let detection = Detection {
        source: source.display().to_string(),
        counts,
        links,
    };
    let out = serde_json::to_string_pretty(&detection).map_err(|error| error.to_string())?;
    fs::write(&out_path, &out)
        .map_err(|error| format!("writing {}: {error}", out_path.display()))?;

    Ok((out_path, counts))
}

/// All sitemap dump files directly under `dir` (skipping any `-detected` output).
fn sitemap_dumps(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut dumps = Vec::new();
    for entry in fs::read_dir(dir)
        .map_err(|error| error.to_string())?
        .flatten()
    {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if name.ends_with(".json") && name.contains("-sitemap") && !name.contains("-detected") {
            dumps.push(path);
        }
    }
    Ok(dumps)
}
