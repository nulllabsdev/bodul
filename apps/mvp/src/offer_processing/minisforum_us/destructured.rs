//! Typed model of the destructured MinisForum US product JSON.
//!
//! [`MinisForumUsDestructuredProduct`] is a faithful, strictly-matched mirror of the JSON the
//! `destructure` binary writes to `data/pages-destructed/MinisForumUs/`: every
//! top-level key is modelled, and every struct uses `#[serde(deny_unknown_fields)]`
//! so an unexpected or unmodelled key fails deserialization rather than being
//! silently dropped.
//!
//! Every scalar leaf is a `String` — the extractor stringifies all values, so
//! prices (`"5629"`), ids and booleans (`"true"`) all arrive as text. Fields that
//! are not present on every page are `Option<String>` (serialized as `null` when
//! absent); lists that may be absent use `#[serde(default)]` (serialized as `[]`).
//!
//! ## How the US data differs from AU (see also `TS002_product-processing.md`)
//!
//! - **No main-product DOM (`xxxx`)** → no gallery images block, and no
//!   `describe_box`. **No `tt_product`.** These keys are absent from every US
//!   page and are therefore not modelled here.
//! - **Both `product` and `xcotton_pp_variants` are present** on every page. They
//!   carry the same Shopify product shape; `product` is the canonical source (as
//!   in AU) but the strict model mirrors both keys.
//! - **`feature_chart`** is present on 128/198 pages (modelled as `Option`).
//! - **JSON-LD offers carry no `name` and no `sku`** (unlike AU), use the
//!   `http://schema.org/...` availability scheme (note: `http`, not `https`), and
//!   their `currency` may be `USD` or `CAD`. The schema-level `sku`/`offers` are
//!   both optional and there are no breadcrumb (`itemListElement`) schemas.

/// One destructured US product page.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUsDestructuredProduct {
    /// Page locale, e.g. "en" or "es".
    pub locale: String,
    /// JSON-LD blocks: a Product schema and (usually) a BreadcrumbList-less
    /// second Product/brand schema.
    pub schemas: Vec<MinisForumUsSchema>,
    /// Full Shopify product object (`const product = {...}`) — the canonical
    /// source.
    pub product: MinisForumUsProduct,
    /// `<script id="xcotton_pp_variants">` — the same Shopify product shape as
    /// `product`; mirrored for fidelity but not used as the canonical source.
    pub xcotton_pp_variants: MinisForumUsProduct,
    /// web-pixels-manager `initData`: shop, page, related products.
    pub pixels: MinisForumUsPixels,
    /// `var meta = {...}`: the product and its variants.
    pub meta: MinisForumUsMeta,
    /// `track("Viewed Product", {...})`: the currently viewed variant.
    pub viewed_product: MinisForumUsViewedProduct,
    /// Product specification chart (not present on every page).
    pub feature_chart: Option<MinisForumUsFeatureChart>,
}

/// One JSON-LD block — a Product schema (`brand`, optional `sku`, optional
/// `offers`). The US store has no breadcrumb (`itemListElement`) schemas, so
/// every field but `brand` is optional.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUsSchema {
    pub brand: String,
    pub sku: Option<String>,
    #[serde(default)]
    pub offers: Vec<MinisForumUsOffer>,
}

/// A Product schema offer. Unlike AU, US offers carry **no `name` and no `sku`**;
/// `price_valid_until` is not always present.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUsOffer {
    pub availability: MinisForumUsAvailability,
    pub currency: String,
    pub price: String,
    pub price_valid_until: Option<String>,
}

/// Offer stock status, (de)serialized as the schema.org availability URL — the
/// same string in and out. US uses the `http://` scheme (AU uses `https://`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MinisForumUsAvailability {
    #[serde(rename = "http://schema.org/InStock")]
    InStock,
    #[serde(rename = "http://schema.org/OutOfStock")]
    OutOfStock,
}

