//! Processed MinisForum UK product model.
//!
//! The output of `process_products`: built up field by field by mapping from the
//! [`MinisForumUkDestructuredProduct`] of the previous step into our own shape.
//!
//! Unlike the AU store, the UK product core and its variants are sourced from the
//! `xcotton_pp_variants` block (there is no `const product` object). The combined
//! variants therefore match `xcotton_pp_variants.variants`, `meta.variants` and the
//! first schema's `offers` by SKU.

use chrono::NaiveDate;
use money::{Currency, Money};

use super::destructured::{
    MinisForumUkAvailability, MinisForumUkDestructuredProduct, MinisForumUkFeature,
    MinisForumUkMetaVariant, MinisForumUkOffer, MinisForumUkXcottonProduct,
    MinisForumUkXcottonVariant,
};

/// A processed MinisForum UK product.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumUkProcessedProduct {
    /// Page locale.
    pub locale: MinisForumUkLocale,
    /// The core product fields.
    pub product: MinisForumUkProcessedProductInfo,
    /// Product image URLs, from the `xxxx` gallery. The rest of `xxxx` (title,
    /// price, variants) is intentionally skipped — it duplicates data mapped
    /// elsewhere.
    pub images: Vec<String>,
    /// Specification rows, from `feature_chart` (the column-major chart is
    /// flattened). Empty when the page has no feature chart. The chart's `h2`
    /// heading is not mapped.
    pub features: Vec<MinisForumUkProcessedFeature>,

    // new: combining all variants and offers into vector of Variant
    pub variants: Vec<ZzzVariant>,
}

/// One product specification: a `label` and its `value` lines (the raw cell text
/// split on `\n`, with blank/whitespace-only lines dropped).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumUkProcessedFeature {
    pub label: String,
    pub value: Vec<String>,
}

impl From<MinisForumUkFeature> for MinisForumUkProcessedFeature {
    fn from(feature: MinisForumUkFeature) -> Self {
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

/// One variant combining the xcotton product-variant, `meta`-variant and JSON-LD
/// offer rows that share a SKU. Each lifted field is guarded; single-source fields
/// are de-prefixed except `meta_variant_id`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ZzzVariant {
    /// SKU — guaranteed identical across all three sources.
    pub sku: Option<String>,
    /// Price — guaranteed identical across all three sources.
    #[serde(with = "money_wire")]
    pub price: Money,
    /// Availability — guaranteed identical across the xcotton and offer sources.
    pub availability: MinisForumUkProcessedAvailability,
    /// Variant title — the offer name; the `meta` title must match it when present.
    pub title: String,
    pub price_valid_until: NaiveDate,
    // From the xcotton product object's variant.
    pub option1: String,
    pub option2: Option<String>,
    pub option3: Option<String>,
    #[serde(with = "option_money_wire")]
    pub compare_at_price: Option<Money>,
    // From the `meta` analytics variant.
    pub meta_variant_id: String,
}

/// Page locale. The UK store is English-only; (de)serialized as the locale code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MinisForumUkLocale {
    #[serde(rename = "en")]
    En,
}

impl MinisForumUkLocale {
    /// Builds the variant from the destructured locale code. Errors on any code
    /// other than "en".
    pub fn from_string(locale: &str) -> Result<Self, String> {
        match locale {
            "en" => Ok(MinisForumUkLocale::En),
            other => Err(format!("unexpected locale: {other:?}")),
        }
    }
}

/// The mapped subset of the xcotton product object.
///
/// Intentionally not mapped:
/// - `price_min` / `price_max` — we don't track the variant price range.
/// - `media` — product images come from the gallery instead (see the `xxxx`
///   gallery section), not from `xcotton_pp_variants.media`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumUkProcessedProductInfo {
    pub id: MinisForumUkProductId,
    pub handle: String,
    pub title: String,
    pub vendor: String,
    #[serde(rename = "type")]
    pub product_type: MinisForumUkProductType,
    #[serde(with = "money_wire")]
    pub price: Money,
    #[serde(with = "option_money_wire")]
    pub compare_at_price: Option<Money>,
    pub availability: MinisForumUkProcessedAvailability,
    //pub variants: Vec<MinisForumUkProcessedVariant>,
}

