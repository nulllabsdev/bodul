//! Typed model of the destructured MinisForum HK product JSON.
//!
//! [`MinisForumHkDestructuredProduct`] is a faithful, strictly-matched mirror of
//! the JSON the `destructure` binary writes to `data/pages-destructed/MinisForumHk/`:
//! every top-level key is modelled, and every struct uses
//! `#[serde(deny_unknown_fields)]` so an unexpected or unmodelled key fails
//! deserialization rather than being silently dropped.
//!
//! The HK store is **SPARSE / JSON-only**: unlike AU there is no Shopify
//! `product` object, no main-product DOM (`xxxx`), no `feature_chart` and no
//! `describe_box`. The blocks that exist on every page are `locale`, `schemas`,
//! `tt_product`, `pixels`, `meta` and `viewed_product` — modelled below.
//!
//! Every scalar leaf is a `String` — the extractor stringifies all values, so
//! prices (`"1000"`), ids and locale codes all arrive as text. Fields that are
//! not present on every page are `Option<String>` (serialized as `null` when
//! absent); lists that may be absent use `#[serde(default)]` (serialized as `[]`).

/// One destructured HK product page.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumHkDestructuredProduct {
    /// Page locale — always "zh-TW" on the HK store.
    pub locale: String,
    /// JSON-LD blocks: a Product schema and a BreadcrumbList schema.
    pub schemas: Vec<MinisForumHkSchema>,
    /// `<script id="tt_product">` summary.
    pub tt_product: MinisForumHkTtProduct,
    /// web-pixels-manager `initData`: shop, page, related products.
    pub pixels: MinisForumHkPixels,
    /// `var meta = {...}`: the product and its variants.
    pub meta: MinisForumHkMeta,
    /// `track("Viewed Product", {...})`: the currently viewed variant.
    pub viewed_product: MinisForumHkViewedProduct,
    /// `<script id="bm_product_variants">`: the Shopify variant array.
    #[serde(default)]
    pub bm_product_variants: Vec<MinisForumHkBmVariant>,
    /// Express theme main product DOM section (title/gallery/price/variants).
    pub express_main: Option<MinisForumHkExpressMain>,
}

/// A variant from the `bm_product_variants` JSON array.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumHkBmVariant {
    pub id: String,
    pub title: String,
    pub available: String,
    pub price: String,
    pub sku: Option<String>,
    pub option1: Option<String>,
    pub option2: Option<String>,
    pub option3: Option<String>,
    pub compare_at_price: Option<String>,
    pub public_title: Option<String>,
}

/// One JSON-LD block — either a Product or a BreadcrumbList, so every field is
/// optional.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumHkSchema {
    pub sku: Option<String>,
    pub product_id: Option<String>,
    pub brand: Option<String>,
    #[serde(default)]
    pub offers: Vec<MinisForumHkOffer>,
    #[serde(default, rename = "itemListElement")]
    pub item_list_element: Vec<MinisForumHkBreadcrumbItem>,
}

/// A Product schema offer.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumHkOffer {
    pub name: String,
    pub price: String,
    pub currency: String,
    pub availability: MinisForumHkAvailability,
    pub price_valid_until: String,
    pub sku: Option<String>,
}

/// Offer stock status, (de)serialized as the schema.org availability URL — the
/// same string in and out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MinisForumHkAvailability {
    #[serde(rename = "https://schema.org/InStock")]
    InStock,
    #[serde(rename = "https://schema.org/OutOfStock")]
    OutOfStock,
}

/// A BreadcrumbList entry. On HK the leaf crumb (the current product) carries no
/// `name`, and the root "Home" crumb carries no `name` either, so `name` is
/// optional; `url` is present on every crumb.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumHkBreadcrumbItem {
    pub name: Option<String>,
    pub url: String,
}

/// `<script id="tt_product">` summary.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumHkTtProduct {
    pub id: String,
    pub title: String,
    pub image_url: String,
}

