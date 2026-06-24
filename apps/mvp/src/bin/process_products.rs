//! Processes destructured product JSON into processed product JSON.
//!
//! It reads every `*.json` under `data/pages-destructed/{Retailer}/` and writes a
//! processed copy into `data/pages-processed/{Retailer}/`, mirroring the directory
//! layout. For now processing is the identity (output = input); this is the seam
//! where normalization/enrichment will live.
//!
//! Each retailer is routed through its own typed model (a strict mirror of its
//! destructured JSON → a normalized processed product). A retailer dir without a
//! model falls back to untyped `serde_json::Value` passthrough.

use std::fs;
use std::path::{Path, PathBuf};

use mvp::offer_processing::{
    minisforum_au, minisforum_ca, minisforum_eu, minisforum_fr, minisforum_hk, minisforum_jp,
    minisforum_kr, minisforum_ru, minisforum_uk, minisforum_us,
};

fn main() {
    let input_root = PathBuf::from("data/pages-destructed");
    let output_root = PathBuf::from("data/pages-processed");

    let retailers = match fs::read_dir(&input_root) {
        Ok(entries) => entries,
        Err(error) => {
            eprintln!("error reading {}/: {error}", input_root.display());
            std::process::exit(1);
        }
    };

    let mut processed = 0usize;
    let mut failed = 0usize;

    for retailer in retailers.flatten() {
        let retailer_dir = retailer.path();
        if !retailer_dir.is_dir() {
            continue;
        }
        let retailer_name = retailer.file_name();
        let retailer_name = retailer_name.to_string_lossy();
        let output_dir = output_root.join(retailer.file_name());

        if let Err(error) = fs::create_dir_all(&output_dir) {
            eprintln!("error creating {}: {error}", output_dir.display());
            std::process::exit(1);
        }

        let entries = match fs::read_dir(&retailer_dir) {
            Ok(entries) => entries,
            Err(error) => {
                eprintln!("error reading {}/: {error}", retailer_dir.display());
                failed += 1;
                continue;
            }
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() || path.extension().is_none_or(|ext| ext != "json") {
                continue;
            }

            match process_file(&path, &output_dir, &retailer_name) {
                Ok(out_path) => {
                    println!("ok   {} -> {}", path.display(), out_path.display());
                    processed += 1;
                }
                Err(error) => {
                    eprintln!("fail {}: {error}", path.display());
                    failed += 1;
                }
            }
        }
    }

    println!("done: {processed} processed, {failed} failed");
    if processed == 0 {
        std::process::exit(1);
    }
}

/// Deserializes `raw` into a retailer's destructured model, converts it to the
/// processed model, and returns the pretty JSON. Errors carry `path` context.
macro_rules! process_typed {
    ($raw:expr, $path:expr, $dest:ty, $proc:ty) => {{
        let destructured: $dest = serde_json::from_str($raw)
            .map_err(|error| format!("parsing {}: {error}", $path.display()))?;
        let processed = <$proc>::try_from(destructured)
            .map_err(|error| format!("processing {}: {error}", $path.display()))?;
        serde_json::to_string_pretty(&processed).map_err(|error| error.to_string())?
    }};
}

/// Reads one destructured JSON, processes it through the retailer's typed model,
/// and writes the result. Retailers without a model fall back to `Value`.
fn process_file(path: &Path, output_dir: &Path, retailer: &str) -> Result<PathBuf, String> {
    let raw = fs::read_to_string(path).map_err(|error| error.to_string())?;

    let json = match retailer {
        "MinisForumAu" => process_typed!(
            &raw,
            path,
            minisforum_au::MinisForumAuDestructuredProduct,
            minisforum_au::MinisForumAuProcessedProduct
        ),
        "MinisForumCa" => process_typed!(
            &raw,
            path,
            minisforum_ca::MinisForumCaDestructuredProduct,
            minisforum_ca::MinisForumCaProcessedProduct
        ),
        "MinisForumEu" => process_typed!(
            &raw,
            path,
            minisforum_eu::MinisForumEuDestructuredProduct,
            minisforum_eu::MinisForumEuProcessedProduct
        ),
        "MinisForumFr" => process_typed!(
            &raw,
            path,
            minisforum_fr::MinisForumFrDestructuredProduct,
            minisforum_fr::MinisForumFrProcessedProduct
        ),
        "MinisForumHk" => process_typed!(
            &raw,
            path,
            minisforum_hk::MinisForumHkDestructuredProduct,
            minisforum_hk::MinisForumHkProcessedProduct
        ),
        "MinisForumJp" => process_typed!(
            &raw,
            path,
            minisforum_jp::MinisForumJpDestructuredProduct,
            minisforum_jp::MinisForumJpProcessedProduct
        ),
        "MinisForumKr" => process_typed!(
            &raw,
            path,
            minisforum_kr::MinisForumKrDestructuredProduct,
            minisforum_kr::MinisForumKrProcessedProduct
        ),
        "MinisForumRu" => process_typed!(
            &raw,
            path,
            minisforum_ru::MinisForumRuDestructuredProduct,
            minisforum_ru::MinisForumRuProcessedProduct
        ),
        "MinisForumUk" => process_typed!(
            &raw,
            path,
            minisforum_uk::MinisForumUkDestructuredProduct,
            minisforum_uk::MinisForumUkProcessedProduct
        ),
        "MinisForumUs" => process_typed!(
            &raw,
            path,
            minisforum_us::MinisForumUsDestructuredProduct,
            minisforum_us::MinisForumUsProcessedProduct
        ),
        _ => {
            let value: serde_json::Value = serde_json::from_str(&raw)
                .map_err(|error| format!("parsing {}: {error}", path.display()))?;
            serde_json::to_string_pretty(&value).map_err(|error| error.to_string())?
        }
    };

    let file_name = path.file_name().unwrap_or_else(|| "page.json".as_ref());
    let out_path = output_dir.join(file_name);
    fs::write(&out_path, json)
        .map_err(|error| format!("writing {}: {error}", out_path.display()))?;

    Ok(out_path)
}
