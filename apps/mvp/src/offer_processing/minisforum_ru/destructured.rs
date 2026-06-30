//! Typed model of the destructured MinisForum RU product JSON.
//!
//! [`MinisForumRuDestructuredProduct`] is a faithful, strictly-matched mirror of
//! the JSON the `destructure` binary writes to `data/pages-destructed/MinisForumRu/`:
//! every top-level key seen in the data is modelled, and every struct uses
//! `#[serde(deny_unknown_fields)]` so an unexpected or unmodelled key fails
//! deserialization rather than being silently dropped.
//!
//! The RU store is **JSON-only / sparse**: there is no Shopify `const product`
//! object, no `<script id="tt_product">` summary, no main-product DOM section
//! (`xxxx`), and no `feature_chart`. The only blocks present are `locale`,
//! `meta`, `pixels`, `viewed_product`, an optional `describe_box` (4/13 files),
//! and `schemas` — which is present on 12 of 13 files and **omitted entirely**
//! on the 13th (`products-minisforum-n5-max-ai-nas.json`). To let that file
//! deserialize, `schemas` is `#[serde(default)]` so the missing key parses as an
//! empty `Vec`.
//!
//! Every scalar leaf is a `String` — the extractor stringifies all values, so
//! prices (`"98700"`), ids and dates all arrive as text. Fields that are not
//! present on every page are `Option<String>`; lists that may be absent use
//! `#[serde(default)]`.

/// One destructured RU product page.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumRuDestructuredProduct {
    /// Page locale, e.g. "en".
    pub locale: String,
    /// JSON-LD blocks — a Product schema carrying `offers`. Present on 12/13
    /// files and omitted entirely on one, so `#[serde(default)]` lets the
    /// schema-less file deserialize as an empty list.
    #[serde(default)]
    pub schemas: Vec<MinisForumRuSchema>,
    /// web-pixels-manager `initData`: shop, page, related products.
    pub pixels: MinisForumRuPixels,
    /// `var meta = {...}`: the product and its variants.
    pub meta: MinisForumRuMeta,
    /// `track("Viewed Product", {...})`: the currently viewed variant.
    pub viewed_product: MinisForumRuViewedProduct,
    /// Product highlights box (not present on every page — 4/13 files).
    pub describe_box: Option<MinisForumRuDescribeBox>,
    /// Custom "Motion" theme main product DOM section (title/gallery/price/variants).
    pub motion_main: Option<MinisForumRuMotionMain>,
    /// `data-variant-json` textarea — variant array (option1/2/3, price, etc.).
    #[serde(default)]
    pub product_variants: Vec<MinisForumRuProductVariant>,
}

/// One JSON-LD block. On RU the schema carries only an optional product `sku`
/// and the offer list (no `brand`/`product_id`, no `BreadcrumbList`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumRuSchema {
    pub sku: Option<String>,
    #[serde(default)]
    pub offers: Vec<MinisForumRuOffer>,
}

/// A Product schema offer. Unlike AU, RU offers carry **no `name`**.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumRuOffer {
    pub price: String,
    pub currency: String,
    pub availability: MinisForumRuAvailability,
    pub price_valid_until: String,
    pub sku: Option<String>,
}

/// Offer stock status, (de)serialized as the schema.org availability URL — the
/// same string in and out. Note RU uses the **`http://`** scheme (AU uses
/// `https://`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MinisForumRuAvailability {
    #[serde(rename = "http://schema.org/InStock")]
    InStock,
    #[serde(rename = "http://schema.org/OutOfStock")]
    OutOfStock,
}

/// web-pixels-manager `initData`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumRuPixels {
    pub shop_name: String,
    pub currency: String,
    pub country: String,
    pub page_type: String,
    pub product_id: String,
    #[serde(default)]
    pub products: Vec<MinisForumRuPixelsProduct>,
}

/// A related product from the pixels `products` list.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumRuPixelsProduct {
    pub id: String,
    pub handle: String,
    pub title: String,
    pub vendor: String,
    pub url: String,
}

