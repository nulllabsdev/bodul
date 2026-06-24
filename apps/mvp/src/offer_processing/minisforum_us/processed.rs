//! Processed MinisForum US product model.
//!
//! The output of `process_products`: built up field by field by mapping from the
//! [`MinisForumUsDestructuredProduct`] of the previous step into our own shape.
//!
//! ## How this differs from AU
//!
//! - **No main-product DOM (`xxxx`)** on US pages, so there is **no gallery
//!   source for `images`**. We keep the `images: Vec<String>` field for
//!   shape-consistency with the AU processed product, but it is **always empty**
//!   (`product.media` is intentionally not mapped, exactly as in AU). See the
//!   field doc on [`MinisForumUsProcessedProduct::images`].
//! - **Locale** may be `en` or `es` (the US store serves a Spanish locale too).
//! - **Currency** may be `USD` or `CAD` (Canadian-locale pages quote CAD). The
//!   page currency is taken from `viewed_product.currency`.
//! - **JSON-LD offers carry no `sku` and no `name`**, and their count does not
//!   line up with the variants, so they **cannot** be joined per-variant by SKU
//!   the way AU does. Instead the combined `variants` are built from the
//!   product-object and `meta` variants (which align 1:1 by SKU), and the schema
//!   offers are surfaced separately at the product level (see `offers`).

use chrono::NaiveDate;
use money::{Currency, Money};

use super::destructured::{
    MinisForumUsAvailability, MinisForumUsDestructuredProduct, MinisForumUsFeature,
    MinisForumUsMetaVariant, MinisForumUsOffer, MinisForumUsProduct, MinisForumUsProductVariant,
};

/// A processed MinisForum US product.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumUsProcessedProduct {
    /// Page locale.
    pub locale: MinisForumUsLocale,
    /// The core product fields.
    pub product: MinisForumUsProcessedProductInfo,
    /// Product image URLs.
    ///
    /// **Always empty for US.** AU sources these from the main-product DOM
    /// (`xxxx`) gallery, which the US pages do not have. The field is kept (rather
    /// than omitted) for shape-consistency with the AU processed product;
    /// `product.media` is intentionally not used as a substitute (AU drops it for
    /// the same reason).
    pub images: Vec<String>,
    /// Specification rows, from `feature_chart` (the column-major chart is
    /// flattened). Empty when the page has no feature chart. The chart's `h1`/`h2`
    /// headings are not mapped.
    pub features: Vec<MinisForumUsProcessedFeature>,
    /// Combined product + `meta` variants, joined by SKU.
    pub variants: Vec<MinisForumUsVariant>,
    /// The JSON-LD offers for this product, mapped 1:1 from the first schema's
    /// `offers`. Unlike AU these have no SKU/name and do not align with the
    /// variants, so they are surfaced here at the product level rather than folded
    /// into each variant.
    pub offers: Vec<MinisForumUsProcessedOffer>,
}

/// One product specification: a `label` and its `value` lines (the raw cell text
/// split on `\n`, with blank/whitespace-only lines dropped).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumUsProcessedFeature {
    pub label: String,
    pub value: Vec<String>,
}

impl From<MinisForumUsFeature> for MinisForumUsProcessedFeature {
    fn from(feature: MinisForumUsFeature) -> Self {
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

/// One variant combining the product-variant and `meta`-variant rows that share a
/// SKU. (US JSON-LD offers have no SKU and are surfaced at the product level
/// instead — see [`MinisForumUsProcessedProduct::offers`].) Fields guaranteed
/// identical across the two sources are lifted to a single field and guarded.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumUsVariant {
    /// SKU — guaranteed identical across both sources.
    pub sku: Option<String>,
    /// Price — guaranteed identical across both sources.
    #[serde(with = "money_wire")]
    pub price: Money,
    /// Availability — from the product object's variant.
    pub availability: MinisForumUsProcessedAvailability,
    // From the product object's variant.
    pub option1: String,
    pub option2: Option<String>,
    pub option3: Option<String>,
    #[serde(with = "option_money_wire")]
    pub compare_at_price: Option<Money>,
    /// Variant title — from the `meta` analytics variant (the product object's
    /// variant has no title; US offers carry none either).
    pub title: Option<String>,
    // From the `meta` analytics variant.
    pub meta_variant_id: String,
}

/// A processed JSON-LD offer.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumUsProcessedOffer {
    #[serde(with = "money_wire")]
    pub price: Money,
    pub availability: MinisForumUsProcessedAvailability,
    pub price_valid_until: Option<NaiveDate>,
}