/// web-pixels-manager `initData`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumHkPixels {
    pub shop_name: String,
    pub currency: String,
    pub country: String,
    pub page_type: String,
    pub product_id: String,
    #[serde(default)]
    pub products: Vec<MinisForumHkPixelsProduct>,
}

/// A related product from the pixels `products` list.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumHkPixelsProduct {
    pub id: String,
    pub handle: String,
    pub title: String,
    pub vendor: String,
    pub url: String,
}

/// `var meta = {...}`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumHkMeta {
    pub id: String,
    pub gid: String,
    pub vendor: String,
    pub handle: String,
    #[serde(rename = "type")]
    pub product_type: Option<String>,
    pub page_type: String,
    pub resource_id: String,
    #[serde(default)]
    pub variants: Vec<MinisForumHkMetaVariant>,
}

/// A variant from the `meta` block.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumHkMetaVariant {
    pub variant_id: String,
    pub price: String,
    pub sku: Option<String>,
    pub title: Option<String>,
}

/// `track("Viewed Product", {...})`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumHkViewedProduct {
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

/// Express theme main product DOM section. Present on HK pages that carry the
/// Express theme `div.shopify-section--main-product` block.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumHkExpressMain {
    pub title: Option<String>,
    #[serde(default)]
    pub price: Option<MinisForumHkExpressPrice>,
    #[serde(default)]
    pub gallery: Option<MinisForumHkExpressGallery>,
    #[serde(default)]
    pub options: Vec<MinisForumHkExpressOption>,
}

/// Express theme price block.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumHkExpressPrice {
    #[serde(default)]
    pub sale_price: Option<String>,
}

/// Express theme gallery.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumHkExpressGallery {
    #[serde(default)]
    pub media: Vec<MinisForumHkExpressMedia>,
}

/// A gallery image from the Express theme DOM.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumHkExpressMedia {
    #[serde(default)]
    pub src: Option<String>,
    #[serde(default)]
    pub alt: Option<String>,
}

/// One variant option group in the Express theme DOM.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumHkExpressOption {
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub selected: Option<String>,
    #[serde(default)]
    pub values: Vec<MinisForumHkExpressOptionValue>,
}

/// A selectable value within an option group.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumHkExpressOptionValue {
    #[serde(default)]
    pub value: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{MinisForumHkAvailability, MinisForumHkDestructuredProduct};

    /// Availability (de)serializes as the schema.org URL — same string in and out.
    #[test]
    fn availability_round_trips() {
        for (url, variant) in [
            (
                "https://schema.org/InStock",
                MinisForumHkAvailability::InStock,
            ),
            (
                "https://schema.org/OutOfStock",
                MinisForumHkAvailability::OutOfStock,
            ),
        ] {
            let from_url: MinisForumHkAvailability =
                serde_json::from_str(&format!("\"{url}\"")).unwrap();
            assert_eq!(from_url, variant);

            let serialized = serde_json::to_string(&from_url).unwrap();
            assert_eq!(serialized, format!("\"{url}\""));
        }
    }

    /// Every destructured HK page deserializes into the strict model. With
    /// `deny_unknown_fields` this also proves the model captures every key.
    #[test]
    #[ignore = "TODO: requires local data/pages-destructed fixtures from a destructure run"]
    fn deserializes_every_hk_page() {
        let dir = std::path::Path::new("data/pages-destructed/MinisForumHk");
        let mut count = 0;
        for entry in std::fs::read_dir(dir)
            .expect("HK destructed dir exists")
            .flatten()
        {
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "json") {
                continue;
            }
            let raw = std::fs::read_to_string(&path).expect("reads file");
            let _product: MinisForumHkDestructuredProduct = serde_json::from_str(&raw)
                .unwrap_or_else(|e| panic!("deserialize {}: {e}", path.display()));
            count += 1;
        }
        assert!(count >= 27, "expected at least 27 HK pages, got {count}");
    }
}
