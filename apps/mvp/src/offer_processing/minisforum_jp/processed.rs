//! Processed MinisForum JP product model.
//!
//! The output of `process_products`: built up field by field by mapping from the
//! [`MinisForumJpDestructuredProduct`] of the previous step into our own shape.
//!
//! ## Adapting the AU shape to a SPARSE store (no product object)
//!
//! The JP store has **no `product` object**, so the AU strategy of deriving the
//! product core (and the variant join) from `product` does not apply. Instead:
//!
//! - **Product core** is derived from `meta` (id / handle / vendor / type) plus
//!   the first JSON-LD Product `schema` (sku / brand) and that schema's single
//!   offer (price / availability). See [`MinisForumJpProcessedProductInfo`].
//! - **Variants** combine **two** sources by SKU: `meta.variants` (the full list,
//!   each with its own cents price) and the **single** `schema` offer. JP offers
//!   carry no sku, so the offer is attributed to the meta variant whose `sku`
//!   equals the schema's `sku` (the default/representative variant); every other
//!   meta variant has no offer and so no offer-derived fields. There is **no
//!   product-variant source and no `option1/2/3`** — meta variants have no
//!   options, so those fields are omitted from [`ZzzVariant`].
//!
//! ## Price units
//!
//! `meta.variants[].price` is a **cents** string (`"11999900"` = ¥119,999.00) →
//! [`parse_cents`]. The offer `price` is a **major-unit (yen)** string that uses a
//! period as a *thousands separator* (`"119.999"` = ¥119,999; `"1000"` = ¥1,000)
//! → [`dollars_to_cents`], which strips the separators and multiplies by 100.
//! Both land on the same minor-unit count, so the offer and meta prices can be
//! guarded for equality.

use chrono::NaiveDate;
use money::{Currency, Money};

use super::destructured::{
    MinisForumJpAvailability, MinisForumJpDestructuredProduct, MinisForumJpFeature,
    MinisForumJpMetaVariant, MinisForumJpOffer, MinisForumJpProductVariant, MinisForumJpSchema,
};

/// A processed MinisForum JP product.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumJpProcessedProduct {
    /// Page locale.
    pub locale: MinisForumJpLocale,
    /// The core product fields, derived from `meta` + the Product `schema`.
    pub product: MinisForumJpProcessedProductInfo,
    /// Product image URLs, from the `xxxx` gallery (`src` only). Empty when the
    /// page has no `xxxx` block.
    pub images: Vec<String>,
    /// Specification rows, from `feature_chart` (the column-major chart is
    /// flattened). Empty when the page has no feature chart.
    pub features: Vec<MinisForumJpProcessedFeature>,
    /// The combined variants — `meta.variants` joined with the single offer by
    /// SKU.
    pub variants: Vec<ZzzVariant>,
}

/// One product specification: a `label` and its `value` lines (the raw cell text
/// split on `\n`, with blank/whitespace-only lines dropped).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumJpProcessedFeature {
    pub label: String,
    pub value: Vec<String>,
}

impl From<MinisForumJpFeature> for MinisForumJpProcessedFeature {
    fn from(feature: MinisForumJpFeature) -> Self {
        Self {
            label: feature.label,
            value: collapse_after_colon(&feature.value)
                .split('\n')
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .map(String::from)
                .collect(),
        }
    }
}

/// Collapses the whitespace that follows a colon (ASCII `:` or full-width `：`)
/// down to a single space — so a sub-label keeps its value on the same line
/// (e.g. `"Processor：\n  Core Ultra 5"` → `"Processor： Core Ultra 5"`).
fn collapse_after_colon(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(current) = chars.next() {
        out.push(current);
        if (current == ':' || current == '：')
            && chars.peek().is_some_and(|next| next.is_whitespace())
        {
            while chars.peek().is_some_and(|next| next.is_whitespace()) {
                chars.next();
            }
            out.push(' ');
        }
    }
    out
}

/// One combined variant for the SPARSE JP store: the `meta` analytics variant,
/// optionally joined with the page's single JSON-LD offer (only the meta variant
/// whose sku matches the schema sku gets the offer).
///
/// Fields are prefixed with their source where provenance matters. There is no
/// product-object source here and meta variants carry no options, so the AU
/// `option1/2/3` fields are intentionally absent.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ZzzVariant {
    /// SKU, from the `meta` variant (null on some variants).
    pub sku: Option<String>,
    /// Price — the meta cents value. When this variant carries the offer, the
    /// offer's major-unit price is guarded to equal it.
    #[serde(with = "money_wire")]
    pub price: Money,
    /// Availability — from the offer when this variant carries it; otherwise
    /// `None` (meta variants have no standalone availability flag).
    pub availability: Option<MinisForumJpProcessedAvailability>,
    /// Variant title, from the `meta` variant (present on all but one).
    pub title: Option<String>,
    /// From the offer, when this variant carries it.
    pub price_valid_until: Option<NaiveDate>,
    /// The `meta` analytics variant id (distinct from SKU).
    pub meta_variant_id: String,
    /// Variant option values — enriched from `product_variants` by SKU.
    pub option1: Option<String>,
    pub option2: Option<String>,
    pub option3: Option<String>,
    /// Compare-at price — enriched from `product_variants` by SKU (in cents).
    #[serde(with = "option_money_wire")]
    pub compare_at_price: Option<Money>,
}

