//! Typed model of the destructured MinisForum EU product JSON.
//!
//! [`MinisForumEuDestructuredProduct`] is a faithful, strictly-matched mirror of the JSON the
//! `destructure` binary writes to `data/offers-destructed/MinisForumEu/`: every
//! top-level key is modelled, and every struct uses `#[serde(deny_unknown_fields)]`
//! so an unexpected or unmodelled key fails deserialization rather than being
//! silently dropped.
//!
//! Every scalar leaf is a `String` — the extractor stringifies all values, so
//! prices (`"2590"`), ids and booleans (`"true"`) all arrive as text. Fields that
//! are not present on every page are `Option<String>` (serialized as `null` when
//! absent); lists that may be absent use `#[serde(default)]` (serialized as `[]`).
//!
//! Shape differences from the AU mirror:
//! - There is **no** `const product` and **no** `tt_product`. The canonical
//!   product source is the `xcotton_pp_variants` block (same field shape as AU's
//!   `product`), modelled by [`MinisForumEuXcottonPpVariants`].
//! - The page is multilingual: `locale` is either `"de"` or `"en"`.
//! - JSON-LD offers carry **no** `name` and **no** `sku`, their
//!   `price_valid_until` is optional, and the availability URL uses the `http://`
//!   scheme (AU used `https://`).

/// One destructured EU product page.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumEuDestructuredProduct {
    /// Page locale, either "de" or "en".
    pub locale: String,
    /// JSON-LD blocks: a Product schema, a brand-only schema and a BreadcrumbList.
    pub schemas: Vec<MinisForumEuSchema>,
    /// `xcotton_pp_variants` — the full Shopify product object (same shape as AU's
    /// `const product`); the canonical product source for this store.
    pub xcotton_pp_variants: MinisForumEuXcottonPpVariants,
    /// web-pixels-manager `initData`: shop, page, related products.
    pub pixels: MinisForumEuPixels,
    /// `var meta = {...}`: the product and its variants.
    pub meta: MinisForumEuMeta,
    /// `track("Viewed Product", {...})`: the currently viewed variant.
    pub viewed_product: MinisForumEuViewedProduct,
    /// The main product DOM section (title/gallery/price/variants).
    #[serde(rename = "xxxx")]
    pub main_product: MinisForumEuMainProduct,
    /// Product specification chart (not present on every page; 22/254).
    pub feature_chart: Option<MinisForumEuFeatureChart>,
    /// Product highlights / contact note box (not present on every page; 74/254).
    pub describe_box: Option<MinisForumEuDescribeBox>,
}

/// One JSON-LD block — a Product, a brand-only block or a BreadcrumbList, so
/// every field is optional.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumEuSchema {
    pub sku: Option<String>,
    pub brand: Option<String>,
    #[serde(default)]
    pub offers: Vec<MinisForumEuOffer>,
    #[serde(default, rename = "itemListElement")]
    pub item_list_element: Vec<MinisForumEuBreadcrumbItem>,
}

/// A Product schema offer. Unlike AU, EU offers carry no `name` and no `sku`, and
/// `price_valid_until` is not always present.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumEuOffer {
    pub price: String,
    pub currency: String,
    pub availability: MinisForumEuAvailability,
    pub price_valid_until: Option<String>,
}

/// Offer stock status, (de)serialized as the schema.org availability URL — the
/// same string in and out. EU uses the `http://` scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MinisForumEuAvailability {
    #[serde(rename = "http://schema.org/InStock")]
    InStock,
    #[serde(rename = "http://schema.org/OutOfStock")]
    OutOfStock,
}

/// A BreadcrumbList entry.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumEuBreadcrumbItem {
    pub name: String,
    pub url: String,
}

/// The full Shopify product object (the `xcotton_pp_variants` key).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumEuXcottonPpVariants {
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
    pub variants: Vec<MinisForumEuProductVariant>,
    #[serde(default)]
    pub media: Vec<MinisForumEuProductMedia>,
}

/// A variant of the product object.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumEuProductVariant {
    pub available: String,
    pub price: String,
    pub option1: String,
    pub option2: Option<String>,
    pub option3: Option<String>,
    pub sku: Option<String>,
    pub compare_at_price: Option<String>,
}

/// A media item of the product object.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumEuProductMedia {
    pub src: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
    #[serde(rename = "type")]
    pub media_type: String,
}

/// web-pixels-manager `initData`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumEuPixels {
    pub shop_name: String,
    pub currency: String,
    pub country: String,
    pub page_type: String,
    pub product_id: String,
    #[serde(default)]
    pub products: Vec<MinisForumEuPixelsProduct>,
}

