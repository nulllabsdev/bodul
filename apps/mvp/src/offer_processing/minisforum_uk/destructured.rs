//! Typed model of the destructured MinisForum UK product JSON.
//!
//! [`MinisForumUkDestructuredProduct`] is a faithful, strictly-matched mirror of the JSON the
//! `destructure` binary writes to `data/pages-destructed/MinisForumUk/`: every
//! top-level key is modelled, and every struct uses `#[serde(deny_unknown_fields)]`
//! so an unexpected or unmodelled key fails deserialization rather than being
//! silently dropped.
//!
//! Every scalar leaf is a `String` — the extractor stringifies all values, so
//! prices (`"2590"`), ids and booleans (`"true"`) all arrive as text. Fields that
//! are not present on every page are `Option<String>` (serialized as `null` when
//! absent); lists that may be absent use `#[serde(default)]` (serialized as `[]`).
//!
//! Unlike the AU store, the UK store has **no** `const product = {...}` Shopify
//! product object — the product is sourced from `xcotton_pp_variants` instead.

/// One destructured UK product page.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUkDestructuredProduct {
    /// Page locale, e.g. "en".
    pub locale: String,
    /// JSON-LD blocks: a Product schema and a BreadcrumbList schema.
    pub schemas: Vec<MinisForumUkSchema>,
    /// `<script id="tt_product">` summary.
    pub tt_product: MinisForumUkTtProduct,
    /// The xcotton product-variants object (`var __xcotton_pp_variants__`). The UK
    /// product source — there is no `const product` block.
    pub xcotton_pp_variants: MinisForumUkXcottonProduct,
    /// web-pixels-manager `initData`: shop, page, related products.
    pub pixels: MinisForumUkPixels,
    /// `var meta = {...}`: the product and its variants.
    pub meta: MinisForumUkMeta,
    /// `track("Viewed Product", {...})`: the currently viewed variant.
    pub viewed_product: MinisForumUkViewedProduct,
    /// The main product DOM section (title/gallery/price/variants).
    #[serde(rename = "xxxx")]
    pub main_product: MinisForumUkMainProduct,
    /// Product specification chart (not present on every page).
    pub feature_chart: Option<MinisForumUkFeatureChart>,
    /// Product highlights / contact note box (not present on every page).
    pub describe_box: Option<MinisForumUkDescribeBox>,
}

/// One JSON-LD block — either a Product or a BreadcrumbList, so every field is
/// optional.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUkSchema {
    pub sku: Option<String>,
    pub product_id: Option<String>,
    pub brand: Option<String>,
    #[serde(default)]
    pub offers: Vec<MinisForumUkOffer>,
    #[serde(default, rename = "itemListElement")]
    pub item_list_element: Vec<MinisForumUkBreadcrumbItem>,
}

/// A Product schema offer.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUkOffer {
    pub name: String,
    pub price: String,
    pub currency: String,
    pub availability: MinisForumUkAvailability,
    pub price_valid_until: String,
    pub sku: Option<String>,
}

/// Offer stock status, (de)serialized as the schema.org availability URL — the
/// same string in and out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MinisForumUkAvailability {
    #[serde(rename = "https://schema.org/InStock")]
    InStock,
    #[serde(rename = "https://schema.org/OutOfStock")]
    OutOfStock,
}

/// A BreadcrumbList entry.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUkBreadcrumbItem {
    pub name: String,
    pub url: String,
}

/// `<script id="tt_product">` summary.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUkTtProduct {
    pub id: String,
    pub title: String,
    pub image_url: String,
}

/// The xcotton product-variants object. The UK product source: it carries the
/// product core (id/title/handle/vendor/type/price), its variants and its media.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUkXcottonProduct {
    pub id: String,
    pub title: String,
    pub handle: String,
    pub vendor: String,
    #[serde(rename = "type")]
    pub product_type: String,
    pub available: String,
    pub price: String,
    pub price_min: String,
    pub price_max: String,
    pub compare_at_price: Option<String>,
    #[serde(default)]
    pub variants: Vec<MinisForumUkXcottonVariant>,
    #[serde(default)]
    pub media: Vec<MinisForumUkXcottonMedia>,
}

/// A variant of the xcotton product object.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUkXcottonVariant {
    pub available: String,
    pub price: String,
    pub option1: String,
    pub option2: Option<String>,
    pub option3: Option<String>,
    pub sku: Option<String>,
    pub compare_at_price: Option<String>,
}

/// A media item of the xcotton product object. Images carry `src`/`width`/
/// `height`; a `video` item may have only `type`, so those three are optional.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUkXcottonMedia {
    pub src: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
    #[serde(rename = "type")]
    pub media_type: String,
}

