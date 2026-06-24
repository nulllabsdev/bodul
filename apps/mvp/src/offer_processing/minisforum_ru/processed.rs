//! Processed MinisForum RU product model.
//!
//! The output of `process_products`: built up field by field by mapping from the
//! [`MinisForumRuDestructuredProduct`] of the previous step into our own shape.
//!
//! The RU store is **JSON-only / sparse**, so this model is a heavily adapted
//! version of the AU one:
//!
//! - There is **no** `images` (no gallery DOM) and **no** `features` (no
//!   `feature_chart`) — both AU root keys are dropped.
//! - There is no Shopify `const product` object, so the product **core** is
//!   built from `meta` alone (`id`, `handle`, `vendor`). `meta` carries no
//!   title/price/availability/type, so those product-level fields do not exist
//!   here.
//! - `variants` combines **two** sources — `meta.variants` and the JSON-LD
//!   `schema.offers` — instead of AU's three (there is no product-object variant
//!   list). RU SKUs are almost always `null`, so the two sources are paired
//!   **by position** (index) rather than by SKU, guarded by an equal-length
//!   check and a per-pair SKU/price agreement check.
//! - One file (`products-minisforum-n5-max-ai-nas.json`) has **no schemas and
//!   thus no offers**. Rather than failing the file, it is handled gracefully:
//!   its variants are built from `meta` alone, with the offer-only fields
//!   (`availability`, `price_valid_until`) left `None`. This is the documented
//!   degraded path for a schema-less / offer-less page.

use chrono::NaiveDate;
use money::{Currency, Money};

use super::destructured::{
    MinisForumRuAvailability, MinisForumRuDestructuredProduct, MinisForumRuMeta,
    MinisForumRuMetaVariant, MinisForumRuOffer, MinisForumRuProductVariant,
};

/// A processed MinisForum RU product.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumRuProcessedProduct {
    /// Page locale.
    pub locale: MinisForumRuLocale,
    /// The core product fields (from `meta`).
    pub product: MinisForumRuProcessedProductInfo,
    /// Product image URLs, from the `motion_main` gallery (`src` only). Empty
    /// when the page has no Motion theme DOM.
    pub images: Vec<String>,
    /// Combined variants (meta-variant + JSON-LD offer, paired by position).
    pub variants: Vec<MinisForumRuVariant>,
}

/// One variant combining the `meta`-variant and JSON-LD offer rows that occupy
/// the same position in their lists. Each field is prefixed with its source
/// (`meta_`/`offer_`) where the provenance would otherwise be ambiguous.
///
/// The offer-derived fields (`availability`, `price_valid_until`) are `Option`
/// because a schema-less page has no offers — those variants are built from
/// `meta` alone.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumRuVariant {
    /// SKU — when both sources carry one they must agree; often `null` on RU.
    pub sku: Option<String>,
    /// Price (USD). When an offer is present its price must agree with `meta`'s.
    #[serde(with = "money_wire")]
    pub price: Money,
    /// Availability — from the offer; `None` when the page has no offers.
    pub availability: Option<MinisForumRuProcessedAvailability>,
    /// Variant title — from the `meta` variant (RU offers carry no name).
    pub title: Option<String>,
    /// From the offer; `None` when the page has no offers.
    pub price_valid_until: Option<NaiveDate>,
    /// The `meta` analytics variant id (distinct from SKU).
    pub meta_variant_id: String,
    /// Variant option values — enriched from `product_variants` textarea by position.
    pub option1: Option<String>,
    pub option2: Option<String>,
    pub option3: Option<String>,
    /// Compare-at price — enriched from `product_variants` textarea by position (in cents).
    #[serde(with = "option_money_wire")]
    pub compare_at_price: Option<Money>,
}

/// Page locale. The RU store is English-only; (de)serialized as the locale code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MinisForumRuLocale {
    #[serde(rename = "en")]
    En,
}

impl MinisForumRuLocale {
    /// Builds the variant from the destructured locale code. Errors on any code
    /// other than "en".
    pub fn from_string(locale: &str) -> Result<Self, String> {
        match locale {
            "en" => Ok(MinisForumRuLocale::En),
            other => Err(format!("unexpected locale: {other:?}")),
        }
    }
}

