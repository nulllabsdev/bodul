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

use ::retailer_sourcing::retailers::minisforum::{
    au as minisforumau, ca as minisforumca, eu as minisforumeu, fr as minisforumfr, hk as minisforumhk,
    jp as minisforumjp, kr as minisforumkr, ru as minisforumru, uk as minisforumuk, us as minisforumus,
};

fn main() {
    dotenvy::dotenv().ok();
    let _guard = mvp::logging::init();
    let input_root = PathBuf::from("data/pages-destructed");
    let output_root = PathBuf::from("data/pages-processed");

    let retailers = match fs::read_dir(&input_root) {
        Ok(entries) => entries,
        Err(error) => {
            tracing::error!("error reading {}/: {error}", input_root.display());
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
            tracing::error!("error creating {}: {error}", output_dir.display());
            std::process::exit(1);
        }

        let entries = match fs::read_dir(&retailer_dir) {
            Ok(entries) => entries,
            Err(error) => {
                tracing::error!("error reading {}/: {error}", retailer_dir.display());
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
                    tracing::error!("fail {}: {error}", path.display());
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
        let destructured: $dest =
            serde_json::from_str($raw).map_err(|error| format!("parsing {}: {error}", $path.display()))?;
        let processed =
            <$proc>::try_from(destructured).map_err(|error| format!("processing {}: {error}", $path.display()))?;
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
            minisforumau::prelude::MinisForumAuDestructuredProduct,
            minisforumau::prelude::MinisForumAuProcessedProduct
        ),
        "MinisForumCa" => process_typed!(
            &raw,
            path,
            minisforumca::prelude::MinisForumCaDestructuredProduct,
            minisforumca::prelude::MinisForumCaProcessedProduct
        ),
        "MinisForumEu" => process_typed!(
            &raw,
            path,
            minisforumeu::prelude::MinisForumEuDestructuredProduct,
            minisforumeu::prelude::MinisForumEuProcessedProduct
        ),
        "MinisForumFr" => process_typed!(
            &raw,
            path,
            minisforumfr::prelude::MinisForumFrDestructuredProduct,
            minisforumfr::prelude::MinisForumFrProcessedProduct
        ),
        "MinisForumHk" => process_typed!(
            &raw,
            path,
            minisforumhk::prelude::MinisForumHkDestructuredProduct,
            minisforumhk::prelude::MinisForumHkProcessedProduct
        ),
        "MinisForumJp" => process_typed!(
            &raw,
            path,
            minisforumjp::prelude::MinisForumJpDestructuredProduct,
            minisforumjp::prelude::MinisForumJpProcessedProduct
        ),
        "MinisForumKr" => process_typed!(
            &raw,
            path,
            minisforumkr::prelude::MinisForumKrDestructuredProduct,
            minisforumkr::prelude::MinisForumKrProcessedProduct
        ),
        "MinisForumRu" => process_typed!(
            &raw,
            path,
            minisforumru::prelude::MinisForumRuDestructuredProduct,
            minisforumru::prelude::MinisForumRuProcessedProduct
        ),
        "MinisForumUk" => process_typed!(
            &raw,
            path,
            minisforumuk::prelude::MinisForumUkDestructuredProduct,
            minisforumuk::prelude::MinisForumUkProcessedProduct
        ),
        "MinisForumUs" => process_typed!(
            &raw,
            path,
            minisforumus::prelude::MinisForumUsDestructuredProduct,
            minisforumus::prelude::MinisForumUsProcessedProduct
        ),
        _ => {
            let value: serde_json::Value =
                serde_json::from_str(&raw).map_err(|error| format!("parsing {}: {error}", path.display()))?;
            serde_json::to_string_pretty(&value).map_err(|error| error.to_string())?
        }
    };

    let file_name = path.file_name().unwrap_or_else(|| "page.json".as_ref());
    let out_path = output_dir.join(file_name);
    fs::write(&out_path, json).map_err(|error| format!("writing {}: {error}", out_path.display()))?;

    Ok(out_path)
}