/// A mapped product variant.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumUkProcessedVariant {
    pub sku: Option<String>,
    pub option1: String,
    pub option2: Option<String>,
    pub option3: Option<String>,
    #[serde(with = "money_wire")]
    pub price: Money,
    #[serde(with = "option_money_wire")]
    pub compare_at_price: Option<Money>,
    pub availability: MinisForumUkProcessedAvailability,
}

/// Parses a destructured cents string (e.g. "2590") into a minor-unit count.
/// Errors if the string is not a valid integer.
fn parse_cents(price: &str) -> Result<i64, String> {
    price
        .parse()
        .map_err(|error| format!("invalid price {price:?}: {error}"))
}

/// Converts a major-unit pound string (e.g. "959.0", "25.90") into a minor-unit
/// (pence) count. Errors on a non-numeric value or more than two fractional
/// digits (GBP has a minor-unit exponent of 2; we do not round).
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

/// The canonical [`Money`] wire shape: `{ "amount_minor": "63900", "currency":
/// "GBP" }` (amount as a base-10 string so it round-trips exactly).
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
        "GBP" => Ok(Currency::GBP),
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

/// Product category, (de)serialized as the original `type` string. Surveyed from
/// the UK data — distinct from AU's set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MinisForumUkProductType {
    #[serde(rename = "Mini PC")]
    MiniPc,
    #[serde(rename = "Mini Workstation")]
    MiniWorkstation,
    #[serde(rename = "Peripheral accessories")]
    PeripheralAccessories,
    #[serde(rename = "Motherboard")]
    Motherboard,
    #[serde(rename = "eGPU Dock Station")]
    EgpuDockStation,
    #[serde(rename = "Gift Card")]
    GiftCard,
    #[serde(rename = "Portable monitor")]
    PortableMonitor,
    #[serde(rename = "Keyboard")]
    Keyboard,
    #[serde(rename = "NAS")]
    Nas,
    #[serde(rename = "shipping-protection")]
    ShippingProtection,
}

impl MinisForumUkProductType {
    /// Builds the variant from the destructured `type` string. Errors on any
    /// value not seen in the data.
    pub fn from_string(product_type: &str) -> Result<Self, String> {
        match product_type {
            "Mini PC" => Ok(MinisForumUkProductType::MiniPc),
            "Mini Workstation" => Ok(MinisForumUkProductType::MiniWorkstation),
            "Peripheral accessories" => Ok(MinisForumUkProductType::PeripheralAccessories),
            "Motherboard" => Ok(MinisForumUkProductType::Motherboard),
            "eGPU Dock Station" => Ok(MinisForumUkProductType::EgpuDockStation),
            "Gift Card" => Ok(MinisForumUkProductType::GiftCard),
            "Portable monitor" => Ok(MinisForumUkProductType::PortableMonitor),
            "Keyboard" => Ok(MinisForumUkProductType::Keyboard),
            "NAS" => Ok(MinisForumUkProductType::Nas),
            "shipping-protection" => Ok(MinisForumUkProductType::ShippingProtection),
            other => Err(format!("unexpected product type: {other:?}")),
        }
    }
}

/// A product id. Built from the destructured id string, (de)serialized as a
/// number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MinisForumUkProductId(pub u64);

