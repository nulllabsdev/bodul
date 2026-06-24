//! Processed MinisForum EU product model.
//!
//! The output of `process_products`: built up field by field by mapping from the
//! [`MinisForumEuDestructuredProduct`] of the previous step into our own shape.
//!
//! Shape differences from the AU model:
//! - The product core and the product variants come from the
//!   `xcotton_pp_variants` block (there is no `const product` / `tt_product`).
//! - The store is multilingual: [`MinisForumEuLocale`] has `De` and `En`.
//! - The currency is EUR.
//! - JSON-LD offers carry no `sku` and no `name` and are **not** one-per-variant
//!   (a page has 0, 1 or 2 offers regardless of how many variants it has). They
//!   therefore cannot be joined to a variant by SKU the way AU does. The combined
//!   [`ZzzVariant`] is built from the two **per-variant** sources that *are*
//!   present and SKU-aligned — `xcotton_pp_variants.variants` and
//!   `meta.variants` — joined by SKU, with `sku`/`price` guarded across both and
//!   `title` taken from the `meta` variant.

use money::{Currency, Money};

use super::destructured::{
    MinisForumEuDestructuredProduct, MinisForumEuFeature, MinisForumEuMetaVariant,
    MinisForumEuProductVariant, MinisForumEuXcottonPpVariants,
};

/// A processed MinisForum EU product.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumEuProcessedProduct {
    /// Page locale (`de` or `en`).
    pub locale: MinisForumEuLocale,
    /// The core product fields.
    pub product: MinisForumEuProcessedProductInfo,
    /// Product image URLs, from the `xxxx` gallery. The rest of `xxxx` (title,
    /// price, variants) is intentionally skipped — it duplicates data mapped
    /// elsewhere.
    pub images: Vec<String>,
    /// Specification rows, from `feature_chart` (the column-major chart is
    /// flattened). Empty when the page has no feature chart. The chart's `h1`/`h2`
    /// headings are not mapped.
    pub features: Vec<MinisForumEuProcessedFeature>,

    // new: combining all variants into a vector of Variant
    pub variants: Vec<ZzzVariant>,
}

/// One product specification: a `label` and its `value` lines (the raw cell text
/// split on `\n`, with blank/whitespace-only lines dropped).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumEuProcessedFeature {
    pub label: String,
    pub value: Vec<String>,
}

impl From<MinisForumEuFeature> for MinisForumEuProcessedFeature {
    fn from(feature: MinisForumEuFeature) -> Self {
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

/// One variant combining the `xcotton_pp_variants` variant and the `meta` variant
/// that share a SKU. Each field is de-prefixed where the value is lifted across
/// both sources; the analytics id stays prefixed (`meta_`) to keep its provenance
/// explicit.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ZzzVariant {
    /// SKU — guaranteed identical across both sources.
    pub sku: Option<String>,
    /// Price — guaranteed identical across both sources.
    #[serde(with = "money_wire")]
    pub price: Money,
    /// Availability — from the product variant's `available` flag.
    pub availability: MinisForumEuProcessedAvailability,
    /// Variant title — the `meta` variant's title (offers carry no name in EU).
    pub title: Option<String>,
    // From the product object's variant.
    pub option1: String,
    pub option2: Option<String>,
    pub option3: Option<String>,
    #[serde(with = "option_money_wire")]
    pub compare_at_price: Option<Money>,
    // From the `meta` analytics variant.
    pub meta_variant_id: String,
}

/// Page locale. The EU store is multilingual; (de)serialized as the locale code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MinisForumEuLocale {
    #[serde(rename = "de")]
    De,
    #[serde(rename = "en")]
    En,
}

impl MinisForumEuLocale {
    /// Builds the variant from the destructured locale code. Errors on any code
    /// other than "de" or "en".
    pub fn from_string(locale: &str) -> Result<Self, String> {
        match locale {
            "de" => Ok(MinisForumEuLocale::De),
            "en" => Ok(MinisForumEuLocale::En),
            other => Err(format!("unexpected locale: {other:?}")),
        }
    }
}

