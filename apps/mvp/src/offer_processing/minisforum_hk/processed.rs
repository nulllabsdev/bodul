//! Processed MinisForum HK product model.
//!
//! The output of `process_products` for the HK store: built up field by field by
//! mapping from the [`MinisForumHkDestructuredProduct`] of the previous step into
//! our own shape.
//!
//! HK is **SPARSE / JSON-only** — there is no Shopify `product` object, no
//! main-product DOM gallery and no spec chart, so this model is an *adapted*,
//! best-effort cut of the AU shape:
//!
//! - root keeps `locale` and `product`, but has **no `images`** (no gallery) and
//!   **no `features`** (no feature chart);
//! - the product core is assembled from the analytics `meta` block plus the
//!   JSON-LD Product `schema` (brand / product_id), since there is no product
//!   object to read a canonical price/availability from;
//! - `variants` are combined from **two** sources — `meta.variants` and the
//!   Product schema `offers` — matched by SKU (AU had three, including the
//!   product object's variants).
//!
//! Not emitted: `pixels`, `viewed_product`, the raw `meta` block, the raw
//! `schema` block and `tt_product`.

use chrono::NaiveDate;
use money::{Currency, Money};

use super::destructured::{
    MinisForumHkAvailability, MinisForumHkBmVariant, MinisForumHkDestructuredProduct,
    MinisForumHkMeta, MinisForumHkMetaVariant, MinisForumHkOffer, MinisForumHkSchema,
};

/// A processed MinisForum HK product.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumHkProcessedProduct {
    /// Page locale — always `ZhTw` on the HK store.
    pub locale: MinisForumHkLocale,
    /// The core product fields.
    pub product: MinisForumHkProcessedProductInfo,
    /// Product image URLs, from the `express_main` gallery (`src` only). Empty
    /// when the page has no Express theme DOM.
    pub images: Vec<String>,
    /// Combined variants — one per SKU, built from `meta.variants` and the
    /// Product schema `offers`, enriched by `bm_product_variants`.
    pub variants: Vec<MinisForumHkVariant>,
}

/// One variant combining the `meta`-variant and JSON-LD offer rows that share a
/// SKU. Each field that is guaranteed identical across both sources is lifted to
/// a single field and guarded; `meta_variant_id` keeps its `meta_` prefix as its
/// provenance is meta-specific (distinct from the SKU).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumHkVariant {
    /// SKU — guaranteed identical across both sources.
    pub sku: Option<String>,
    /// Price — guaranteed identical across both sources (meta cents == offer
    /// dollars→cents).
    #[serde(with = "money_wire")]
    pub price: Money,
    /// Availability — from the offer (the only source that carries it).
    pub availability: MinisForumHkProcessedAvailability,
    /// Variant title — the offer name; the `meta` title must match it when
    /// present and non-empty.
    pub title: String,
    /// From the offer.
    pub price_valid_until: NaiveDate,
    /// From the `meta` analytics variant.
    pub meta_variant_id: String,
    /// Variant option values — enriched from `bm_product_variants` by SKU.
    pub option1: Option<String>,
    pub option2: Option<String>,
    pub option3: Option<String>,
    /// Compare-at price — enriched from `bm_product_variants` by SKU (in cents).
    #[serde(with = "option_money_wire")]
    pub compare_at_price: Option<Money>,
}

/// Page locale. The HK store is `zh-TW`-only; (de)serialized as the locale code
/// `"zh-TW"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MinisForumHkLocale {
    #[serde(rename = "zh-TW")]
    ZhTw,
}

impl MinisForumHkLocale {
    /// Builds the variant from the destructured locale code. Errors on any code
    /// other than "zh-TW".
    pub fn from_string(locale: &str) -> Result<Self, String> {
        match locale {
            "zh-TW" => Ok(MinisForumHkLocale::ZhTw),
            other => Err(format!("unexpected locale: {other:?}")),
        }
    }
}