/// Page locale. The US store serves English and Spanish; (de)serialized as the
/// locale code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MinisForumUsLocale {
    #[serde(rename = "en")]
    En,
    #[serde(rename = "es")]
    Es,
}

impl MinisForumUsLocale {
    /// Builds the variant from the destructured locale code. Errors on any code
    /// other than "en"/"es".
    pub fn from_string(locale: &str) -> Result<Self, String> {
        match locale {
            "en" => Ok(MinisForumUsLocale::En),
            "es" => Ok(MinisForumUsLocale::Es),
            other => Err(format!("unexpected locale: {other:?}")),
        }
    }
}

/// The mapped subset of the full destructured product object.
///
/// Intentionally not mapped:
/// - `price_min` / `price_max` — we don't track the variant price range.
/// - `media` — see [`MinisForumUsProcessedProduct::images`] for why US images are
///   empty.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumUsProcessedProductInfo {
    pub id: MinisForumUsProductId,
    pub handle: String,
    pub title: String,
    pub vendor: String,
    #[serde(rename = "type")]
    pub product_type: Option<MinisForumUsProductType>,
    #[serde(with = "money_wire")]
    pub price: Money,
    #[serde(with = "option_money_wire")]
    pub compare_at_price: Option<Money>,
    pub availability: MinisForumUsProcessedAvailability,
}

/// Parses a destructured cents string (e.g. "5629") into a minor-unit count.
/// Errors if the string is not a valid integer.
fn parse_cents(price: &str) -> Result<i64, String> {
    price
        .parse()
        .map_err(|error| format!("invalid price {price:?}: {error}"))
}

/// Converts a major-unit string (e.g. "56.29", "959.0") into a minor-unit (cents)
/// count. Errors on a non-numeric value or more than two fractional digits
/// (USD/CAD both have a minor-unit exponent of 2; we do not round).
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

/// The canonical [`Money`] wire shape: `{ "amount_minor": "5629", "currency":
/// "USD" }` (amount as a base-10 string so it round-trips exactly).
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

/// Product category, (de)serialized as the original `type` string. Values are
/// those observed in the US data (re-surveyed; note the source typo
/// `"Accesorries"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MinisForumUsProductType {
    #[serde(rename = "Mini PC")]
    MiniPc,
    #[serde(rename = "Work Station")]
    WorkStation,
    #[serde(rename = "Gift Guide")]
    GiftGuide,
    // Note: the source data misspells "Accessories".
    #[serde(rename = "Accesorries")]
    Accessories,
    #[serde(rename = "Motherboards")]
    Motherboards,
    #[serde(rename = "Docking Station")]
    DockingStation,
    #[serde(rename = "NAS")]
    Nas,
}

impl MinisForumUsProductType {
    /// Builds the variant from the destructured `type` string. Errors on any
    /// value not seen in the data.
    pub fn from_string(product_type: &str) -> Result<Self, String> {
        match product_type {
            "Mini PC" => Ok(MinisForumUsProductType::MiniPc),
            "Work Station" => Ok(MinisForumUsProductType::WorkStation),
            "Gift Guide" => Ok(MinisForumUsProductType::GiftGuide),
            "Accesorries" => Ok(MinisForumUsProductType::Accessories),
            "Motherboards" => Ok(MinisForumUsProductType::Motherboards),
            "Docking Station" => Ok(MinisForumUsProductType::DockingStation),
            "NAS" => Ok(MinisForumUsProductType::Nas),
            other => Err(format!("unexpected product type: {other:?}")),
        }
    }
}