/// The mapped subset of the full destructured product object.
///
/// Intentionally not mapped:
/// - `price_min` / `price_max` — we don't track the variant price range.
/// - `media` — product images come from the gallery instead (see the `xxxx`
///   gallery section), not from `xcotton_pp_variants.media`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumEuProcessedProductInfo {
    pub id: MinisForumEuProductId,
    pub handle: String,
    pub title: String,
    pub vendor: String,
    #[serde(rename = "type")]
    pub product_type: Option<MinisForumEuProductType>,
    #[serde(with = "money_wire")]
    pub price: Money,
    #[serde(with = "option_money_wire")]
    pub compare_at_price: Option<Money>,
    pub availability: MinisForumEuProcessedAvailability,
}

/// A mapped product variant.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinisForumEuProcessedVariant {
    pub sku: Option<String>,
    pub option1: String,
    pub option2: Option<String>,
    pub option3: Option<String>,
    #[serde(with = "money_wire")]
    pub price: Money,
    #[serde(with = "option_money_wire")]
    pub compare_at_price: Option<Money>,
    pub availability: MinisForumEuProcessedAvailability,
}

/// Parses a destructured cents string (e.g. "2590") into a minor-unit count.
/// Errors if the string is not a valid integer.
fn parse_cents(price: &str) -> Result<i64, String> {
    price
        .parse()
        .map_err(|error| format!("invalid price {price:?}: {error}"))
}

/// Converts a major-unit euro string (e.g. "959.0", "25.90") into a minor-unit
/// (cents) count. Errors on a non-numeric value or more than two fractional
/// digits (EUR has a minor-unit exponent of 2; we do not round).
#[allow(dead_code)]
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
/// "EUR" }` (amount as a base-10 string so it round-trips exactly).
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
#[allow(dead_code)]
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

/// Product category, (de)serialized as the original `type` string. EU is
/// multilingual, so the same category appears under its German and English label
/// (e.g. `Mini-PC` / `Mini PC`, `Renoviert` / `Refurbished`); note the trailing
/// space in `"Arbeitsplatz "` and the source typo `"Accesorries"`, both kept
/// verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MinisForumEuProductType {
    // German labels.
    #[serde(rename = "Mini-PC")]
    MiniPcDe,
    #[serde(rename = "Renoviert")]
    RenoviertDe,
    #[serde(rename = "Zubehör")]
    ZubehoerDe,
    #[serde(rename = "Andere")]
    AndereDe,
    #[serde(rename = "Tower-Gehäuse")]
    TowerGehaeuseDe,
    #[serde(rename = "Hauptplatinen")]
    HauptplatinenDe,
    #[serde(rename = "Dockingstation")]
    DockingstationDe,
    #[serde(rename = "Geschenkkarte")]
    GeschenkkarteDe,
    // Note: the source data has a trailing space.
    #[serde(rename = "Arbeitsplatz ")]
    ArbeitsplatzDeTrailingSpace,
    #[serde(rename = "Arbeitsplatz")]
    ArbeitsplatzDe,
    #[serde(rename = "Tablet-PC")]
    TabletPcDe,
    // English labels.
    #[serde(rename = "Mini PC")]
    MiniPc,
    #[serde(rename = "Refurbished")]
    Refurbished,
    // Note: the source data misspells "Accessories".
    #[serde(rename = "Accesorries")]
    Accessories,
    #[serde(rename = "Other")]
    Other,
    #[serde(rename = "Motherboards")]
    Motherboards,
    #[serde(rename = "Docking Station")]
    DockingStation,
    #[serde(rename = "Gift Card")]
    GiftCard,
    #[serde(rename = "Work Station")]
    WorkStation,
    // Locale-neutral.
    #[serde(rename = "NAS")]
    Nas,
}