/// The mapped core product fields.
///
/// Assembled from the `meta` block (id / handle / vendor / type) and the JSON-LD
/// Product schema (brand / product_id). HK has no product object, so unlike AU
/// there is no canonical product-level `price` / `compare_at_price` /
/// `availability` here — those live on the combined variants.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumHkProcessedProductInfo {
    pub id: MinisForumHkProductId,
    pub handle: String,
    pub vendor: String,
    /// Product title, from `tt_product.title` when available.
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub product_type: Option<MinisForumHkProductType>,
    /// Manufacturer brand, from the Product schema (absent on a few pages).
    pub brand: Option<String>,
    /// Storefront product id, from the Product schema (absent on a few pages).
    pub schema_product_id: Option<String>,
}

/// Parses a destructured cents string (e.g. "1000") into a minor-unit count.
/// Errors if the string is not a valid integer.
fn parse_cents(price: &str) -> Result<i64, String> {
    price
        .parse()
        .map_err(|error| format!("invalid price {price:?}: {error}"))
}

/// Converts a major-unit dollar string (e.g. "7299.0", "10.0") into a minor-unit
/// (cents) count. Errors on a non-numeric value or more than two fractional
/// digits (HKD has a minor-unit exponent of 2; we do not round).
fn dollars_to_cents(price: &str) -> Result<i64, String> {
    let (whole, fraction) = price.split_once('.').unwrap_or((price, ""));
    if fraction.len() > 2 {
        return Err(format!("price {price:?} has more than 2 fractional digits"));
    }
    let dollars: i64 = whole
        .parse()
        .map_err(|error| format!("invalid price {price:?}: {error}"))?;
    let cents: i64 = format!("{fraction:0<2}")
        .parse()
        .map_err(|error| format!("invalid price {price:?}: {error}"))?;
    Ok(dollars * 100 + cents)
}

/// The canonical [`Money`] wire shape: `{ "amount_minor": "729900", "currency":
/// "HKD" }` (amount as a base-10 string so it round-trips exactly).
#[derive(serde::Serialize, serde::Deserialize)]
struct MoneyWire {
    amount_minor: String,
    currency: String,
}

impl MoneyWire {
    fn from_money(money: &Money) -> Self {
        Self {
            amount_minor: money.minor_units().to_string(),
            currency: currency_code(money.currency()).to_string(),
        }
    }

    fn into_money<E: serde::de::Error>(self) -> Result<Money, E> {
        let amount = self.amount_minor.parse::<i64>().map_err(E::custom)?;
        let currency = currency_from_code(&self.currency).map_err(E::custom)?;
        Ok(Money::new(amount, currency))
    }
}

/// The ISO 4217 alpha code for a [`Currency`].
fn currency_code(currency: Currency) -> &'static str {
    match currency {
        Currency::USD => "USD",
        Currency::EUR => "EUR",
        Currency::CAD => "CAD",
        Currency::AUD => "AUD",
        Currency::GBP => "GBP",
        Currency::JPY => "JPY",
        Currency::KRW => "KRW",
        Currency::HKD => "HKD",
    }
}

/// The [`Currency`] for an ISO 4217 alpha code.
fn currency_from_code(code: &str) -> Result<Currency, String> {
    match code {
        "USD" => Ok(Currency::USD),
        "EUR" => Ok(Currency::EUR),
        "CAD" => Ok(Currency::CAD),
        "AUD" => Ok(Currency::AUD),
        "HKD" => Ok(Currency::HKD),
        other => Err(format!("unknown currency: {other:?}")),
    }
}

/// serde glue for a [`Money`] field, (de)serialized as the canonical wire object.
mod money_wire {
    use super::{Money, MoneyWire};

    pub fn serialize<S: serde::Serializer>(
        money: &Money,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serde::Serialize::serialize(&MoneyWire::from_money(money), serializer)
    }

    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Money, D::Error> {
        let wire: MoneyWire = serde::Deserialize::deserialize(deserializer)?;
        wire.into_money()
    }
}

/// serde glue for an optional [`Money`] field.
mod option_money_wire {
    use super::{Money, MoneyWire};

