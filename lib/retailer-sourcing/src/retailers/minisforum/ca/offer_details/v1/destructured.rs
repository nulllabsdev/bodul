//! Typed model of the destructured MinisForum CA product JSON.
//!
//! [`MinisForumCaDestructuredProduct`] is a faithful, strictly-matched mirror of the JSON the
//! `destructure` binary writes to `data/offers-destructed/MinisForumCa/`: every
//! top-level key is modelled, and every struct uses `#[serde(deny_unknown_fields)]`
//! so an unexpected or unmodelled key fails deserialization rather than being
//! silently dropped.
//!
//! Every scalar leaf is a `String` — the extractor stringifies all values, so
//! prices (`"2590"`), ids and booleans (`"true"`) all arrive as text. Fields that
//! are not present on every page are `Option<String>` (serialized as `null` when
//! absent); lists that may be absent use `#[serde(default)]` (serialized as `[]`).
//!
//! Unlike AU, CA pages also carry an `xcotton_pp_variants` block — a second copy
//! of the full Shopify product object (same shape and paths as `product`). It is
//! modelled here so `deny_unknown_fields` accepts the key, but `product` remains
//! the canonical product source; `xcotton_pp_variants` is not used downstream.

/// One destructured CA product page.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumCaDestructuredProduct {
    /// Page locale, e.g. "en".
    pub locale: String,
    /// JSON-LD blocks: a Product schema and a BreadcrumbList schema.
    pub schemas: Vec<MinisForumCaSchema>,
    /// `<script id="tt_product">` summary.
    pub tt_product: MinisForumCaTtProduct,
    /// Full Shopify product object (`const product = {...}`).
    pub product: MinisForumCaProduct,
    /// A second copy of the full Shopify product object
    /// (`<script id="xcotton_pp_variants">`); same shape as `product`. Present on
    /// every CA page. Modelled so the key is accepted, but not used downstream —
    /// `product` is the canonical source.
    pub xcotton_pp_variants: MinisForumCaProduct,
    /// web-pixels-manager `initData`: shop, page, related products.
    pub pixels: MinisForumCaPixels,
    /// `var meta = {...}`: the product and its variants.
    pub meta: MinisForumCaMeta,
    /// `track("Viewed Product", {...})`: the currently viewed variant.
    pub viewed_product: MinisForumCaViewedProduct,
    /// The main product DOM section (title/gallery/price/variants).
    #[serde(rename = "xxxx")]
    pub main_product: MinisForumCaMainProduct,
    /// Product specification chart (not present on every page).
    pub feature_chart: Option<MinisForumCaFeatureChart>,
    /// Product highlights / contact note box (not present on every page).
    pub describe_box: Option<MinisForumCaDescribeBox>,
}

/// One JSON-LD block — either a Product or a BreadcrumbList, so every field is
/// optional.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumCaSchema {
    pub sku: Option<String>,
    pub product_id: Option<String>,
    pub brand: Option<String>,
    #[serde(default)]
    pub offers: Vec<MinisForumCaOffer>,
    #[serde(default, rename = "itemListElement")]
    pub item_list_element: Vec<MinisForumCaBreadcrumbItem>,
}

/// A Product schema offer.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumCaOffer {
    pub name: String,
    pub price: String,
    pub currency: String,
    pub availability: MinisForumCaAvailability,
    pub price_valid_until: String,
    pub sku: Option<String>,
}

/// Offer stock status, (de)serialized as the schema.org availability URL — the
/// same string in and out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MinisForumCaAvailability {
    #[serde(rename = "https://schema.org/InStock")]
    InStock,
    #[serde(rename = "https://schema.org/OutOfStock")]
    OutOfStock,
}

/// A BreadcrumbList entry.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumCaBreadcrumbItem {
    pub name: String,
    pub url: String,
}

/// `<script id="tt_product">` summary.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumCaTtProduct {
    pub id: String,
    pub title: String,
    pub image_url: String,
}

/// The full Shopify product object.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumCaProduct {
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
    pub variants: Vec<MinisForumCaProductVariant>,
    #[serde(default)]
    pub media: Vec<MinisForumCaProductMedia>,
}

/// A variant of the full product object.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumCaProductVariant {
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
pub struct MinisForumCaProductMedia {
    pub src: String,
    pub width: String,
    pub height: String,
    #[serde(rename = "type")]
    pub media_type: String,
}

