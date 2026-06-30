//! Typed model of the destructured MinisForum KR product JSON.
//!
//! [`MinisForumKrDestructuredProduct`] is a faithful, strictly-matched mirror of
//! the JSON the `destructure` binary writes to
//! `data/pages-destructed/MinisForumKr/`: every top-level key is modelled, and
//! every struct uses `#[serde(deny_unknown_fields)]` so an unexpected or
//! unmodelled key fails deserialization rather than being silently dropped.
//!
//! The KR store is **JSON-only / SPARSE**: there is no `const product` object, no
//! `tt_product`, no `xxxx` main-product DOM section, and no `feature_chart`. Only
//! the schema/meta/pixels/analytics blocks survive, so this model covers exactly
//! the six blocks present in the data: `locale`, `schemas`, `meta`, `pixels`,
//! `viewed_product` and `describe_box`.
//!
//! Every scalar leaf is a `String` — the extractor stringifies all values, so
//! prices (`"75800"`), ids and booleans all arrive as text. Fields that are not
//! present on every page are `Option<String>` (serialized as `null` when absent);
//! lists that may be absent use `#[serde(default)]` (serialized as `[]`).

/// One destructured KR product page.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumKrDestructuredProduct {
    /// Page locale — always "ko" in the KR store.
    pub locale: String,
    /// JSON-LD blocks. Usually a single Product schema carrying only `brand`;
    /// occasionally a second schema, and rarely one with `sku`/`offers`.
    pub schemas: Vec<MinisForumKrSchema>,
    /// `var meta = {...}`: the product and its analytics variants.
    pub meta: MinisForumKrMeta,
    /// web-pixels-manager `initData`: shop, page, related products.
    pub pixels: MinisForumKrPixels,
    /// `track("Viewed Product", {...})`: the currently viewed variant.
    pub viewed_product: MinisForumKrViewedProduct,
    /// `const productVariants = [...]` — variant array from embedded JS
    /// (present on all KR pages).
    #[serde(default)]
    pub product_variants: Vec<MinisForumKrProductVariant>,
    /// Dawn theme main product DOM section (title/gallery/price/variants).
    pub dawn_main: Option<MinisForumKrDawnMain>,
    /// Product highlights box (only `text`; present on 8/16 pages).
    pub describe_box: Option<MinisForumKrDescribeBox>,
}

/// One JSON-LD block. In the KR data a schema almost always carries only `brand`;
/// `sku` and `offers` appear on a small minority of pages, so every field is
/// optional.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumKrSchema {
    pub brand: Option<String>,
    pub sku: Option<String>,
    #[serde(default)]
    pub offers: Vec<MinisForumKrOffer>,
}

/// A Product schema offer. In the KR data `price_valid_until` is frequently
/// absent, so it is optional.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumKrOffer {
    pub price: String,
    pub currency: String,
    pub availability: MinisForumKrAvailability,
    pub price_valid_until: Option<String>,
}

/// Offer stock status, (de)serialized as the schema.org availability URL — the
/// same string in and out. The KR data uses the `http://` (not `https://`) form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MinisForumKrAvailability {
    #[serde(rename = "http://schema.org/InStock")]
    InStock,
    #[serde(rename = "http://schema.org/OutOfStock")]
    OutOfStock,
}

/// `var meta = {...}`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumKrMeta {
    pub id: String,
    pub gid: String,
    pub vendor: String,
    pub handle: String,
    /// Product category — absent on a few pages (e.g. accessories).
    #[serde(rename = "type")]
    pub product_type: Option<String>,
    pub page_type: String,
    pub resource_id: String,
    #[serde(default)]
    pub variants: Vec<MinisForumKrMetaVariant>,
}

/// A variant from the `meta` block. SKUs are **not unique** within a page (the
/// same SKU can appear under several analytics `variant_id`s, occasionally with
/// differing prices).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumKrMetaVariant {
    pub variant_id: String,
    pub price: String,
    pub sku: Option<String>,
    pub title: Option<String>,
}