/// Page locale. The JP store is Japanese-only; (de)serialized as the locale code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MinisForumJpLocale {
    #[serde(rename = "ja")]
    Ja,
}

impl MinisForumJpLocale {
    /// Builds the variant from the destructured locale code. Errors on any code
    /// other than "ja".
    pub fn from_string(locale: &str) -> Result<Self, String> {
        match locale {
            "ja" => Ok(MinisForumJpLocale::Ja),
            other => Err(format!("unexpected locale: {other:?}")),
        }
    }
}

/// The product core, derived without a product object: identity from `meta`,
/// sku/brand from the JSON-LD Product `schema`, and price/availability from that
/// schema's single offer.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumJpProcessedProductInfo {
    /// Product id, parsed from `meta.id`.
    pub id: MinisForumJpProductId,
    /// Handle, from `meta.handle`.
    pub handle: String,
    /// Vendor, from `meta.vendor`.
    pub vendor: String,
    /// Product title, from the `xxxx` DOM title when available.
    pub title: Option<String>,
    /// Product type, from `meta.type` (Option — absent on 2 pages).
    #[serde(rename = "type")]
    pub product_type: Option<MinisForumJpProductType>,
    /// SKU, from the Product schema (Option — one offer-bearing schema has none).
    pub sku: Option<String>,
    /// Brand, from the Product schema (Option).
    pub brand: Option<String>,
    /// Price, from the schema offer (major-unit yen → minor units).
    #[serde(with = "money_wire")]
    pub price: Money,
    /// Availability, from the schema offer.
    pub availability: MinisForumJpProcessedAvailability,
}

/// Parses a destructured cents string (e.g. `"11999900"`) into a minor-unit
/// count. Errors if the string is not a valid integer.
fn parse_cents(price: &str) -> Result<i64, String> {
    price
        .parse()
        .map_err(|error| format!("invalid price {price:?}: {error}"))
}

/// Converts a JP offer price into a minor-unit (cents) count.
///
/// JP offer prices are **major-unit (yen)** values whose period is a *thousands*
/// separator, not a decimal point (`"119.999"` = ¥119,999; `"1000"` = ¥1,000).
/// We strip the separators, parse the integer yen amount, and multiply by 100 to
/// reach the same minor-unit count `meta` reports as cents. (JPY has 0 minor-unit
/// digits in reality, but — per the AU `Money` convention — we keep an exponent-2
/// minor-unit representation so meta and offer prices line up exactly.)
fn dollars_to_cents(price: &str) -> Result<i64, String> {
    let yen: i64 = price
        .replace('.', "")
        .parse()
        .map_err(|error| format!("invalid price {price:?}: {error}"))?;
    Ok(yen * 100)
}

/// The canonical [`Money`] wire shape: `{ "amount_minor": "11999900", "currency":
/// "JPY" }` (amount as a base-10 string so it round-trips exactly).
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
///
/// NOTE: the JP store trades in JPY. The shared `money::Currency` enum does not
/// yet carry a `JPY` variant (it currently lists USD/EUR/CAD/AUD); a `JPY`
/// variant must be added there for this module to build. Until then this maps the
/// expected `Currency::JPY`.
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

