//! Processed MinisForum KR product model.
//!
//! The output of `process_products` for the KR store: built up field by field by
//! mapping from the [`MinisForumKrDestructuredProduct`] of the previous step into
//! our own shape.
//!
//! KR is the **JSON-only / SPARSE** store: there is no `const product` object, no
//! gallery and no feature chart, so this shape deliberately drops the AU
//! `images` and `features` root keys. The product core is assembled from the
//! `meta` block (ids/handle/vendor/type) plus the JSON-LD `schemas` (sku/brand
//! and, when present, the single offer's price/availability). `variants` is a
//! left-join of `meta.variants` enriched by any JSON-LD offer that shares the
//! SKU.

use chrono::NaiveDate;
use money::{Currency, Money};

use super::destructured::{
    MinisForumKrAvailability, MinisForumKrDestructuredProduct, MinisForumKrMetaVariant, MinisForumKrOffer,
    MinisForumKrProductVariant, MinisForumKrSchema,
};

/// A processed MinisForum KR product.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumKrProcessedProduct {
    /// Page locale.
    pub locale: MinisForumKrLocale,
    /// The core product fields.
    pub product: MinisForumKrProcessedProductInfo,
    /// Product image URLs, from the `dawn_main` gallery (`src` only). Empty when
    /// the page has no Dawn theme DOM.
    pub images: Vec<String>,
    /// The combined variants (`meta.variants` left-joined with any schema offer,
    /// enriched by `product_variants`).
    pub variants: Vec<MinisForumKrVariant>,
}

/// One processed variant.
///
/// Built from a `meta` analytics variant, optionally enriched by the single
/// JSON-LD `offer` that shares its SKU. Fields that can be sourced from more than
/// one block (`price`, `availability`, `title`) are lifted to a single field with
/// a guard; `meta_variant_id` and `price_valid_until` keep their source prefix /
/// provenance.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumKrVariant {
    /// SKU — from the `meta` variant; when an offer is matched it shares this SKU.
    pub sku: Option<String>,
    /// Price — from the `meta` variant (minor units). When an offer is present
    /// for the SKU, the offer price must agree.
    #[serde(with = "money_wire")]
    pub price: Money,
    /// Availability — only known when a matching offer is present (the `meta`
    /// block carries no stock flag), so it is optional.
    pub availability: Option<MinisForumKrProcessedAvailability>,
    /// Variant title — the `meta` variant title when present and non-empty.
    pub title: Option<String>,
    /// `price_valid_until` — only an offer carries it, and only sometimes.
    pub price_valid_until: Option<NaiveDate>,
    /// The `meta` analytics variant id (distinct from the SKU).
    pub meta_variant_id: String,
    /// Variant option values — enriched from `product_variants` by SKU.
    pub option1: Option<String>,
    pub option2: Option<String>,
    pub option3: Option<String>,
    /// Compare-at price — enriched from `product_variants` by SKU (in cents).
    #[serde(with = "option_money_wire")]
    pub compare_at_price: Option<Money>,
}

/// Page locale. The KR store is Korean-only; (de)serialized as the locale code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MinisForumKrLocale {
    #[serde(rename = "ko")]
    Ko,
}

impl MinisForumKrLocale {
    /// Builds the variant from the destructured locale code. Errors on any code
    /// other than "ko".
    pub fn from_string(locale: &str) -> Result<Self, String> {
        match locale {
            "ko" => Ok(MinisForumKrLocale::Ko),
            other => Err(format!("unexpected locale: {other:?}")),
        }
    }
}