/// web-pixels-manager `initData`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumKrPixels {
    pub shop_name: String,
    pub currency: String,
    pub country: String,
    pub page_type: String,
    pub product_id: String,
    #[serde(default)]
    pub products: Vec<MinisForumKrPixelsProduct>,
}

/// A related product from the pixels `products` list.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumKrPixelsProduct {
    pub id: String,
    pub handle: String,
    pub title: String,
    pub vendor: String,
    pub url: String,
}

/// `track("Viewed Product", {...})`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumKrViewedProduct {
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

/// Product highlights box. In the KR data it only ever carries `text`; the
/// `links` list is modelled (with `#[serde(default)]`) to stay aligned with the
/// AU shape, but is always empty here.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumKrDescribeBox {
    pub text: String,
    #[serde(default)]
    pub links: Vec<MinisForumKrLink>,
}

/// A link within the describe box.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumKrLink {
    pub href: String,
}

/// A variant from the `const productVariants = [...]` JS array. Every field is
/// optional — some variants have `featured_image: null`, null `option3`, etc.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumKrProductVariant {
    pub id: String,
    #[serde(default)]
    pub sku: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub option1: Option<String>,
    #[serde(default)]
    pub option2: Option<String>,
    #[serde(default)]
    pub option3: Option<String>,
    #[serde(default)]
    pub available: Option<String>,
    #[serde(default)]
    pub price: Option<String>,
    #[serde(default)]
    pub compare_at_price: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub image_src: Option<String>,
}

/// Dawn theme main product DOM section. Present on pages that carry the Dawn
/// theme `section.product-section` block.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumKrDawnMain {
    pub title: String,
    #[serde(default)]
    pub price: Option<MinisForumKrDawnPrice>,
    #[serde(default)]
    pub media: Vec<MinisForumKrDawnMedia>,
    #[serde(default)]
    pub options: Vec<MinisForumKrDawnOption>,
}

/// Dawn theme price block (sale / compare-at).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumKrDawnPrice {
    #[serde(default)]
    pub sale_price: Option<String>,
    #[serde(default)]
    pub compare_at_price: Option<String>,
}

/// A gallery image from the Dawn theme DOM.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumKrDawnMedia {
    #[serde(default)]
    pub src: Option<String>,
    #[serde(default)]
    pub alt: Option<String>,
}

/// One variant option group in the Dawn theme DOM.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumKrDawnOption {
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub values: Vec<MinisForumKrDawnOptionValue>,
}

/// A selectable value within an option group.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumKrDawnOptionValue {
    #[serde(default)]
    pub value: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{MinisForumKrAvailability, MinisForumKrDestructuredProduct};

    /// Availability (de)serializes as the schema.org URL — same string in and
    /// out. The KR data uses the `http://` form.
    #[test]
    fn availability_round_trips() {
        for (url, variant) in [
            ("http://schema.org/InStock", MinisForumKrAvailability::InStock),
            ("http://schema.org/OutOfStock", MinisForumKrAvailability::OutOfStock),
        ] {
            let from_url: MinisForumKrAvailability = serde_json::from_str(&format!("\"{url}\"")).unwrap();
            assert_eq!(from_url, variant);

            let serialized = serde_json::to_string(&from_url).unwrap();
            assert_eq!(serialized, format!("\"{url}\""));
        }
    }

    /// Every destructured KR page deserializes into the strict model. With
    /// `deny_unknown_fields` this also proves the model captures every key.
    #[test]
    #[ignore = "TODO: requires local data/pages-destructed fixtures from a destructure run"]
    fn deserializes_every_kr_page() {
        let dir = std::path::Path::new("data/pages-destructed/MinisForumKr");
        let mut count = 0;
        for entry in std::fs::read_dir(dir).expect("KR destructed dir exists").flatten() {
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "json") {
                continue;
            }
            let raw = std::fs::read_to_string(&path).expect("reads file");
            let _product: MinisForumKrDestructuredProduct =
                serde_json::from_str(&raw).unwrap_or_else(|e| panic!("deserialize {}: {e}", path.display()));
            count += 1;
        }
        assert!(count >= 16, "expected at least 16 KR pages, got {count}");
    }
}