/// The [`Currency`] for an ISO 4217 alpha code. JP only ever sees `JPY`.
fn currency_from_code(code: &str) -> Result<Currency, String> {
    match code {
        "JPY" => Ok(Currency::JPY),
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

/// Product category, (de)serialized as the original `meta.type` string. The
/// variants are the distinct JP `meta.type` values observed in the data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MinisForumJpProductType {
    #[serde(rename = "Mini PC")]
    MiniPc,
    #[serde(rename = "Refurbished Mini PC")]
    RefurbishedMiniPc,
    #[serde(rename = "Mini WorkStation")]
    MiniWorkStation,
    #[serde(rename = "Refurbished Mini WorkStation")]
    RefurbishedMiniWorkStation,
    #[serde(rename = "Game PC")]
    GamePc,
    #[serde(rename = "motherboard")]
    Motherboard,
    #[serde(rename = "eGPU Dock")]
    EgpuDock,
    #[serde(rename = "NAS")]
    Nas,
    #[serde(rename = "adapter")]
    Adapter,
    #[serde(rename = "keyboards")]
    Keyboards,
    #[serde(rename = "mousepads")]
    Mousepads,
    #[serde(rename = "Screen Protectors")]
    ScreenProtectors,
    #[serde(rename = "Bag")]
    Bag,
    #[serde(rename = "Cup")]
    Cup,
    #[serde(rename = "gift")]
    Gift,
    #[serde(rename = "shipping-protection")]
    ShippingProtection,
}

impl MinisForumJpProductType {
    /// Builds the variant from the destructured `type` string. Errors on any
    /// value not seen in the data.
    pub fn from_string(product_type: &str) -> Result<Self, String> {
        match product_type {
            "Mini PC" => Ok(MinisForumJpProductType::MiniPc),
            "Refurbished Mini PC" => Ok(MinisForumJpProductType::RefurbishedMiniPc),
            "Mini WorkStation" => Ok(MinisForumJpProductType::MiniWorkStation),
            "Refurbished Mini WorkStation" => {
                Ok(MinisForumJpProductType::RefurbishedMiniWorkStation)
            }
            "Game PC" => Ok(MinisForumJpProductType::GamePc),
            "motherboard" => Ok(MinisForumJpProductType::Motherboard),
            "eGPU Dock" => Ok(MinisForumJpProductType::EgpuDock),
            "NAS" => Ok(MinisForumJpProductType::Nas),
            "adapter" => Ok(MinisForumJpProductType::Adapter),
            "keyboards" => Ok(MinisForumJpProductType::Keyboards),
            "mousepads" => Ok(MinisForumJpProductType::Mousepads),
            "Screen Protectors" => Ok(MinisForumJpProductType::ScreenProtectors),
            "Bag" => Ok(MinisForumJpProductType::Bag),
            "Cup" => Ok(MinisForumJpProductType::Cup),
            "gift" => Ok(MinisForumJpProductType::Gift),
            "shipping-protection" => Ok(MinisForumJpProductType::ShippingProtection),
            other => Err(format!("unexpected product type: {other:?}")),
        }
    }
}

/// A product id. Built from `meta.id`, (de)serialized as a number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MinisForumJpProductId(pub u64);

impl MinisForumJpProductId {
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
pub enum MinisForumJpProcessedAvailability {
    Available,
    Unavailable,
}

impl From<bool> for MinisForumJpProcessedAvailability {
    fn from(available: bool) -> Self {
        if available {
            MinisForumJpProcessedAvailability::Available
        } else {
            MinisForumJpProcessedAvailability::Unavailable
        }
    }
}

impl From<MinisForumJpProcessedAvailability> for bool {
    fn from(availability: MinisForumJpProcessedAvailability) -> Self {
        availability == MinisForumJpProcessedAvailability::Available
    }
}

impl From<MinisForumJpAvailability> for MinisForumJpProcessedAvailability {
    fn from(availability: MinisForumJpAvailability) -> Self {
        match availability {
            MinisForumJpAvailability::InStock => MinisForumJpProcessedAvailability::Available,
            MinisForumJpAvailability::OutOfStock => MinisForumJpProcessedAvailability::Unavailable,
        }
    }
}

impl TryFrom<MinisForumJpDestructuredProduct> for MinisForumJpProcessedProduct {
    type Error = String;

    fn try_from(destructured: MinisForumJpDestructuredProduct) -> Result<Self, Self::Error> {
        // The first JSON-LD schema that carries an offer is the Product schema —
        // the source of sku/brand and the single price/availability offer.
        let product_schema = destructured
            .schemas
            .iter()
            .find(|schema| !schema.offers.is_empty())
            .cloned()
            .ok_or_else(|| "missing product schema with an offer".to_string())?;

        let title = destructured
            .main_product
            .as_ref()
            .map(|main| main.title.clone());

        let product = make_product_info(&destructured.meta, &product_schema, title)?;

        let variants = make_variants(
            destructured.meta.variants,
            &product_schema,
            &destructured.product_variants,
        )?;

        Ok(Self {
            locale: MinisForumJpLocale::from_string(&destructured.locale)?,
            product,
            // `xxxx` is absent on 1/67 pages → empty images.
            images: destructured
                .main_product
                .map(|main| main.gallery.media.into_iter().map(|m| m.src).collect())
                .unwrap_or_default(),
            features: destructured
                .feature_chart
                .map(|chart| {
                    chart
                        .features
                        .into_iter()
                        .flatten()
                        .map(Into::into)
                        .collect()
                })
                .unwrap_or_default(),
            variants,
        })
    }
}

/// Builds the product core from `meta` (identity) and the Product `schema`
/// (sku/brand) plus its single offer (price/availability).
fn make_product_info(
    meta: &super::destructured::MinisForumJpMeta,
    schema: &MinisForumJpSchema,
    title: Option<String>,
) -> Result<MinisForumJpProcessedProductInfo, String> {
    let offer = schema
        .offers
        .first()
        .ok_or_else(|| "product schema has no offer".to_string())?;
    let currency = currency_from_code(&offer.currency)?;
    let price = Money::new(dollars_to_cents(&offer.price)?, currency);

    Ok(MinisForumJpProcessedProductInfo {
        id: MinisForumJpProductId::from_string(&meta.id)?,
        handle: meta.handle.clone(),
        vendor: meta.vendor.clone(),
        title,
        product_type: meta
            .product_type
            .as_deref()
            .map(MinisForumJpProductType::from_string)
            .transpose()?,
        sku: schema.sku.clone(),
        brand: schema.brand.clone(),
        price,
        availability: offer.availability.into(),
    })
}

/// Combines `meta.variants` with the page's single offer by SKU and enriches
/// with `product_variants` data (`option1/2/3`, `compare_at_price`) when a
/// matching PV variant exists.
///
/// The offer carries no sku of its own, so it is attributed to the meta variant
/// whose `sku` equals the schema `sku` (the representative/default variant); all
/// other meta variants get no offer-derived fields. When this variant carries the
/// offer, the offer's price is guarded to equal the meta cents price.
fn make_variants(
    meta_variants: Vec<MinisForumJpMetaVariant>,
    schema: &MinisForumJpSchema,
    product_variants: &[MinisForumJpProductVariant],
) -> Result<Vec<ZzzVariant>, String> {
    let offer = schema.offers.first();
    let schema_sku = schema.sku.as_deref();

    meta_variants
        .into_iter()
        .map(|meta| {
            // Attribute the single offer to the meta variant matching the schema
            // sku (both non-null and equal).
            let carries_offer = match (schema_sku, meta.sku.as_deref()) {
                (Some(s), Some(m)) => s == m,
                _ => false,
            };
            // Find the matching product_variant by SKU for option enrichment.
            let pv = meta.sku.as_deref().and_then(|sku| {
                product_variants
                    .iter()
                    .find(|pv| pv.sku.as_deref() == Some(sku))
            });
            make_variant(meta, if carries_offer { offer } else { None }, pv)
        })
        .collect()
}

/// Builds one [`ZzzVariant`] from a meta variant, an optional offer, and an
/// optional `product_variants` entry. The offer provides availability and
/// `price_valid_until`; the PV provides `option1/2/3` and `compare_at_price`.
fn make_variant(
    meta: MinisForumJpMetaVariant,
    offer: Option<&MinisForumJpOffer>,
    pv: Option<&MinisForumJpProductVariant>,
) -> Result<ZzzVariant, String> {
    let meta_price = Money::new(parse_cents(&meta.price)?, Currency::JPY);

    let (availability, price_valid_until) = match offer {
        Some(offer) => {
            let offer_currency = currency_from_code(&offer.currency)?;
            let offer_price = Money::new(dollars_to_cents(&offer.price)?, offer_currency);
            if offer_price != meta_price {
                return Err(format!(
                    "variant price mismatch for sku {:?}: meta={meta_price:?}, offer={offer_price:?}",
                    meta.sku
                ));
            }
            let valid_until = offer
                .price_valid_until
                .as_deref()
                .map(|date| {
                    date.parse::<NaiveDate>()
                        .map_err(|error| format!("invalid price_valid_until {date:?}: {error}"))
                })
                .transpose()?;
            (Some(offer.availability.into()), valid_until)
        }
        None => (None, None),
    };

    let (option1, option2, option3, compare_at_price) = match pv {
        Some(pv) => {
            let cat = pv
                .compare_at_price
                .as_deref()
                .map(parse_cents)
                .transpose()?
                .map(|cents| Money::new(cents, Currency::JPY));
            (
                pv.option1.clone(),
                pv.option2.clone(),
                pv.option3.clone(),
                cat,
            )
        }
        None => (None, None, None, None),
    };

    Ok(ZzzVariant {
        sku: meta.sku,
        price: meta_price,
        availability,
        title: meta.title,
        price_valid_until,
        meta_variant_id: meta.variant_id,
        option1,
        option2,
        option3,
        compare_at_price,
    })
}