impl MinisForumEuProductType {
    /// Builds the variant from the destructured `type` string. Errors on any
    /// value not seen in the data.
    pub fn from_string(product_type: &str) -> Result<Self, String> {
        match product_type {
            "Mini-PC" => Ok(MinisForumEuProductType::MiniPcDe),
            "Renoviert" => Ok(MinisForumEuProductType::RenoviertDe),
            "Zubehör" => Ok(MinisForumEuProductType::ZubehoerDe),
            "Andere" => Ok(MinisForumEuProductType::AndereDe),
            "Tower-Gehäuse" => Ok(MinisForumEuProductType::TowerGehaeuseDe),
            "Hauptplatinen" => Ok(MinisForumEuProductType::HauptplatinenDe),
            "Dockingstation" => Ok(MinisForumEuProductType::DockingstationDe),
            "Geschenkkarte" => Ok(MinisForumEuProductType::GeschenkkarteDe),
            "Arbeitsplatz " => Ok(MinisForumEuProductType::ArbeitsplatzDeTrailingSpace),
            "Arbeitsplatz" => Ok(MinisForumEuProductType::ArbeitsplatzDe),
            "Tablet-PC" => Ok(MinisForumEuProductType::TabletPcDe),
            "Mini PC" => Ok(MinisForumEuProductType::MiniPc),
            "Refurbished" => Ok(MinisForumEuProductType::Refurbished),
            "Accesorries" => Ok(MinisForumEuProductType::Accessories),
            "Other" => Ok(MinisForumEuProductType::Other),
            "Motherboards" => Ok(MinisForumEuProductType::Motherboards),
            "Docking Station" => Ok(MinisForumEuProductType::DockingStation),
            "Gift Card" => Ok(MinisForumEuProductType::GiftCard),
            "Work Station" => Ok(MinisForumEuProductType::WorkStation),
            "NAS" => Ok(MinisForumEuProductType::Nas),
            other => Err(format!("unexpected product type: {other:?}")),
        }
    }
}

/// A product id. Built from the destructured id string, (de)serialized as a
/// number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MinisForumEuProductId(pub u64);

impl MinisForumEuProductId {
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
pub enum MinisForumEuProcessedAvailability {
    Available,
    Unavailable,
}

impl MinisForumEuProcessedAvailability {
    /// Builds the variant from the destructured `available` flag, matching the
    /// string directly. Errors on any value other than "true"/"false".
    pub fn from_string(available: &str) -> Result<Self, String> {
        match available {
            "true" => Ok(MinisForumEuProcessedAvailability::Available),
            "false" => Ok(MinisForumEuProcessedAvailability::Unavailable),
            other => Err(format!("unexpected availability flag: {other:?}")),
        }
    }
}

impl From<bool> for MinisForumEuProcessedAvailability {
    fn from(available: bool) -> Self {
        if available {
            MinisForumEuProcessedAvailability::Available
        } else {
            MinisForumEuProcessedAvailability::Unavailable
        }
    }
}

impl From<MinisForumEuProcessedAvailability> for bool {
    fn from(availability: MinisForumEuProcessedAvailability) -> Self {
        availability == MinisForumEuProcessedAvailability::Available
    }
}

impl From<super::destructured::MinisForumEuAvailability> for MinisForumEuProcessedAvailability {
    fn from(availability: super::destructured::MinisForumEuAvailability) -> Self {
        match availability {
            super::destructured::MinisForumEuAvailability::InStock => {
                MinisForumEuProcessedAvailability::Available
            }
            super::destructured::MinisForumEuAvailability::OutOfStock => {
                MinisForumEuProcessedAvailability::Unavailable
            }
        }
    }
}

impl TryFrom<MinisForumEuDestructuredProduct> for MinisForumEuProcessedProduct {
    type Error = String;

