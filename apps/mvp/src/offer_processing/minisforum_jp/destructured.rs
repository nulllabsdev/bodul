//! Typed model of the destructured MinisForum JP product JSON.
//!
//! [`MinisForumJpDestructuredProduct`] is a faithful, strictly-matched mirror of
//! the JSON the `destructure` binary writes to `data/pages-destructed/MinisForumJp/`:
//! every top-level key is modelled, and every struct uses
//! `#[serde(deny_unknown_fields)]` so an unexpected or unmodelled key fails
//! deserialization rather than being silently dropped.
//!
//! Every scalar leaf is a `String` — the extractor stringifies all values, so
//! prices (`"11999900"`, `"119.999"`), ids and booleans all arrive as text.
//! Fields that are not present on every page are `Option<String>` (serialized as
//! `null` when absent); lists that may be absent use `#[serde(default)]`
//! (serialized as `[]`).
//!
//! ## How JP differs from AU (this store is SPARSE)
//!
//! The JP store has **no product object** — there is no `product`, no
//! `xcotton_pp_variants` and no `tt_product` block, so none of those are modelled
//! here. The blocks that *are* present (and modelled):
//!
//! - universal: `locale`, `schemas`, `meta`, `pixels`, `viewed_product`.
//! - `xxxx` (main product DOM) — present on 66/67 pages, so the top-level field is
//!   `Option`.
//! - `feature_chart` — 33/67 pages (Option).
//! - `describe_box` — 39/67 pages (Option).
//!
//! Per-block differences from AU observed in the data:
//! - `schemas[].offers[]` have **no `name` and no `sku`** (AU offers carry both);
//!   `price_valid_until` is present on 65/66 offers (Option here).
//! - `schemas[]` have **no `product_id`** field.
//! - availability strings use **`http://schema.org/...`** (AU uses `https://`).
//! - `xxxx` has **no `badge`**; `xxxx.variants.options[]` have **no `selected`**.
//! - `feature_chart` has **no `h1`** (only `h2`, on 28/33 charts).
//! - `xxxx.gallery.media[].alt` is optional (one item lacks it).

/// One destructured JP product page.
///
/// SPARSE store: no `product`, `xcotton_pp_variants` or `tt_product` blocks.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumJpDestructuredProduct {
    /// Page locale — always "ja" for this store.
    pub locale: String,
    /// JSON-LD blocks: a Product schema and a BreadcrumbList schema.
    pub schemas: Vec<MinisForumJpSchema>,
    /// `var meta = {...}`: the product and its variants.
    pub meta: MinisForumJpMeta,
    /// web-pixels-manager `initData`: shop, page, related products.
    pub pixels: MinisForumJpPixels,
    /// `track("Viewed Product", {...})`: the currently viewed variant.
    pub viewed_product: MinisForumJpViewedProduct,
    /// `const productVariants = [...]` — variant array from embedded JS
    /// (present on all JP pages).
    #[serde(default)]
    pub product_variants: Vec<MinisForumJpProductVariant>,
    /// The main product DOM section (title/gallery/price/variants). Absent on
    /// 1/67 pages, so `Option`.
    #[serde(rename = "xxxx")]
    pub main_product: Option<MinisForumJpMainProduct>,
    /// Product specification chart (33/67 pages).
    pub feature_chart: Option<MinisForumJpFeatureChart>,
    /// Product highlights / contact note box (39/67 pages).
    pub describe_box: Option<MinisForumJpDescribeBox>,
}

/// One JSON-LD block — either a Product or a BreadcrumbList, so every field is
/// optional. Note: unlike AU there is no `product_id` field on JP schemas.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumJpSchema {
    pub sku: Option<String>,
    pub brand: Option<String>,
    #[serde(default)]
    pub offers: Vec<MinisForumJpOffer>,
    #[serde(default, rename = "itemListElement")]
    pub item_list_element: Vec<MinisForumJpBreadcrumbItem>,
}

/// A Product schema offer.
///
/// JP offers carry **no `name` and no `sku`** (the owning schema's `sku` is the
/// product's sku). `price` is a major-unit (yen) string using a period as a
/// thousands separator (e.g. `"119.999"` = ¥119,999; `"1000"` = ¥1,000).
/// `price_valid_until` is present on 65/66 offers.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumJpOffer {
    pub price: String,
    pub currency: String,
    pub availability: MinisForumJpAvailability,
    pub price_valid_until: Option<String>,
}

/// Offer stock status, (de)serialized as the schema.org availability URL — the
/// same string in and out. JP uses the **`http://`** scheme (AU uses `https://`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MinisForumJpAvailability {
    #[serde(rename = "http://schema.org/InStock")]
    InStock,
    #[serde(rename = "http://schema.org/OutOfStock")]
    OutOfStock,
}

/// A BreadcrumbList entry.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumJpBreadcrumbItem {
    pub name: String,
    pub url: String,
}