/// The full Shopify product object. Used both for the `product` key and for
/// `xcotton_pp_variants`, which share this exact shape.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUsProduct {
    pub id: String,
    pub title: String,
    pub handle: String,
    pub vendor: String,
    #[serde(rename = "type")]
    pub product_type: Option<String>,
    pub available: String,
    pub price: String,
    pub price_min: String,
    pub price_max: String,
    pub compare_at_price: Option<String>,
    #[serde(default)]
    pub variants: Vec<MinisForumUsProductVariant>,
    #[serde(default)]
    pub media: Vec<MinisForumUsProductMedia>,
}

/// A variant of the full product object.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUsProductVariant {
    pub available: String,
    pub price: String,
    pub option1: String,
    pub option2: Option<String>,
    pub option3: Option<String>,
    pub sku: Option<String>,
    pub compare_at_price: Option<String>,
}

/// A media item of the full product object.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUsProductMedia {
    // `external_video` media items carry only `type` (no src/dimensions).
    pub src: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
    #[serde(rename = "type")]
    pub media_type: String,
}

/// web-pixels-manager `initData`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUsPixels {
    pub shop_name: String,
    pub currency: String,
    pub country: String,
    pub page_type: String,
    pub product_id: String,
    #[serde(default)]
    pub products: Vec<MinisForumUsPixelsProduct>,
}

/// A related product from the pixels `products` list.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUsPixelsProduct {
    pub id: String,
    pub handle: String,
    pub title: String,
    pub vendor: String,
    pub url: String,
}

/// `var meta = {...}`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUsMeta {
    pub id: String,
    pub gid: String,
    pub vendor: String,
    pub handle: String,
    #[serde(rename = "type")]
    pub product_type: Option<String>,
    pub page_type: String,
    pub resource_id: String,
    #[serde(default)]
    pub variants: Vec<MinisForumUsMetaVariant>,
}

/// A variant from the `meta` block.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUsMetaVariant {
    pub variant_id: String,
    pub price: String,
    pub sku: Option<String>,
    pub title: Option<String>,
}

/// `track("Viewed Product", {...})`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUsViewedProduct {
    pub currency: String,
    pub variant_id: String,
    pub product_id: String,
    pub gid: String,
    pub name: String,
    pub price: String,
    pub brand: String,
    pub sku: Option<String>,
    pub variant: Option<String>,
    pub category: Option<String>,
}

/// Product specification chart, column-major (`features[column][row]`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUsFeatureChart {
    #[serde(default)]
    pub features: Vec<Vec<MinisForumUsFeature>>,
    pub h1: Option<String>,
    pub h2: Option<String>,
}

/// One spec cell.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUsFeature {
    pub label: String,
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::{MinisForumUsAvailability, MinisForumUsDestructuredProduct};

    /// Availability (de)serializes as the schema.org URL — same string in and
    /// out. US uses the `http://` scheme.
    #[test]
    fn availability_round_trips() {
        for (url, variant) in [
            (
                "http://schema.org/InStock",
                MinisForumUsAvailability::InStock,
            ),
            (
                "http://schema.org/OutOfStock",
                MinisForumUsAvailability::OutOfStock,
            ),
        ] {
            let from_url: MinisForumUsAvailability =
                serde_json::from_str(&format!("\"{url}\"")).unwrap();
            assert_eq!(from_url, variant);

            let serialized = serde_json::to_string(&from_url).unwrap();
            assert_eq!(serialized, format!("\"{url}\""));
        }
    }

    /// Every destructured US page deserializes into the strict model. With
    /// `deny_unknown_fields` this also proves the model captures every key.
    #[test]
    #[ignore = "TODO: requires local data/pages-destructed fixtures from a destructure run"]
    fn deserializes_every_us_page() {
        let dir = std::path::Path::new("data/pages-destructed/MinisForumUs");
        let mut count = 0;
        for entry in std::fs::read_dir(dir)
            .expect("US destructed dir exists")
            .flatten()
        {
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "json") {
                continue;
            }
            let raw = std::fs::read_to_string(&path).expect("reads file");
            let _product: MinisForumUsDestructuredProduct = serde_json::from_str(&raw)
                .unwrap_or_else(|e| panic!("deserialize {}: {e}", path.display()));
            count += 1;
        }
        assert!(count >= 198, "expected at least 198 US pages, got {count}");
    }
}