impl MinisForumUkProductId {
    /// Builds the id by parsing the destructured id string (e.g. "8090954236019").
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
pub enum MinisForumUkProcessedAvailability {
    Available,
    Unavailable,
}

impl MinisForumUkProcessedAvailability {
    /// Builds the variant from the destructured `available` flag, matching the
    /// string directly. Errors on any value other than "true"/"false".
    pub fn from_string(available: &str) -> Result<Self, String> {
        match available {
            "true" => Ok(MinisForumUkProcessedAvailability::Available),
            "false" => Ok(MinisForumUkProcessedAvailability::Unavailable),
            other => Err(format!("unexpected availability flag: {other:?}")),
        }
    }
}

impl From<bool> for MinisForumUkProcessedAvailability {
    fn from(available: bool) -> Self {
        if available {
            MinisForumUkProcessedAvailability::Available
        } else {
            MinisForumUkProcessedAvailability::Unavailable
        }
    }
}

impl From<MinisForumUkProcessedAvailability> for bool {
    fn from(availability: MinisForumUkProcessedAvailability) -> Self {
        availability == MinisForumUkProcessedAvailability::Available
    }
}

impl From<MinisForumUkAvailability> for MinisForumUkProcessedAvailability {
    fn from(availability: MinisForumUkAvailability) -> Self {
        match availability {
            MinisForumUkAvailability::InStock => MinisForumUkProcessedAvailability::Available,
            MinisForumUkAvailability::OutOfStock => MinisForumUkProcessedAvailability::Unavailable,
        }
    }
}

impl TryFrom<MinisForumUkDestructuredProduct> for MinisForumUkProcessedProduct {
    type Error = String;