/// The mapped product core — built from `meta`.
///
/// RU has no Shopify product object, so unlike AU there is no product-level
/// `title`, `price`, `compare_at_price`, `availability` or `type`: `meta`
/// simply does not carry them. The per-variant price/availability live on
/// [`MinisForumRuVariant`] instead.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumRuProcessedProductInfo {
    pub id: MinisForumRuProductId,
    pub handle: String,
    pub vendor: String,
    /// Product title, from `motion_main.title` when available.
    pub title: Option<String>,
}

impl TryFrom<MinisForumRuMeta> for MinisForumRuProcessedProductInfo {
    type Error = String;

    fn try_from(meta: MinisForumRuMeta) -> Result<Self, Self::Error> {
        Ok(Self {
            id: MinisForumRuProductId::from_string(&meta.id)?,
            handle: meta.handle,
            vendor: meta.vendor,
            title: None,
        })
    }
}

impl MinisForumRuProcessedProductInfo {
    fn with_title(mut self, title: Option<String>) -> Self {
        self.title = title;
        self
    }
}

/// Parses a destructured cents string (e.g. "98700") into a minor-unit count.
/// Errors if the string is not a valid integer.
fn parse_cents(price: &str) -> Result<i64, String> {
    price
        .parse()
        .map_err(|error| format!("invalid price {price:?}: {error}"))
}

/// Converts a major-unit dollar string (e.g. "987.0", "25.90") into a minor-unit
/// (cents) count. Errors on a non-numeric value or more than two fractional
/// digits (USD has a minor-unit exponent of 2; we do not round).
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

/// The canonical [`Money`] wire shape: `{ "amount_minor": "98700", "currency":
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

/// A product id. Built from the destructured id string, (de)serialized as a
/// number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MinisForumRuProductId(pub u64);

impl MinisForumRuProductId {
    /// Builds the id by parsing the destructured id string (e.g.
    /// "8763028766898"). Errors if the string is not a valid number.
    pub fn from_string(id: &str) -> Result<Self, String> {
        id.parse()
            .map(Self)
            .map_err(|error| format!("invalid product id {id:?}: {error}"))
    }
}

/// Product stock status. (De)serialized as a JSON boolean (`true` = available).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(into = "bool", from = "bool")]
pub enum MinisForumRuProcessedAvailability {
    Available,
    Unavailable,
}

impl From<bool> for MinisForumRuProcessedAvailability {
    fn from(available: bool) -> Self {
        if available {
            MinisForumRuProcessedAvailability::Available
        } else {
            MinisForumRuProcessedAvailability::Unavailable
        }
    }
}

impl From<MinisForumRuProcessedAvailability> for bool {
    fn from(availability: MinisForumRuProcessedAvailability) -> Self {
        availability == MinisForumRuProcessedAvailability::Available
    }
}

impl From<MinisForumRuAvailability> for MinisForumRuProcessedAvailability {
    fn from(availability: MinisForumRuAvailability) -> Self {
        match availability {
            MinisForumRuAvailability::InStock => MinisForumRuProcessedAvailability::Available,
            MinisForumRuAvailability::OutOfStock => MinisForumRuProcessedAvailability::Unavailable,
        }
    }
}

impl TryFrom<MinisForumRuDestructuredProduct> for MinisForumRuProcessedProduct {
    type Error = String;

    fn try_from(destructured: MinisForumRuDestructuredProduct) -> Result<Self, Self::Error> {
        // RU schema list is Product-only (one schema, or none). Flatten every
        // schema's offers into one offer list — the variant source.
        let offers: Vec<MinisForumRuOffer> = destructured
            .schemas
            .into_iter()
            .flat_map(|schema| schema.offers)
            .collect();

        let variants = make_variants(
            destructured.meta.variants.clone(),
            offers,
            &destructured.product_variants,
        )?;

        let title = destructured
            .motion_main
            .as_ref()
            .and_then(|main| main.title.clone());

        let images = destructured
            .motion_main
            .map(|main| main.media.into_iter().filter_map(|m| m.src).collect())
            .unwrap_or_default();

        Ok(Self {
            locale: MinisForumRuLocale::from_string(&destructured.locale)?,
            product: MinisForumRuProcessedProductInfo::try_from(destructured.meta)?
                .with_title(title),
            images,
            variants,
        })
    }
}