    pub fn serialize<S: serde::Serializer>(
        money: &Option<Money>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match money {
            Some(money) => serializer.serialize_some(&MoneyWire::from_money(money)),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Money>, D::Error> {
        let wire: Option<MoneyWire> = serde::Deserialize::deserialize(deserializer)?;
        wire.map(MoneyWire::into_money).transpose()
    }
}

/// Product category, (de)serialized as the original `type` string. HK categories
/// are zh-TW labels (plus the English "Mini PC").
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MinisForumHkProductType {
    #[serde(rename = "Mini PC")]
    MiniPc,
    /// 主機板 — motherboard.
    #[serde(rename = "主機板")]
    Motherboard,
    /// 高性能銳龍 — high-performance Ryzen.
    #[serde(rename = "高性能銳龍")]
    HighPerformanceRyzen,
    /// 配件 — accessory.
    #[serde(rename = "配件")]
    Accessory,
}

impl MinisForumHkProductType {
    /// Builds the variant from the destructured `type` string. Errors on any
    /// value not seen in the data.
    pub fn from_string(product_type: &str) -> Result<Self, String> {
        match product_type {
            "Mini PC" => Ok(MinisForumHkProductType::MiniPc),
            "主機板" => Ok(MinisForumHkProductType::Motherboard),
            "高性能銳龍" => Ok(MinisForumHkProductType::HighPerformanceRyzen),
            "配件" => Ok(MinisForumHkProductType::Accessory),
            other => Err(format!("unexpected product type: {other:?}")),
        }
    }
}

/// A product id. Built from the destructured `meta.id` string, (de)serialized as
/// a number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MinisForumHkProductId(pub u64);

impl MinisForumHkProductId {
    /// Builds the id by parsing the destructured id string. Errors if the string
    /// is not a valid number.
    pub fn from_string(id: &str) -> Result<Self, String> {
        id.parse()
            .map(Self)
            .map_err(|error| format!("invalid product id {id:?}: {error}"))
    }
}

/// Product stock status. (De)serialized as a JSON boolean (`true` = available).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(into = "bool", from = "bool")]
pub enum MinisForumHkProcessedAvailability {
    Available,
    Unavailable,
}

impl From<bool> for MinisForumHkProcessedAvailability {
    fn from(available: bool) -> Self {
        if available {
            MinisForumHkProcessedAvailability::Available
        } else {
            MinisForumHkProcessedAvailability::Unavailable
        }
    }
}

impl From<MinisForumHkProcessedAvailability> for bool {
    fn from(availability: MinisForumHkProcessedAvailability) -> Self {
        availability == MinisForumHkProcessedAvailability::Available
    }
}

impl From<MinisForumHkAvailability> for MinisForumHkProcessedAvailability {
    fn from(availability: MinisForumHkAvailability) -> Self {
        match availability {
            MinisForumHkAvailability::InStock => MinisForumHkProcessedAvailability::Available,
            MinisForumHkAvailability::OutOfStock => MinisForumHkProcessedAvailability::Unavailable,
        }
    }
}

impl TryFrom<MinisForumHkDestructuredProduct> for MinisForumHkProcessedProduct {
    type Error = String;

    fn try_from(destructured: MinisForumHkDestructuredProduct) -> Result<Self, Self::Error> {
        // The first schema is the Product schema (brand / product_id / offers);
        // the second is the BreadcrumbList. We need the Product schema both for
        // the core (brand / product_id) and as the offers source for variants.
        let product_schema = destructured
            .schemas
            .into_iter()
            .next()
            .ok_or_else(|| "missing product schema".to_string())?;

        let title = Some(destructured.tt_product.title.clone());

        let variants = make_variants(
            destructured.meta.variants.clone(),
            product_schema.offers.clone(),
            &destructured.bm_product_variants,
        )?;

        let product = make_product_info(&destructured.meta, &product_schema, title)?;

        let images = destructured
            .express_main
            .and_then(|main| main.gallery)
            .map(|g| g.media.into_iter().filter_map(|m| m.src).collect())
            .unwrap_or_default();

        Ok(Self {
            locale: MinisForumHkLocale::from_string(&destructured.locale)?,
            product,
            images,
            variants,
        })
    }
}

/// Assembles the core product info from the `meta` block and the Product schema.
fn make_product_info(
    meta: &MinisForumHkMeta,
    schema: &MinisForumHkSchema,
    title: Option<String>,
) -> Result<MinisForumHkProcessedProductInfo, String> {
    Ok(MinisForumHkProcessedProductInfo {
        id: MinisForumHkProductId::from_string(&meta.id)?,
        handle: meta.handle.clone(),
        vendor: meta.vendor.clone(),
        title,
        product_type: meta
            .product_type
            .as_deref()
            .map(MinisForumHkProductType::from_string)
            .transpose()?,
        brand: schema.brand.clone(),
        schema_product_id: schema.product_id.clone(),
    })
}

/// Combines `meta.variants` and Product schema `offers` into [`MinisForumHkVariant`]s,
/// matched by SKU. Both sources are present on every HK page; the lengths must
/// agree and every meta variant must have a same-SKU offer. Enriched by
/// `bm_product_variants` for option/compare-at data.
fn make_variants(
    meta_variants: Vec<MinisForumHkMetaVariant>,
    offers: Vec<MinisForumHkOffer>,
    bm_product_variants: &[MinisForumHkBmVariant],
) -> Result<Vec<MinisForumHkVariant>, String> {
    // Guard the two sources have the same length.
    if meta_variants.len() != offers.len() {
        return Err(format!(
            "variant source length mismatch: meta={}, offers={}",
            meta_variants.len(),
            offers.len(),
        ));
    }

    // For each meta variant, find the offer with the same SKU and combine them.
    let mut combined = Vec::with_capacity(meta_variants.len());
    for meta in meta_variants {
        let offer = offers
            .iter()
            .find(|offer| offer.sku == meta.sku)
            .cloned()
            .ok_or_else(|| format!("no offer for sku {:?}", meta.sku))?;
        let bmv = meta.sku.as_deref().and_then(|sku| {
            bm_product_variants
                .iter()
                .find(|bmv| bmv.sku.as_deref() == Some(sku))
        });
        combined.push(make_variant(meta, offer, bmv)?);
    }

    Ok(combined)
}

/// Combines one matched (`meta` variant, offer) pair into a [`MinisForumHkVariant`],
/// enriched by the matching `bm_product_variants` entry for option/compare-at data.
fn make_variant(
    meta: MinisForumHkMetaVariant,
    offer: MinisForumHkOffer,
    bmv: Option<&MinisForumHkBmVariant>,
) -> Result<MinisForumHkVariant, String> {
    // Both sources must agree on the SKU; lift it to a single field.
    if meta.sku != offer.sku {
        return Err(format!(
            "variant sku mismatch: meta={:?}, offer={:?}",
            meta.sku, offer.sku
        ));
    }

    let offer_currency = currency_from_code(&offer.currency)?;

    // Both sources must agree on the price; lift it to a single field.
    let meta_price = Money::new(parse_cents(&meta.price)?, Currency::HKD);
    let offer_price = Money::new(dollars_to_cents(&offer.price)?, offer_currency);
    if meta_price != offer_price {
        return Err(format!(
            "variant price mismatch: meta={meta_price:?}, offer={offer_price:?}"
        ));
    }

    // Title comes from the offer name (always present); the `meta` title, when
    // present and non-empty, must match it.
    let title = offer.name;
    if let Some(meta_title) = &meta.title {
        if !meta_title.is_empty() && meta_title != &title {
            return Err(format!(
                "variant title mismatch: meta={meta_title:?}, offer={title:?}"
            ));
        }
    }

    let (option1, option2, option3, compare_at_price) = match bmv {
        Some(bmv) => {
            let cat = bmv
                .compare_at_price
                .as_deref()
                .map(parse_cents)
                .transpose()?
                .map(|cents| Money::new(cents, Currency::HKD));
            (
                bmv.option1.clone(),
                bmv.option2.clone(),
                bmv.option3.clone(),
                cat,
            )
        }
        None => (None, None, None, None),
    };

    Ok(MinisForumHkVariant {
        sku: meta.sku,
        price: meta_price,
        availability: offer.availability.into(),
        title,
        price_valid_until: offer.price_valid_until.parse().map_err(|error| {
            format!(
                "invalid price_valid_until {:?}: {error}",
                offer.price_valid_until
            )
        })?,
        meta_variant_id: meta.variant_id,
        option1,
        option2,
        option3,
        compare_at_price,
    })
}
