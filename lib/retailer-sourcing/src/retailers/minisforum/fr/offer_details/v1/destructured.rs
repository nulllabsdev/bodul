//! Typed model of the destructured MinisForum FR product JSON.
//!
//! [`MinisForumFrDestructuredProduct`] is a faithful, strictly-matched mirror of the JSON the
//! `destructure` binary writes to `data/offers-destructed/MinisForumFr/`: every
//! top-level key is modelled, and every struct uses `#[serde(deny_unknown_fields)]`
//! so an unexpected or unmodelled key fails deserialization rather than being
//! silently dropped.
//!
//! Every scalar leaf is a `String` — the extractor stringifies all values, so
//! prices (`"76900"`), ids and booleans (`"true"`) all arrive as text. Fields that
//! are not present on every page are `Option<String>` (serialized as `null` when
//! absent); lists that may be absent use `#[serde(default)]` (serialized as `[]`).
//!
//! The FR store differs from AU: the product object is `xcotton_pp_variants`
//! (there is **no** `const product` and **no** `tt_product`), and the JSON-LD
//! `offers` carry **no** `name`/`sku` (they are product-level, not per-variant).

/// One destructured FR product page.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumFrDestructuredProduct {
    /// Page locale: "en" or "fr".
    pub locale: String,
    /// JSON-LD blocks: a Product schema, a BreadcrumbList schema, and brand-only
    /// blocks.
    pub schemas: Vec<MinisForumFrSchema>,
    /// web-pixels-manager `initData`: shop, page, related products.
    pub pixels: MinisForumFrPixels,
    /// `var meta = {...}`: the product and its variants.
    pub meta: MinisForumFrMeta,
    /// `track("Viewed Product", {...})`: the currently viewed variant.
    pub viewed_product: MinisForumFrViewedProduct,
    /// The Shopify product object (`var __xcotton_pp_variants__ = {...}`).
    pub xcotton_pp_variants: MinisForumFrXcottonPpVariants,
    /// The main product DOM section (title/gallery/price/variants).
    #[serde(rename = "xxxx")]
    pub main_product: MinisForumFrMainProduct,
    /// Product specification chart (rare — only 2/98 pages).
    pub feature_chart: Option<MinisForumFrFeatureChart>,
    /// Product highlights / contact note box (not present on every page).
    pub describe_box: Option<MinisForumFrDescribeBox>,
}

/// One JSON-LD block — a Product, a BreadcrumbList, or a brand-only block, so
/// every field is optional. FR offers carry **no** `name`/`sku`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumFrSchema {
    pub sku: Option<String>,
    pub brand: Option<String>,
    #[serde(default)]
    pub offers: Vec<MinisForumFrOffer>,
    #[serde(default, rename = "itemListElement")]
    pub item_list_element: Vec<MinisForumFrBreadcrumbItem>,
}

/// A Product schema offer. FR offers have no `name`/`sku` and are product-level
/// (0, 1, or 2 per page), not aligned to individual variants.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumFrOffer {
    pub price: String,
    pub currency: String,
    pub availability: MinisForumFrAvailability,
    pub price_valid_until: Option<String>,
}

/// Offer stock status, (de)serialized as the schema.org availability URL — the
/// same string in and out. FR uses the `http://` (not `https://`) form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MinisForumFrAvailability {
    #[serde(rename = "http://schema.org/InStock")]
    InStock,
    #[serde(rename = "http://schema.org/OutOfStock")]
    OutOfStock,
}

/// A BreadcrumbList entry.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumFrBreadcrumbItem {
    pub name: String,
    pub url: String,
}

/// The Shopify product object (`var __xcotton_pp_variants__ = {...}`). This is
/// the FR analogue of AU's `const product`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumFrXcottonPpVariants {
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
    pub variants: Vec<MinisForumFrXcottonVariant>,
    #[serde(default)]
    pub media: Vec<MinisForumFrXcottonMedia>,
}

/// A variant of the `xcotton_pp_variants` product object. FR variants have at
/// most two option columns (no `option3`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumFrXcottonVariant {
    pub available: String,
    pub price: String,
    pub option1: String,
    pub option2: Option<String>,
    pub sku: Option<String>,
    pub compare_at_price: Option<String>,
}

/// A media item of the `xcotton_pp_variants` product object.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumFrXcottonMedia {
    pub src: String,
    pub width: String,
    pub height: String,
    #[serde(rename = "type")]
    pub media_type: String,
}