/// A related product from the pixels `products` list.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumEuPixelsProduct {
    pub id: String,
    pub handle: String,
    pub title: String,
    pub vendor: String,
    pub url: String,
}

/// `var meta = {...}`. Unlike AU there is no `gid`... it carries the same shape
/// as AU's meta block.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumEuMeta {
    pub id: String,
    pub gid: String,
    pub vendor: String,
    pub handle: String,
    #[serde(rename = "type")]
    pub product_type: Option<String>,
    pub page_type: String,
    pub resource_id: String,
    #[serde(default)]
    pub variants: Vec<MinisForumEuMetaVariant>,
}

/// A variant from the `meta` block.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumEuMetaVariant {
    pub variant_id: String,
    pub price: String,
    pub sku: Option<String>,
    pub title: Option<String>,
}

/// `track("Viewed Product", {...})`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumEuViewedProduct {
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

/// The main product DOM section (the `xxxx` key).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumEuMainProduct {
    pub title: String,
    pub badge: Option<String>,
    pub gallery: MinisForumEuGallery,
    pub price: MinisForumEuPrice,
    pub variants: Option<MinisForumEuMainVariants>,
}

/// The gallery within the main product section.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumEuGallery {
    #[serde(default)]
    pub media: Vec<MinisForumEuGalleryMedia>,
}

/// A gallery image. `alt` is not always present in EU.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumEuGalleryMedia {
    pub src: String,
    pub alt: Option<String>,
}

/// The price block within the main product section.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumEuPrice {
    pub sale_price: String,
    pub compare_at_price: Option<String>,
}

/// The variant picker within the main product section.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumEuMainVariants {
    #[serde(default)]
    pub options: Vec<MinisForumEuVariantOption>,
}

/// One option group of the variant picker (e.g. "CPU:").
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumEuVariantOption {
    pub label: String,
    #[serde(default)]
    pub values: Vec<MinisForumEuVariantValue>,
}

/// One selectable value of an option group.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumEuVariantValue {
    pub value: String,
}

/// Product specification chart, column-major (`features[column][row]`). EU charts
/// carry only an `h2` heading; `h1` is never present.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumEuFeatureChart {
    #[serde(default)]
    pub features: Vec<Vec<MinisForumEuFeature>>,
    pub h1: Option<String>,
    pub h2: Option<String>,
}

/// One spec cell.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumEuFeature {
    pub label: String,
    pub value: String,
}

/// Product highlights / contact note box. EU boxes carry only `text`; the `links`
/// list is never present but is modelled (as `#[serde(default)]`) for parity.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumEuDescribeBox {
    pub text: String,
    #[serde(default)]
    pub links: Vec<MinisForumEuLink>,
}

/// A link within the describe box.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumEuLink {
    pub href: String,
}

#[cfg(test)]
mod tests {
    use super::{MinisForumEuAvailability, MinisForumEuDestructuredProduct};

    /// Availability (de)serializes as the schema.org URL — same string in and out
    /// (EU uses the `http://` scheme).
    #[test]
    fn availability_round_trips() {
        for (url, variant) in [
            ("http://schema.org/InStock", MinisForumEuAvailability::InStock),
            ("http://schema.org/OutOfStock", MinisForumEuAvailability::OutOfStock),
        ] {
            let from_url: MinisForumEuAvailability = serde_json::from_str(&format!("\"{url}\"")).unwrap();
            assert_eq!(from_url, variant);

            let serialized = serde_json::to_string(&from_url).unwrap();
            assert_eq!(serialized, format!("\"{url}\""));
        }
    }

    /// Every destructured EU page deserializes into the strict model. With
    /// `deny_unknown_fields` this also proves the model captures every key.
    #[test]
    #[ignore = "TODO: requires local data/offers-destructed fixtures from a destructure run"]
    fn deserializes_every_eu_page() {
        let dir = std::path::Path::new("data/offers-destructed/MinisForumEu");
        let mut count = 0;
        for entry in std::fs::read_dir(dir).expect("EU destructed dir exists").flatten() {
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "json") {
                continue;
            }
            let raw = std::fs::read_to_string(&path).expect("reads file");
            let _product: MinisForumEuDestructuredProduct =
                serde_json::from_str(&raw).unwrap_or_else(|e| panic!("deserialize {}: {e}", path.display()));
            count += 1;
        }
        assert!(count >= 254, "expected at least 254 EU pages, got {count}");
    }
}