    fn try_from(destructured: MinisForumEuDestructuredProduct) -> Result<Self, Self::Error> {
        let variants = make_variants(
            destructured.xcotton_pp_variants.variants.clone(),
            destructured.meta.variants.clone(),
        )?;

        Ok(Self {
            locale: MinisForumEuLocale::from_string(&destructured.locale)?,
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

/// Combines the per-variant sources into [`ZzzVariant`]s, **matching by SKU**.
///
/// Unlike AU, the JSON-LD offers are not a per-variant source in EU (they carry
/// no SKU or name and are not one-per-variant), so the join is over the two
/// sources that *are* per-variant and SKU-aligned: the `xcotton_pp_variants`
/// variant and the `meta` variant.
fn make_variants(
    product_variants: Vec<MinisForumEuProductVariant>,
    meta_variants: Vec<MinisForumEuMetaVariant>,
) -> Result<Vec<ZzzVariant>, String> {
    // guard they have the same length
    if product_variants.len() != meta_variants.len() {
        return Err(format!(
            "variant source length mismatch: product={}, meta={}",
            product_variants.len(),
            meta_variants.len(),
        ));
    }

    // For each product variant, find the `meta` variant with the same SKU, and
    // combine the two into one `ZzzVariant`.
    let mut combined = Vec::with_capacity(product_variants.len());
    for product in product_variants {
        let meta = meta_variants
            .iter()
            .find(|meta| meta.sku == product.sku)
            .cloned()
            .ok_or_else(|| format!("no meta variant for sku {:?}", product.sku))?;
        combined.push(make_variant(product, meta)?);
    }

    Ok(combined)
}

/// Combines one matched (product variant, `meta` variant) pair into a
/// [`ZzzVariant`], applying the same per-field conversions as elsewhere.
fn make_variant(
    product: MinisForumEuProductVariant,
    meta: MinisForumEuMetaVariant,
) -> Result<ZzzVariant, String> {
    // Both sources must agree on the SKU; lift it to a single field.
    if product.sku != meta.sku {
        return Err(format!(
            "variant sku mismatch: product={:?}, meta={:?}",
            product.sku, meta.sku
        ));
    }

    // Both sources must agree on the price; lift it to a single field.
    let product_price = Money::new(parse_cents(&product.price)?, Currency::EUR);
    let meta_price = Money::new(parse_cents(&meta.price)?, Currency::EUR);
    if product_price != meta_price {
        return Err(format!(
            "variant price mismatch: product={product_price:?}, meta={meta_price:?}"
        ));
    }

    let availability = MinisForumEuProcessedAvailability::from_string(&product.available)?;

    Ok(ZzzVariant {
        sku: product.sku,
        price: product_price,
        availability,
        // Title comes from the `meta` variant; offers carry no name in EU.
        title: meta.title,
        option1: product.option1,
        option2: product.option2,
        option3: product.option3,
        compare_at_price: product
            .compare_at_price
            .as_deref()
            .map(parse_cents)
            .transpose()?
            .map(|cents| Money::new(cents, Currency::EUR)),
        meta_variant_id: meta.variant_id,
    })
}

impl TryFrom<MinisForumEuXcottonPpVariants> for MinisForumEuProcessedProductInfo {
    type Error = String;

    fn try_from(product: MinisForumEuXcottonPpVariants) -> Result<Self, Self::Error> {
        Ok(Self {
            id: MinisForumEuProductId::from_string(&product.id)?,
            handle: product.handle,
            title: product.title,
            vendor: product.vendor,
            product_type: product
                .product_type
                .as_deref()
                .map(MinisForumEuProductType::from_string)
                .transpose()?,
            price: Money::new(parse_cents(&product.price)?, Currency::EUR),
            compare_at_price: product
                .compare_at_price
                .as_deref()
                .map(parse_cents)
                .transpose()?
                .map(|cents| Money::new(cents, Currency::EUR)),
            availability: MinisForumEuProcessedAvailability::from_string(&product.available)?,
        })
    }
}

impl TryFrom<MinisForumEuProductVariant> for MinisForumEuProcessedVariant {
    type Error = String;

    fn try_from(variant: MinisForumEuProductVariant) -> Result<Self, Self::Error> {
        Ok(Self {
            sku: variant.sku,
            option1: variant.option1,
            option2: variant.option2,
            option3: variant.option3,
            price: Money::new(parse_cents(&variant.price)?, Currency::EUR),
            compare_at_price: variant
                .compare_at_price
                .as_deref()
                .map(parse_cents)
                .transpose()?
                .map(|cents| Money::new(cents, Currency::EUR)),
            availability: MinisForumEuProcessedAvailability::from_string(&variant.available)?,
        })
    }
}