/// The mapped product core.
///
/// Assembled from several JSON-only blocks since KR has no `const product`:
/// - `id` / `handle` / `vendor` / `type` from `meta`.
/// - `sku` / `brand` from the JSON-LD `schemas`.
/// - `price` / `availability` from the single JSON-LD offer **when present**
///   (most KR pages carry no offer, so both are `Option`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumKrProcessedProductInfo {
    pub id: MinisForumKrProductId,
    pub handle: String,
    pub vendor: String,
    /// Product title, from `dawn_main.title` when available.
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub product_type: Option<MinisForumKrProductType>,
    /// Catalogue SKU, from the JSON-LD schema (absent on most pages).
    pub sku: Option<String>,
    /// Brand, from the JSON-LD schema.
    pub brand: Option<String>,
    /// Price, from the single JSON-LD offer (absent when the page has no offer).
    #[serde(with = "option_money_wire")]
    pub price: Option<Money>,
    /// Availability, from the single JSON-LD offer (absent when no offer).
    pub availability: Option<MinisForumKrProcessedAvailability>,
}

/// Parses a destructured minor-unit (won-cents) string (e.g. "7580000") into a
/// minor-unit count. Errors if the string is not a valid integer.
fn parse_cents(price: &str) -> Result<i64, String> {
    price
        .parse()
        .map_err(|error| format!("invalid price {price:?}: {error}"))
}

/// Converts a KR offer price string into a minor-unit count.
///
/// KRW is presented in **major units** (won) in the offer block, sometimes with a
/// `.` as a thousands separator (`"786.900"` → 786900 won) and sometimes plain
/// (`"68000"` → 68000 won). KRW has no sub-unit, so the offer string never has a
/// real fraction; we strip any `.` separators, parse the integer won amount, and
/// scale to the minor units the `meta` block uses (× 100) so the two sources are
/// directly comparable.
fn dollars_to_cents(price: &str) -> Result<i64, String> {
    let won_digits: String = price.chars().filter(|c| *c != '.').collect();
    let won: i64 = won_digits
        .parse()
        .map_err(|error| format!("invalid price {price:?}: {error}"))?;
    Ok(won * 100)
}

/// The canonical [`Money`] wire shape: `{ "amount_minor": "7580000", "currency":
/// "KRW" }` (amount as a base-10 string so it round-trips exactly).
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
        "KRW" => Ok(Currency::KRW),
        other => Err(format!("unknown currency: {other:?}")),
    }
}

/// serde glue for a [`Money`] field, (de)serialized as the canonical wire object.
mod money_wire {
    use super::{Money, MoneyWire};

    pub fn serialize<S: serde::Serializer>(money: &Money, serializer: S) -> Result<S::Ok, S::Error> {
        serde::Serialize::serialize(&MoneyWire::from_money(money), serializer)
    }

    pub fn deserialize<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Money, D::Error> {
        let wire: MoneyWire = serde::Deserialize::deserialize(deserializer)?;
        wire.into_money()
    }
}

/// serde glue for an optional [`Money`] field.
mod option_money_wire {
    use super::{Money, MoneyWire};