/// web-pixels-manager `initData`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumCaPixels {
    pub shop_name: String,
    pub currency: String,
    pub country: String,
    pub page_type: String,
    pub product_id: String,
    #[serde(default)]
    pub products: Vec<MinisForumCaPixelsProduct>,
}

/// A related product from the pixels `products` list.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumCaPixelsProduct {
    pub id: String,
    pub handle: String,
    pub title: String,
    pub vendor: String,
    pub url: String,
}

/// `var meta = {...}`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumCaMeta {
    pub id: String,
    pub gid: String,
    pub vendor: String,
    pub handle: String,
    #[serde(rename = "type")]
    pub product_type: Option<String>,
    pub page_type: String,
    pub resource_id: String,
    #[serde(default)]
    pub variants: Vec<MinisForumCaMetaVariant>,
}

/// A variant from the `meta` block.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumCaMetaVariant {
    pub variant_id: String,
    pub price: String,
    pub sku: Option<String>,
    pub title: Option<String>,
}

/// `track("Viewed Product", {...})`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumCaViewedProduct {
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
pub struct MinisForumCaMainProduct {
    pub title: String,
    pub badge: Option<String>,
    pub gallery: MinisForumCaGallery,
    pub price: MinisForumCaPrice,
    pub variants: Option<MinisForumCaMainVariants>,
}

/// The gallery within the main product section.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumCaGallery {
    #[serde(default)]
    pub media: Vec<MinisForumCaGalleryMedia>,
}

/// A gallery image.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumCaGalleryMedia {
    pub src: String,
    pub alt: String,
}

/// The price block within the main product section.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumCaPrice {
    pub sale_price: String,
    pub savings: String,
    pub compare_at_price: Option<String>,
}

/// The variant picker within the main product section.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumCaMainVariants {
    #[serde(default)]
    pub options: Vec<MinisForumCaVariantOption>,
}

/// One option group of the variant picker (e.g. "CPU:").
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumCaVariantOption {
    pub label: String,
    pub selected: String,
    #[serde(default)]
    pub values: Vec<MinisForumCaVariantValue>,
}

/// One selectable value of an option group.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumCaVariantValue {
    pub value: String,
}

/// Product specification chart, column-major (`features[column][row]`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumCaFeatureChart {
    #[serde(default)]
    pub features: Vec<Vec<MinisForumCaFeature>>,
    pub h1: Option<String>,
    pub h2: Option<String>,
}

/// One spec cell.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumCaFeature {
    pub label: String,
    pub value: String,
}

/// Product highlights / contact note box.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumCaDescribeBox {
    pub text: String,
    #[serde(default)]
    pub links: Vec<MinisForumCaLink>,
}

/// A link within the describe box.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumCaLink {
    pub href: String,
}

#[cfg(test)]
mod tests {
    use super::{MinisForumCaAvailability, MinisForumCaDestructuredProduct};

    /// Availability (de)serializes as the schema.org URL — same string in and out.
    #[test]
    fn availability_round_trips() {
        for (url, variant) in [
            ("https://schema.org/InStock", MinisForumCaAvailability::InStock),
            ("https://schema.org/OutOfStock", MinisForumCaAvailability::OutOfStock),
        ] {
            let from_url: MinisForumCaAvailability = serde_json::from_str(&format!("\"{url}\"")).unwrap();
            assert_eq!(from_url, variant);

            let serialized = serde_json::to_string(&from_url).unwrap();
            assert_eq!(serialized, format!("\"{url}\""));
        }
    }

    /// Every destructured CA page deserializes into the strict model. With
    /// `deny_unknown_fields` this also proves the model captures every key.
    #[test]
    #[ignore = "TODO: requires local data/offers-destructed fixtures from a destructure run"]
    fn deserializes_every_ca_page() {
        let dir = std::path::Path::new("data/offers-destructed/MinisForumCa");
        let mut count = 0;
        for entry in std::fs::read_dir(dir).expect("CA destructed dir exists").flatten() {
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "json") {
                continue;
            }
            let raw = std::fs::read_to_string(&path).expect("reads file");
            let _product: MinisForumCaDestructuredProduct =
                serde_json::from_str(&raw).unwrap_or_else(|e| panic!("deserialize {}: {e}", path.display()));
            count += 1;
        }
        assert!(count >= 32, "expected at least 32 CA pages, got {count}");
    }
}