/// web-pixels-manager `initData`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumFrPixels {
    pub shop_name: String,
    pub currency: String,
    pub country: String,
    pub page_type: String,
    pub product_id: String,
    #[serde(default)]
    pub products: Vec<MinisForumFrPixelsProduct>,
}

/// A related product from the pixels `products` list.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumFrPixelsProduct {
    pub id: String,
    pub handle: String,
    pub title: String,
    pub vendor: String,
    pub url: String,
}

/// `var meta = {...}`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumFrMeta {
    pub id: String,
    pub gid: String,
    pub vendor: String,
    pub handle: String,
    #[serde(rename = "type")]
    pub product_type: Option<String>,
    pub page_type: String,
    pub resource_id: String,
    #[serde(default)]
    pub variants: Vec<MinisForumFrMetaVariant>,
}

/// A variant from the `meta` block.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumFrMetaVariant {
    pub variant_id: String,
    pub price: String,
    pub sku: Option<String>,
    pub title: Option<String>,
}

/// `track("Viewed Product", {...})`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumFrViewedProduct {
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
pub struct MinisForumFrMainProduct {
    pub title: String,
    pub badge: Option<String>,
    pub gallery: MinisForumFrGallery,
    pub price: MinisForumFrPrice,
    pub variants: Option<MinisForumFrMainVariants>,
}

/// The gallery within the main product section.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumFrGallery {
    #[serde(default)]
    pub media: Vec<MinisForumFrGalleryMedia>,
}

/// A gallery image.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumFrGalleryMedia {
    pub src: String,
    pub alt: String,
}

/// The price block within the main product section.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumFrPrice {
    pub sale_price: String,
    pub savings: String,
    pub compare_at_price: Option<String>,
}

/// The variant picker within the main product section.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumFrMainVariants {
    #[serde(default)]
    pub options: Vec<MinisForumFrVariantOption>,
}

/// One option group of the variant picker (e.g. "CPU:").
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumFrVariantOption {
    pub label: String,
    pub selected: String,
    #[serde(default)]
    pub values: Vec<MinisForumFrVariantValue>,
}

/// One selectable value of an option group.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumFrVariantValue {
    pub value: String,
}

/// Product specification chart, column-major (`features[column][row]`). Rare on
/// FR (only 2/98 pages); `h1` is never present, `h2` always is.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumFrFeatureChart {
    #[serde(default)]
    pub features: Vec<Vec<MinisForumFrFeature>>,
    pub h1: Option<String>,
    pub h2: Option<String>,
}

/// One spec cell.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumFrFeature {
    pub label: String,
    pub value: String,
}

/// Product highlights / contact note box.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumFrDescribeBox {
    pub text: String,
    #[serde(default)]
    pub links: Vec<MinisForumFrLink>,
}

/// A link within the describe box.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumFrLink {
    pub href: String,
}

#[cfg(test)]
mod tests {
    use super::{MinisForumFrAvailability, MinisForumFrDestructuredProduct};

    /// Availability (de)serializes as the schema.org URL — same string in and
    /// out. FR uses the `http://` form.
    #[test]
    fn availability_round_trips() {
        for (url, variant) in [
            ("http://schema.org/InStock", MinisForumFrAvailability::InStock),
            ("http://schema.org/OutOfStock", MinisForumFrAvailability::OutOfStock),
        ] {
            let from_url: MinisForumFrAvailability = serde_json::from_str(&format!("\"{url}\"")).unwrap();
            assert_eq!(from_url, variant);

            let serialized = serde_json::to_string(&from_url).unwrap();
            assert_eq!(serialized, format!("\"{url}\""));
        }
    }

    /// Every destructured FR page deserializes into the strict model. With
    /// `deny_unknown_fields` this also proves the model captures every key.
    #[test]
    #[ignore = "TODO: requires local data/offers-destructed fixtures from a destructure run"]
    fn deserializes_every_fr_page() {
        let dir = std::path::Path::new("data/offers-destructed/MinisForumFr");
        let mut count = 0;
        for entry in std::fs::read_dir(dir).expect("FR destructed dir exists").flatten() {
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "json") {
                continue;
            }
            let raw = std::fs::read_to_string(&path).expect("reads file");
            let _product: MinisForumFrDestructuredProduct =
                serde_json::from_str(&raw).unwrap_or_else(|e| panic!("deserialize {}: {e}", path.display()));
            count += 1;
        }
        assert!(count >= 98, "expected at least 98 FR pages, got {count}");
    }
}