    pub fn serialize<S: serde::Serializer>(money: &Option<Money>, serializer: S) -> Result<S::Ok, S::Error> {
        match money {
            Some(money) => serializer.serialize_some(&MoneyWire::from_money(money)),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Option<Money>, D::Error> {
        let wire: Option<MoneyWire> = serde::Deserialize::deserialize(deserializer)?;
        wire.map(MoneyWire::into_money).transpose()
    }
}

/// Product category, (de)serialized as the original `meta.type` string. The KR
/// catalogue's categories differ from AU's, so this enum is store-specific.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MinisForumKrProductType {
    #[serde(rename = "Mini PC")]
    MiniPc,
    #[serde(rename = "Motherboard")]
    Motherboard,
    #[serde(rename = "Docking Station")]
    DockingStation,
    #[serde(rename = "laptop bag")]
    LaptopBag,
    #[serde(rename = "mousepad")]
    Mousepad,
    #[serde(rename = "gift")]
    Gift,
}

impl MinisForumKrProductType {
    /// Builds the variant from the destructured `type` string. Errors on any
    /// value not seen in the data.
    pub fn from_string(product_type: &str) -> Result<Self, String> {
        match product_type {
            "Mini PC" => Ok(MinisForumKrProductType::MiniPc),
            "Motherboard" => Ok(MinisForumKrProductType::Motherboard),
            "Docking Station" => Ok(MinisForumKrProductType::DockingStation),
            "laptop bag" => Ok(MinisForumKrProductType::LaptopBag),
            "mousepad" => Ok(MinisForumKrProductType::Mousepad),
            "gift" => Ok(MinisForumKrProductType::Gift),
            other => Err(format!("unexpected product type: {other:?}")),
        }
    }
}

/// A product id. Built from the destructured `meta.id` string, (de)serialized as
/// a number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MinisForumKrProductId(pub u64);

impl MinisForumKrProductId {
    /// Builds the id by parsing the destructured id string (e.g. "7598010597422").
    /// Errors if the string is not a valid number.
    pub fn from_string(id: &str) -> Result<Self, String> {
        id.parse()
            .map(Self)
            .map_err(|error| format!("invalid product id {id:?}: {error}"))
    }
}

/// Product stock status. (De)serialized as a JSON boolean (`true` = available).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(into = "bool", from = "bool")]
pub enum MinisForumKrProcessedAvailability {
    Available,
    Unavailable,
}

impl From<bool> for MinisForumKrProcessedAvailability {
    fn from(available: bool) -> Self {
        if available {
            MinisForumKrProcessedAvailability::Available
        } else {
            MinisForumKrProcessedAvailability::Unavailable
        }
    }
}

impl From<MinisForumKrProcessedAvailability> for bool {
    fn from(availability: MinisForumKrProcessedAvailability) -> Self {
        availability == MinisForumKrProcessedAvailability::Available
    }
}

impl From<MinisForumKrAvailability> for MinisForumKrProcessedAvailability {
    fn from(availability: MinisForumKrAvailability) -> Self {
        match availability {
            MinisForumKrAvailability::InStock => MinisForumKrProcessedAvailability::Available,
            MinisForumKrAvailability::OutOfStock => MinisForumKrProcessedAvailability::Unavailable,
        }
    }
}

impl TryFrom<MinisForumKrDestructuredProduct> for MinisForumKrProcessedProduct {
    type Error = String;