/// A product id. Built from the destructured id string, (de)serialized as a
/// number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MinisForumUsProductId(pub u64);

impl MinisForumUsProductId {
    /// Builds the id by parsing the destructured id string (e.g. "8239381807349").
    /// Errors if the string is not a valid number.
    pub fn from_string(id: &str) -> Result<Self, String> {
        id.parse()
            .map(Self)
            .map_err(|error| format!("invalid product id {id:?}: {error}"))
    }
}

/// Product stock status, normalized from the destructured `available`
/// ("true"/"false") flag. (De)serialized as a JSON boolean (`true` = available).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(into = "bool", from = "bool")]
pub enum MinisForumUsProcessedAvailability {
    Available,
    Unavailable,
}

impl MinisForumUsProcessedAvailability {
    /// Builds the variant from the destructured `available` flag, matching the
    /// string directly. Errors on any value other than "true"/"false".
    pub fn from_string(available: &str) -> Result<Self, String> {
        match available {
            "true" => Ok(MinisForumUsProcessedAvailability::Available),
            "false" => Ok(MinisForumUsProcessedAvailability::Unavailable),
            other => Err(format!("unexpected availability flag: {other:?}")),
        }
    }
}

impl From<bool> for MinisForumUsProcessedAvailability {
    fn from(available: bool) -> Self {
        if available {
            MinisForumUsProcessedAvailability::Available
        } else {
            MinisForumUsProcessedAvailability::Unavailable
        }
    }
}

impl From<MinisForumUsProcessedAvailability> for bool {
    fn from(availability: MinisForumUsProcessedAvailability) -> Self {
        availability == MinisForumUsProcessedAvailability::Available
    }
}

impl From<MinisForumUsAvailability> for MinisForumUsProcessedAvailability {
    fn from(availability: MinisForumUsAvailability) -> Self {
        match availability {
            MinisForumUsAvailability::InStock => MinisForumUsProcessedAvailability::Available,
            MinisForumUsAvailability::OutOfStock => MinisForumUsProcessedAvailability::Unavailable,
        }
    }
}

impl TryFrom<MinisForumUsDestructuredProduct> for MinisForumUsProcessedProduct {
    type Error = String;