/// web-pixels-manager `initData`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUkPixels {
    pub shop_name: String,
    pub currency: String,
    pub country: String,
    pub page_type: String,
    pub product_id: String,
    #[serde(default)]
    pub products: Vec<MinisForumUkPixelsProduct>,
}

/// A related product from the pixels `products` list.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUkPixelsProduct {
    pub id: String,
    pub handle: String,
    pub title: String,
    pub vendor: String,
    pub url: String,
}

/// `var meta = {...}`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUkMeta {
    pub id: String,
    pub gid: String,
    pub vendor: String,
    pub handle: String,
    #[serde(rename = "type")]
    pub product_type: String,
    pub page_type: String,
    pub resource_id: String,
    #[serde(default)]
    pub variants: Vec<MinisForumUkMetaVariant>,
}

/// A variant from the `meta` block.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUkMetaVariant {
    pub variant_id: String,
    pub price: String,
    pub sku: Option<String>,
    pub title: Option<String>,
}

/// `track("Viewed Product", {...})`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUkViewedProduct {
    pub currency: String,
    pub variant_id: String,
    pub product_id: String,
    pub gid: String,
    pub name: String,
    pub price: String,
    pub brand: String,
    pub sku: Option<String>,
    pub variant: Option<String>,
    pub category: String,
}

/// The main product DOM section (the `xxxx` key).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUkMainProduct {
    pub title: String,
    pub badge: Option<String>,
    pub gallery: MinisForumUkGallery,
    pub price: MinisForumUkPrice,
    pub variants: Option<MinisForumUkMainVariants>,
}

/// The gallery within the main product section.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUkGallery {
    #[serde(default)]
    pub media: Vec<MinisForumUkGalleryMedia>,
}

/// A gallery image.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUkGalleryMedia {
    pub src: String,
    pub alt: String,
}

/// The price block within the main product section.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUkPrice {
    pub sale_price: String,
    pub savings: String,
    pub compare_at_price: Option<String>,
}

/// The variant picker within the main product section.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUkMainVariants {
    #[serde(default)]
    pub options: Vec<MinisForumUkVariantOption>,
}

/// One option group of the variant picker (e.g. "CPU:").
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUkVariantOption {
    pub label: String,
    pub selected: String,
    #[serde(default)]
    pub values: Vec<MinisForumUkVariantValue>,
}

/// One selectable value of an option group.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUkVariantValue {
    pub value: String,
}

/// Product specification chart, column-major (`features[column][row]`). UK charts
/// carry an optional `h2` heading and (unlike AU) never an `h1`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUkFeatureChart {
    #[serde(default)]
    pub features: Vec<Vec<MinisForumUkFeature>>,
    pub h1: Option<String>,
    pub h2: Option<String>,
}

/// One spec cell.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUkFeature {
    pub label: String,
    pub value: String,
}

/// Product highlights / contact note box.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUkDescribeBox {
    pub text: String,
    #[serde(default)]
    pub links: Vec<MinisForumUkLink>,
}

/// A link within the describe box.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinisForumUkLink {
    pub href: String,
}

#[cfg(test)]
mod tests {
    use super::{MinisForumUkAvailability, MinisForumUkDestructuredProduct};

    /// Availability (de)serializes as the schema.org URL — same string in and out.
    #[test]
    fn availability_round_trips() {
        for (url, variant) in [
            ("https://schema.org/InStock", MinisForumUkAvailability::InStock),
            ("https://schema.org/OutOfStock", MinisForumUkAvailability::OutOfStock),
        ] {
            let from_url: MinisForumUkAvailability = serde_json::from_str(&format!("\"{url}\"")).unwrap();
            assert_eq!(from_url, variant);

            let serialized = serde_json::to_string(&from_url).unwrap();
            assert_eq!(serialized, format!("\"{url}\""));
        }
    }

    /// Every destructured UK page deserializes into the strict model. With
    /// `deny_unknown_fields` this also proves the model captures every key.
    #[test]
    #[ignore = "TODO: requires local data/pages-destructed fixtures from a destructure run"]
    fn deserializes_every_uk_page() {
        let dir = std::path::Path::new("data/pages-destructed/MinisForumUk");
        let mut count = 0;
        for entry in std::fs::read_dir(dir).expect("UK destructed dir exists").flatten() {
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "json") {
                continue;
            }
            let raw = std::fs::read_to_string(&path).expect("reads file");
            let _product: MinisForumUkDestructuredProduct =
                serde_json::from_str(&raw).unwrap_or_else(|e| panic!("deserialize {}: {e}", path.display()));
            count += 1;
        }
        assert!(count >= 61, "expected at least 61 UK pages, got {count}");
    }
}