/// `var meta = {...}`. RU has **no `type`** key (AU does).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumRuMeta {
    pub id: String,
    pub gid: String,
    pub vendor: String,
    pub handle: String,
    pub page_type: String,
    pub resource_id: String,
    #[serde(default)]
    pub variants: Vec<MinisForumRuMetaVariant>,
}

/// A variant from the `meta` block. Its `price` is a **cents** string (e.g.
/// `"98700"`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumRuMetaVariant {
    pub variant_id: String,
    pub price: String,
    pub sku: Option<String>,
    pub title: Option<String>,
}

/// `track("Viewed Product", {...})`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumRuViewedProduct {
    pub currency: String,
    pub variant_id: String,
    pub product_id: String,
    pub gid: String,
    pub name: String,
    pub price: String,
    pub brand: String,
    pub sku: Option<String>,
    pub variant: Option<String>,
}

/// Product highlights box. RU's variant carries only `text` (no `links`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumRuDescribeBox {
    pub text: String,
}

/// Custom "Motion" theme main product DOM section. Present on pages with the
/// `div.product-section` block.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumRuMotionMain {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub price: Option<MinisForumRuMotionPrice>,
    #[serde(default)]
    pub media: Vec<MinisForumRuMotionMedia>,
    #[serde(default)]
    pub options: Vec<MinisForumRuMotionOption>,
}

/// Motion theme price block.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumRuMotionPrice {
    #[serde(default)]
    pub sale_price: Option<String>,
}

/// A gallery image from the Motion theme DOM.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumRuMotionMedia {
    #[serde(default)]
    pub src: Option<String>,
    #[serde(default)]
    pub alt: Option<String>,
}

/// One variant option group in the Motion theme DOM.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumRuMotionOption {
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub values: Vec<MinisForumRuMotionOptionValue>,
}

/// A selectable value within an option group.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumRuMotionOptionValue {
    #[serde(default)]
    pub value: Option<String>,
}

/// A variant from the `data-variant-json` textarea array. Every field is
/// optional — some variants have null `sku`, null `option3`, etc.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumRuProductVariant {
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
}

#[cfg(test)]
mod tests {
    use super::{MinisForumRuAvailability, MinisForumRuDestructuredProduct};

    /// Availability (de)serializes as the (http) schema.org URL — same string in
    /// and out.
    #[test]
    fn availability_round_trips() {
        for (url, variant) in [
            ("http://schema.org/InStock", MinisForumRuAvailability::InStock),
            ("http://schema.org/OutOfStock", MinisForumRuAvailability::OutOfStock),
        ] {
            let from_url: MinisForumRuAvailability = serde_json::from_str(&format!("\"{url}\"")).unwrap();
            assert_eq!(from_url, variant);

            let serialized = serde_json::to_string(&from_url).unwrap();
            assert_eq!(serialized, format!("\"{url}\""));
        }
    }

    /// Every destructured RU page deserializes into the strict model, including
    /// the one file (`products-minisforum-n5-max-ai-nas.json`) that omits the
    /// `schemas` key entirely. With `deny_unknown_fields` this also proves the
    /// model captures every key.
    #[test]
    #[ignore = "TODO: requires local data/pages-destructed fixtures from a destructure run"]
    fn deserializes_every_ru_page() {
        let dir = std::path::Path::new("data/pages-destructed/MinisForumRu");
        let mut count = 0;
        for entry in std::fs::read_dir(dir).expect("RU destructed dir exists").flatten() {
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "json") {
                continue;
            }
            let raw = std::fs::read_to_string(&path).expect("reads file");
            let _product: MinisForumRuDestructuredProduct =
                serde_json::from_str(&raw).unwrap_or_else(|e| panic!("deserialize {}: {e}", path.display()));
            count += 1;
        }
        assert_eq!(count, 13, "expected 13 RU pages, got {count}");
    }
}