    fn try_from(destructured: MinisForumUsDestructuredProduct) -> Result<Self, Self::Error> {
        // The page currency: USD on US-locale pages, CAD on Canadian-locale
        // pages. The product/variant prices carry no currency annotation, so we
        // take it from `viewed_product`, which always quotes the page currency.
        let currency = currency_from_code(&destructured.viewed_product.currency)?;

        let variants = make_variants(
            destructured.product.variants.clone(),
            destructured.meta.variants.clone(),
            currency,
        )?;

        // The first schema (Product) is the source of the offers.
        let offers = destructured
            .schemas
            .into_iter()
            .next()
            .map(|schema| {
                schema
                    .offers
                    .into_iter()
                    .map(MinisForumUsProcessedOffer::try_from)
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?
            .unwrap_or_default();

        Ok(Self {
            locale: MinisForumUsLocale::from_string(&destructured.locale)?,
            product: process_product_info(destructured.product, currency)?,
            // US pages have no main-product DOM gallery, so there is no image
            // source; kept empty for shape-consistency with AU.
            images: Vec::new(),
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
            offers,
        })
    }
}

impl TryFrom<MinisForumUsOffer> for MinisForumUsProcessedOffer {
    type Error = String;

    fn try_from(offer: MinisForumUsOffer) -> Result<Self, Self::Error> {
        let currency = currency_from_code(&offer.currency)?;
        Ok(Self {
            price: Money::new(dollars_to_cents(&offer.price)?, currency),
            availability: offer.availability.into(),
            price_valid_until: offer
                .price_valid_until
                .as_deref()
                .map(|date| {
                    date.parse::<NaiveDate>()
                        .map_err(|error| format!("invalid price_valid_until {date:?}: {error}"))
                })
                .transpose()?,
        })
    }
}

/// Combines the product-object and `meta` variant lists into the merged
/// [`MinisForumUsVariant`] list. The two sources align 1:1 by SKU; a length
/// mismatch is a hard error.
fn make_variants(
    product_variants: Vec<MinisForumUsProductVariant>,
    meta_variants: Vec<MinisForumUsMetaVariant>,
    currency: Currency,
) -> Result<Vec<MinisForumUsVariant>, String> {
    // Guard the two sources have the same length.
    if product_variants.len() != meta_variants.len() {
        return Err(format!(
            "variant source length mismatch: product={}, meta={}",
            product_variants.len(),
            meta_variants.len(),
        ));
    }

    // For each product variant, find the `meta` variant with the same SKU and
    // combine the two into one `MinisForumUsVariant`.
    let mut combined = Vec::with_capacity(product_variants.len());
    for product in product_variants {
        let meta = meta_variants
            .iter()
            .find(|meta| meta.sku == product.sku)
            .cloned()
            .ok_or_else(|| format!("no meta variant for sku {:?}", product.sku))?;
        combined.push(make_variant(product, meta, currency)?);
    }

    Ok(combined)
}

/// Combines one matched (product variant, `meta` variant) pair into a
/// [`MinisForumUsVariant`], applying the same per-field conversions as elsewhere.
fn make_variant(
    product: MinisForumUsProductVariant,
    meta: MinisForumUsMetaVariant,
    currency: Currency,
) -> Result<MinisForumUsVariant, String> {
    // Both sources must agree on the SKU; lift it to a single field.
    if product.sku != meta.sku {
        return Err(format!(
            "variant sku mismatch: product={:?}, meta={:?}",
            product.sku, meta.sku
        ));
    }

    // Both sources must agree on the price; lift it to a single field.
    let product_price = Money::new(parse_cents(&product.price)?, currency);
    let meta_price = Money::new(parse_cents(&meta.price)?, currency);
    if product_price != meta_price {
        return Err(format!(
            "variant price mismatch: product={product_price:?}, meta={meta_price:?}"
        ));
    }

    Ok(MinisForumUsVariant {
        sku: product.sku,
        price: product_price,
        availability: MinisForumUsProcessedAvailability::from_string(&product.available)?,
        option1: product.option1,
        option2: product.option2,
        option3: product.option3,
        compare_at_price: product
            .compare_at_price
            .as_deref()
            .map(parse_cents)
            .transpose()?
            .map(|cents| Money::new(cents, currency)),
        title: meta.title.filter(|title| !title.is_empty()),
        meta_variant_id: meta.variant_id,
    })
}

/// Maps the full destructured product object into the processed product info,
/// pricing it in the supplied page currency.
fn process_product_info(
    product: MinisForumUsProduct,
    currency: Currency,
) -> Result<MinisForumUsProcessedProductInfo, String> {
    Ok(MinisForumUsProcessedProductInfo {
        id: MinisForumUsProductId::from_string(&product.id)?,
        handle: product.handle,
        title: product.title,
        vendor: product.vendor,
        product_type: product
            .product_type
            .as_deref()
            .map(MinisForumUsProductType::from_string)
            .transpose()?,
        price: Money::new(parse_cents(&product.price)?, currency),
        compare_at_price: product
            .compare_at_price
            .as_deref()
            .map(parse_cents)
            .transpose()?
            .map(|cents| Money::new(cents, currency)),
        availability: MinisForumUsProcessedAvailability::from_string(&product.available)?,
    })
}