    fn try_from(destructured: MinisForumKrDestructuredProduct) -> Result<Self, Self::Error> {
        // Collect every JSON-LD offer keyed by its (schema-level) SKU. KR offers
        // are rare — at most one per page — but keying them lets us enrich the
        // matching `meta` variant generically.
        let offers_by_sku = collect_offers_by_sku(&destructured.schemas);

        // The single offer (if any) also feeds the product core's price/
        // availability. There is at most one across the whole page.
        let core_offer = destructured
            .schemas
            .iter()
            .flat_map(|schema| schema.offers.iter())
            .next()
            .cloned();

        // Brand/sku for the core come from the first schema carrying them.
        let core_sku = destructured.schemas.iter().find_map(|schema| schema.sku.clone());
        let core_brand = destructured.schemas.iter().find_map(|schema| schema.brand.clone());

        // Derive product-level price from the first `product_variants` entry
        // when no schema offer is present (most KR pages lack offers).
        let core_price = core_offer
            .as_ref()
            .map(|offer| -> Result<Money, String> {
                Ok(Money::new(
                    dollars_to_cents(&offer.price)?,
                    currency_from_code(&offer.currency)?,
                ))
            })
            .transpose()?
            .or_else(|| {
                destructured
                    .product_variants
                    .first()
                    .and_then(|pv| pv.price.as_deref())
                    .map(parse_cents)
                    .transpose()
                    .ok()
                    .flatten()
                    .map(|cents| Money::new(cents, Currency::KRW))
            });

        let product = MinisForumKrProcessedProductInfo {
            id: MinisForumKrProductId::from_string(&destructured.meta.id)?,
            handle: destructured.meta.handle,
            vendor: destructured.meta.vendor,
            title: destructured.dawn_main.as_ref().map(|d| d.title.clone()),
            product_type: destructured
                .meta
                .product_type
                .as_deref()
                .map(MinisForumKrProductType::from_string)
                .transpose()?,
            sku: core_sku,
            brand: core_brand,
            price: core_price,
            availability: core_offer.as_ref().map(|offer| offer.availability.into()),
        };

        let variants = destructured
            .meta
            .variants
            .into_iter()
            .map(|meta| {
                let pv = meta.sku.as_deref().and_then(|sku| {
                    destructured
                        .product_variants
                        .iter()
                        .find(|pv| pv.sku.as_deref() == Some(sku))
                });
                make_variant(meta, &offers_by_sku, pv)
            })
            .collect::<Result<_, _>>()?;

        let images = destructured
            .dawn_main
            .map(|d| d.media.into_iter().filter_map(|m| m.src).collect())
            .unwrap_or_default();

        Ok(Self {
            locale: MinisForumKrLocale::from_string(&destructured.locale)?,
            product,
            images,
            variants,
        })
    }
}

/// Indexes every offer in the page by its parent schema's SKU. An offer with no
/// SKU on its schema cannot be matched to a variant, so it is skipped here.
fn collect_offers_by_sku(schemas: &[MinisForumKrSchema]) -> Vec<(String, MinisForumKrOffer)> {
    let mut by_sku = Vec::new();
    for schema in schemas {
        if let Some(sku) = &schema.sku {
            for offer in &schema.offers {
                by_sku.push((sku.clone(), offer.clone()));
            }
        }
    }
    by_sku
}

/// Builds one [`MinisForumKrVariant`] from a `meta` analytics variant, enriched by
/// the JSON-LD offer that shares its SKU (when one exists) and by the matching
/// `product_variants` entry for option/compare-at data.
///
/// `meta` is the only always-present source: it provides the SKU, the minor-unit
/// price, the analytics variant id and the title. The offer — present on a
/// minority of pages — adds availability and `price_valid_until`, and its price
/// (won, major units) is guarded to agree with the `meta` minor-unit price.
fn make_variant(
    meta: MinisForumKrMetaVariant,
    offers_by_sku: &[(String, MinisForumKrOffer)],
    pv: Option<&MinisForumKrProductVariant>,
) -> Result<MinisForumKrVariant, String> {
    let meta_price = Money::new(parse_cents(&meta.price)?, Currency::KRW);

    // Find a matching offer by SKU (only when the meta variant has a SKU).
    let matched = meta.sku.as_ref().and_then(|sku| {
        offers_by_sku
            .iter()
            .find(|(offer_sku, _)| offer_sku == sku)
            .map(|(_, offer)| offer)
    });

    let mut availability = None;
    let mut price_valid_until = None;

    if let Some(offer) = matched {
        // The offer price (won → minor units) must agree with the meta price.
        let offer_currency = currency_from_code(&offer.currency)?;
        let offer_price = Money::new(dollars_to_cents(&offer.price)?, offer_currency);
        if offer_price != meta_price {
            return Err(format!(
                "variant price mismatch for sku {:?}: meta={meta_price:?}, offer={offer_price:?}",
                meta.sku
            ));
        }

        availability = Some(offer.availability.into());

        price_valid_until = offer
            .price_valid_until
            .as_deref()
            .map(|raw| {
                raw.parse::<NaiveDate>()
                    .map_err(|error| format!("invalid price_valid_until {raw:?}: {error}"))
            })
            .transpose()?;
    }

    // Title comes from the meta variant; drop empty strings to `None`.
    let title = meta.title.filter(|title| !title.is_empty());

    let (option1, option2, option3, compare_at_price) = match pv {
        Some(pv) => {
            let cat = pv
                .compare_at_price
                .as_deref()
                .map(parse_cents)
                .transpose()?
                .map(|cents| Money::new(cents, Currency::KRW));
            (pv.option1.clone(), pv.option2.clone(), pv.option3.clone(), cat)
        }
        None => (None, None, None, None),
    };

    Ok(MinisForumKrVariant {
        sku: meta.sku,
        price: meta_price,
        availability,
        title,
        price_valid_until,
        meta_variant_id: meta.variant_id,
        option1,
        option2,
        option3,
        compare_at_price,
    })
}