    fn try_from(destructured: MinisForumUkDestructuredProduct) -> Result<Self, Self::Error> {
        // The first schema (Product) is the source of the offers used to build
        // the combined variants.
        let product_schema = destructured
            .schemas
            .into_iter()
            .next()
            .ok_or_else(|| "missing product schema".to_string())?;

        let variants = make_variants(
            destructured.xcotton_pp_variants.variants.clone(),
            destructured.meta.variants.clone(),
            product_schema.offers,
        )?;

        Ok(Self {
            locale: MinisForumUkLocale::from_string(&destructured.locale)?,
            product: destructured.xcotton_pp_variants.try_into()?,
            images: destructured
                .main_product
                .gallery
                .media
                .into_iter()
                .map(|media| media.src)
                .collect(),
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

fn make_variants(
    product_variants: Vec<MinisForumUkXcottonVariant>,
    meta_variants: Vec<MinisForumUkMetaVariant>,
    offers: Vec<MinisForumUkOffer>,
) -> Result<Vec<ZzzVariant>, String> {
    //guard they all have same lengths
    if product_variants.len() != meta_variants.len() || meta_variants.len() != offers.len() {
        return Err(format!(
            "variant source length mismatch: product={}, meta={}, offers={}",
            product_variants.len(),
            meta_variants.len(),
            offers.len(),
        ));
    }

    // For each xcotton variant, find the `meta` variant and the offer with the
    // same SKU, and combine the three into one `ZzzVariant`.
    let mut combined = Vec::with_capacity(product_variants.len());
    for product in product_variants {
        let meta = meta_variants
            .iter()
            .find(|meta| meta.sku == product.sku)
            .cloned()
            .ok_or_else(|| format!("no meta variant for sku {:?}", product.sku))?;
        let offer = offers
            .iter()
            .find(|offer| offer.sku == product.sku)
            .cloned()
            .ok_or_else(|| format!("no offer for sku {:?}", product.sku))?;
        combined.push(make_variant(product, meta, offer)?);
    }

    Ok(combined)
}

/// Combines one matched (xcotton variant, `meta` variant, offer) triple into a
/// [`ZzzVariant`], applying the same per-field conversions as elsewhere.
fn make_variant(
    product: MinisForumUkXcottonVariant,
    meta: MinisForumUkMetaVariant,
    offer: MinisForumUkOffer,
) -> Result<ZzzVariant, String> {
    // All three sources must agree on the SKU; lift it to a single field.
    if product.sku != meta.sku || meta.sku != offer.sku {
        return Err(format!(
            "variant sku mismatch: product={:?}, meta={:?}, offer={:?}",
            product.sku, meta.sku, offer.sku
        ));
    }

    let offer_currency = currency_from_code(&offer.currency)?;

    // All three sources must agree on the price; lift it to a single field.
    let product_price = Money::new(parse_cents(&product.price)?, Currency::GBP);
    let meta_price = Money::new(parse_cents(&meta.price)?, Currency::GBP);
    let offer_price = Money::new(dollars_to_cents(&offer.price)?, offer_currency);
    if product_price != meta_price || meta_price != offer_price {
        return Err(format!(
            "variant price mismatch: product={product_price:?}, meta={meta_price:?}, offer={offer_price:?}"
        ));
    }

    // The xcotton and offer sources must agree on availability; lift it.
    let product_availability = MinisForumUkProcessedAvailability::from_string(&product.available)?;
    let offer_availability: MinisForumUkProcessedAvailability = offer.availability.into();
    if product_availability != offer_availability {
        return Err(format!(
            "variant availability mismatch: product={product_availability:?}, offer={offer_availability:?}"
        ));
    }

    // Title comes from the offer name (always present); the `meta` title, when
    // present and non-empty, must match it.
    let title = offer.name;
    if let Some(meta_title) = &meta.title
        && !meta_title.is_empty()
        && meta_title != &title
    {
        return Err(format!(
            "variant title mismatch: meta={meta_title:?}, offer={title:?}"
        ));
    }

    Ok(ZzzVariant {
        sku: product.sku,
        price: product_price,
        availability: product_availability,
        title,
        price_valid_until: offer.price_valid_until.parse().map_err(|error| {
            format!(
                "invalid price_valid_until {:?}: {error}",
                offer.price_valid_until
            )
        })?,
        option1: product.option1,
        option2: product.option2,
        option3: product.option3,
        compare_at_price: product
            .compare_at_price
            .as_deref()
            .map(parse_cents)
            .transpose()?
            .map(|cents| Money::new(cents, Currency::GBP)),
        meta_variant_id: meta.variant_id,
    })
}

impl TryFrom<MinisForumUkXcottonProduct> for MinisForumUkProcessedProductInfo {
    type Error = String;

    fn try_from(product: MinisForumUkXcottonProduct) -> Result<Self, Self::Error> {
        Ok(Self {
            id: MinisForumUkProductId::from_string(&product.id)?,
            handle: product.handle,
            title: product.title,
            vendor: product.vendor,
            product_type: MinisForumUkProductType::from_string(&product.product_type)?,
            price: Money::new(parse_cents(&product.price)?, Currency::GBP),
            compare_at_price: product
                .compare_at_price
                .as_deref()
                .map(parse_cents)
                .transpose()?
                .map(|cents| Money::new(cents, Currency::GBP)),
            availability: MinisForumUkProcessedAvailability::from_string(&product.available)?,
            // variants: product
            //     .variants
            //     .into_iter()
            //     .map(MinisForumUkProcessedVariant::try_from)
            //     .collect::<Result<_, _>>()?,
        })
    }
}

impl TryFrom<MinisForumUkXcottonVariant> for MinisForumUkProcessedVariant {
    type Error = String;

    fn try_from(variant: MinisForumUkXcottonVariant) -> Result<Self, Self::Error> {
        Ok(Self {
            sku: variant.sku,
            option1: variant.option1,
            option2: variant.option2,
            option3: variant.option3,
            price: Money::new(parse_cents(&variant.price)?, Currency::GBP),
            compare_at_price: variant
                .compare_at_price
                .as_deref()
                .map(parse_cents)
                .transpose()?
                .map(|cents| Money::new(cents, Currency::GBP)),
            availability: MinisForumUkProcessedAvailability::from_string(&variant.available)?,
        })
    }
}