/// `var meta = {...}`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumJpMeta {
    pub id: String,
    pub gid: String,
    pub vendor: String,
    pub handle: String,
    #[serde(rename = "type")]
    pub product_type: Option<String>,
    pub page_type: String,
    pub resource_id: String,
    #[serde(default)]
    pub variants: Vec<MinisForumJpMetaVariant>,
}

/// A variant from the `meta` block. Prices are **cents** strings (e.g.
/// `"11999900"` = ¥119,999.00). `sku` is null on 12/265 variants; `title` is
/// present on all but one.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumJpMetaVariant {
    pub variant_id: String,
    pub price: String,
    pub sku: Option<String>,
    pub title: Option<String>,
}

/// web-pixels-manager `initData`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumJpPixels {
    pub shop_name: String,
    pub currency: String,
    pub country: String,
    pub page_type: String,
    pub product_id: String,
    #[serde(default)]
    pub products: Vec<MinisForumJpPixelsProduct>,
}

/// A related product from the pixels `products` list.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumJpPixelsProduct {
    pub id: String,
    pub handle: String,
    pub title: String,
    pub vendor: String,
    pub url: String,
}

/// `track("Viewed Product", {...})`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumJpViewedProduct {
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

/// The main product DOM section (the `xxxx` key). Unlike AU there is no `badge`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumJpMainProduct {
    pub title: String,
    pub gallery: MinisForumJpGallery,
    pub price: MinisForumJpPrice,
    pub variants: Option<MinisForumJpMainVariants>,
}

/// The gallery within the main product section.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumJpGallery {
    #[serde(default)]
    pub media: Vec<MinisForumJpGalleryMedia>,
}

/// A gallery image. `alt` is optional (one item in the data lacks it).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumJpGalleryMedia {
    pub src: String,
    pub alt: Option<String>,
}

/// The price block within the main product section.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumJpPrice {
    pub sale_price: String,
    pub savings: Option<String>,
    pub compare_at_price: Option<String>,
}

/// The variant picker within the main product section.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumJpMainVariants {
    #[serde(default)]
    pub options: Vec<MinisForumJpVariantOption>,
}

/// One option group of the variant picker. Unlike AU there is no `selected`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumJpVariantOption {
    pub label: String,
    #[serde(default)]
    pub values: Vec<MinisForumJpVariantValue>,
}

/// One selectable value of an option group.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumJpVariantValue {
    pub value: String,
}

/// Product specification chart, column-major (`features[column][row]`). Unlike
/// AU there is no `h1`; `h2` is present on 28/33 charts.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumJpFeatureChart {
    #[serde(default)]
    pub features: Vec<Vec<MinisForumJpFeature>>,
    pub h2: Option<String>,
}

/// One spec cell.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumJpFeature {
    pub label: String,
    pub value: String,
}

/// Product highlights / contact note box.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumJpDescribeBox {
    pub text: String,
    #[serde(default)]
    pub links: Vec<MinisForumJpLink>,
}

/// A link within the describe box.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumJpLink {
    pub href: String,
}

/// A variant from the `const productVariants = [...]` JS array. Every field is
/// optional — some pages have `featured_image: null` (so `image_src` is absent),
/// and some variants lack SKU or have null `option3`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumJpProductVariant {
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

#[cfg(test)]
mod tests {
    use super::{MinisForumJpAvailability, MinisForumJpDestructuredProduct};

    /// Availability (de)serializes as the schema.org URL — same string in and
    /// out. JP uses the `http://` scheme.
    #[test]
    fn availability_round_trips() {
        for (url, variant) in [
            ("http://schema.org/InStock", MinisForumJpAvailability::InStock),
            ("http://schema.org/OutOfStock", MinisForumJpAvailability::OutOfStock),
        ] {
            let from_url: MinisForumJpAvailability = serde_json::from_str(&format!("\"{url}\"")).unwrap();
            assert_eq!(from_url, variant);

            let serialized = serde_json::to_string(&from_url).unwrap();
            assert_eq!(serialized, format!("\"{url}\""));
        }
    }

    /// Every destructured JP page deserializes into the strict model. With
    /// `deny_unknown_fields` this also proves the model captures every key.
    #[test]
    #[ignore = "TODO: requires local data/pages-destructed fixtures from a destructure run"]
    fn deserializes_every_jp_page() {
        let dir = std::path::Path::new("data/pages-destructed/MinisForumJp");
        let mut count = 0;
        for entry in std::fs::read_dir(dir).expect("JP destructed dir exists").flatten() {
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "json") {
                continue;
            }
            let raw = std::fs::read_to_string(&path).expect("reads file");
            let _product: MinisForumJpDestructuredProduct =
                serde_json::from_str(&raw).unwrap_or_else(|e| panic!("deserialize {}: {e}", path.display()));
            count += 1;
        }
        assert!(count >= 67, "expected at least 67 JP pages, got {count}");
    }
}