/// Combines the two variant sources into [`MinisForumRuVariant`]s, enriched by
/// `product_variants` textarea data matched by position.
///
/// RU SKUs are almost always `null`, so the sources are paired **by position**.
/// When offers are present they must be equal in count to the meta variants;
/// when offers are absent (a schema-less page) the variants are built from
/// `meta` alone, with the offer-only fields left `None`.
fn make_variants(
    meta_variants: Vec<MinisForumRuMetaVariant>,
    offers: Vec<MinisForumRuOffer>,
    product_variants: &[MinisForumRuProductVariant],
) -> Result<Vec<MinisForumRuVariant>, String> {
    // No offers at all (e.g. the schema-less file): build from meta only.
    if offers.is_empty() {
        return meta_variants
            .into_iter()
            .enumerate()
            .map(|(i, meta)| make_variant_meta_only(meta, product_variants.get(i)))
            .collect();
    }

    // Otherwise both sources must line up one-to-one by position.
    if meta_variants.len() != offers.len() {
        return Err(format!(
            "variant source length mismatch: meta={}, offers={}",
            meta_variants.len(),
            offers.len(),
        ));
    }

    meta_variants
        .into_iter()
        .enumerate()
        .zip(offers)
        .map(|((i, meta), offer)| make_variant(meta, offer, product_variants.get(i)))
        .collect()
}

/// Builds a variant from a `meta` variant alone (no offer): a degraded page with
/// no schema/offers. Offer-only fields are left `None`. The price is `meta`'s
/// cents value. Enriched by `product_variants` textarea data by position.
fn make_variant_meta_only(
    meta: MinisForumRuMetaVariant,
    pv: Option<&MinisForumRuProductVariant>,
) -> Result<MinisForumRuVariant, String> {
    let (option1, option2, option3, compare_at_price) = extract_options(pv);
    Ok(MinisForumRuVariant {
        sku: meta.sku,
        price: Money::new(parse_cents(&meta.price)?, Currency::USD),
        availability: None,
        title: meta.title,
        price_valid_until: None,
        meta_variant_id: meta.variant_id,
        option1,
        option2,
        option3,
        compare_at_price,
    })
}

/// Combines one positionally-matched (`meta` variant, offer) pair into a
/// [`MinisForumRuVariant`].
///
/// Fields present in both sources are guarded:
/// - `sku` — when both carry one they must agree (often both `null`).
/// - `price` — `meta` cents must equal the offer's dollars→cents value.
/// Enriched by `product_variants` textarea data by position.
fn make_variant(
    meta: MinisForumRuMetaVariant,
    offer: MinisForumRuOffer,
    pv: Option<&MinisForumRuProductVariant>,
) -> Result<MinisForumRuVariant, String> {
    // When both sources carry a SKU they must agree; lift it to one field.
    if let (Some(meta_sku), Some(offer_sku)) = (&meta.sku, &offer.sku) {
        if meta_sku != offer_sku {
            return Err(format!(
                "variant sku mismatch: meta={meta_sku:?}, offer={offer_sku:?}"
            ));
        }
    }

    let offer_currency = currency_from_code(&offer.currency)?;

    // Both sources must agree on the price; lift it to a single field.
    let meta_price = Money::new(parse_cents(&meta.price)?, Currency::USD);
    let offer_price = Money::new(dollars_to_cents(&offer.price)?, offer_currency);
    if meta_price != offer_price {
        return Err(format!(
            "variant price mismatch: meta={meta_price:?}, offer={offer_price:?}"
        ));
    }

    let (option1, option2, option3, compare_at_price) = extract_options(pv);

    Ok(MinisForumRuVariant {
        // Prefer the offer SKU when present, else the meta SKU (they agree when
        // both present).
        sku: offer.sku.or(meta.sku),
        price: meta_price,
        availability: Some(offer.availability.into()),
        title: meta.title,
        price_valid_until: Some(offer.price_valid_until.parse().map_err(|error| {
            format!(
                "invalid price_valid_until {:?}: {error}",
                offer.price_valid_until
            )
        })?),
        meta_variant_id: meta.variant_id,
        option1,
        option2,
        option3,
        compare_at_price,
    })
}

/// Extracts option/compare-at values from a `product_variants` textarea entry.
fn extract_options(
    pv: Option<&MinisForumRuProductVariant>,
) -> (
    Option<String>,
    Option<String>,
    Option<String>,
    Option<Money>,
) {
    let Some(pv) = pv else {
        return (None, None, None, None);
    };
    let cat = pv
        .compare_at_price
        .as_deref()
        .and_then(|price| parse_cents(price).ok())
        .map(|cents| Money::new(cents, Currency::USD));
    (
        pv.option1.clone(),
        pv.option2.clone(),
        pv.option3.clone(),
        cat,
    )
}
