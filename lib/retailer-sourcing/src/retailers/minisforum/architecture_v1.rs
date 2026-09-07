//! Shared page architecture for the MinisForum Shopify stores.
//!
//! Every regional store (AU/CA/EU/US/UK/FR/KR/JP/RU/HK) runs the same Shopify
//! theme, so they share the bulk of their architecture: the same JSON-LD, the
//! same `web-pixels-manager`/`var meta`/`Viewed Product` analytics blocks, the
//! same feature-chart and main-product DOM. They differ only in *which* embedded
//! product-data scripts the page carries — captured by [`Config`].
//!
//! [`offer_detail_architecture_v1`] assembles the common structures, gated by the
//! config. Blocks that may be absent on a given store (e.g. the `feature_chart` /
//! `main_product` segments) are always included: a segment whose selector matches
//! nothing simply yields no output, so it is harmless.

use crate::parsing::structure::{RetailerArchitecture, Structure, collection};
use crate::parsing::structure::{json, json_after, particle, scrub, segment, trash};

/// Where a store keeps its full Shopify product object (the "xcotton" data).
pub enum Xcotton {
    /// Not present.
    None,
    /// As a `<script id="xcotton_pp_variants">` JSON block (CA/EU/US).
    Script,
    /// As a `var __xcotton_pp_variants__ = {...}` JS assignment (UK/FR).
    JsVar,
}

/// Which optional product-data scripts a given store embeds. The rest of the
/// architecture is shared across every store.
pub struct Config {
    /// `<script id="tt_product">` JSON (AU/CA/UK/HK).
    pub tt_product: bool,
    /// How the full product object is embedded, if at all.
    pub xcotton: Xcotton,
    /// `const product = {...}` product JSON embedded in JS (AU/CA/US).
    pub const_product: bool,
    /// `const productVariants = [...]` variant array embedded in JS (JP/KR).
    pub const_product_variants: bool,
}

/// Assembles the MinisForum architecture for a store described by `config`.
pub fn offer_detail_architecture_v1(config: Config) -> RetailerArchitecture {
    let mut structures = vec![locale(), schemas()];
    if config.tt_product {
        structures.push(tt_product());
    }
    match config.xcotton {
        Xcotton::None => {}
        Xcotton::Script => structures.push(xcotton_script()),
        Xcotton::JsVar => structures.push(xcotton_jsvar()),
    }
    if config.const_product {
        structures.push(const_product());
    }
    if config.const_product_variants {
        structures.push(product_variants());
    }
    structures.push(pixels());
    structures.push(meta());
    structures.push(viewed_product());
    structures.push(feature_chart());
    // newsletter signup forms (boilerplate)
    structures.push(trash("form#NewsletterForm"));
    structures.push(trash("div#notify-button"));
    structures.push(main_product());
    structures.push(describe_box());
    RetailerArchitecture::new(structures)
}

/// Page locale, e.g. "en"/"de"/"ja". Read from the `<html lang>` attribute —
/// cleaner than the `Shopify.locale = "..."` JS assignment, and on every page.
fn locale() -> Structure {
    particle("html", "locale", vec![("lang", "value")])
}

/// `<script type="application/ld+json">` — the Product and BreadcrumbList schemas.
fn schemas() -> Structure {
    collection(
        r#"script[type="application/ld+json"]"#,
        "schemas",
        vec![json(
            r#"script"#,
            "",
            vec![
                // Product schema.
                ("sku", "sku"),
                ("productID", "product_id"),
                ("brand.name", "brand"),
                ("offers[].name", "name"),
                ("offers[].price", "price"),
                ("offers[].priceCurrency", "currency"),
                ("offers[].availability", "availability"),
                ("offers[].priceValidUntil", "price_valid_until"),
                ("offers[].sku", "sku"),
                // BreadcrumbList schema (the second ld+json block).
                ("itemListElement[].name", "name"),
                ("itemListElement[].item", "url"),
            ],
        )],
    )
}

/// `<script id="tt_product">` — a small product summary.
fn tt_product() -> Structure {
    json(
        r#"script#tt_product"#,
        "tt_product",
        vec![("id", "id"), ("title", "title"), ("image_url", "image_url")],
    )
}

/// `<script id="xcotton_pp_variants">` — the full Shopify product object.
fn xcotton_script() -> Structure {
    json(r#"script#xcotton_pp_variants"#, "xcotton_pp_variants", product_paths())
}

/// The same full product object as a `var __xcotton_pp_variants__ = {...}` JS
/// assignment (UK/FR stores).
fn xcotton_jsvar() -> Structure {
    json_after(
        "script",
        "__xcotton_pp_variants__ =",
        "xcotton_pp_variants",
        product_paths(),
    )
}

/// The same full product object, embedded in JS as `const product = {...}`.
fn const_product() -> Structure {
    json_after("script", "const product =", "product", product_paths())
}

/// `const productVariants = [...]` — variant array embedded in JS (JP/KR).
/// Each element has `price`/`compare_at_price` in cents, `option1/2/3`, `sku`,
/// `available`, and optionally `featured_image.src`.
fn product_variants() -> Structure {
    json_after(
        "script",
        "const productVariants =",
        "product_variants",
        vec![
            ("id", "id"),
            ("sku", "sku"),
            ("title", "title"),
            ("option1", "option1"),
            ("option2", "option2"),
            ("option3", "option3"),
            ("available", "available"),
            ("price", "price"),
            ("compare_at_price", "compare_at_price"),
            ("name", "name"),
            ("featured_image.src", "image_src"),
        ],
    )
}

/// Dotted paths for the full Shopify product object (shared by `xcotton` and
/// `const_product`).
fn product_paths() -> Vec<(&'static str, &'static str)> {
    vec![
        ("id", "id"),
        ("title", "title"),
        ("handle", "handle"),
        ("vendor", "vendor"),
        ("type", "type"),
        ("available", "available"),
        ("price", "price"),
        ("price_min", "price_min"),
        ("price_max", "price_max"),
        ("compare_at_price", "compare_at_price"),
        ("variants[].sku", "sku"),
        ("variants[].available", "available"),
        ("variants[].price", "price"),
        ("variants[].compare_at_price", "compare_at_price"),
        ("variants[].option1", "option1"),
        ("variants[].option2", "option2"),
        ("variants[].option3", "option3"),
        ("media[].src", "src"),
        ("media[].width", "width"),
        ("media[].height", "height"),
        ("media[].media_type", "type"),
    ]
}

/// Shopify web-pixels-manager `initData: {...}`: shop info, current page, and the
/// related-products list.
fn pixels() -> Structure {
    json_after(
        "script",
        "initData:",
        "pixels",
        vec![
            ("shop.name", "shop_name"),
            ("shop.paymentSettings.currencyCode", "currency"),
            ("shop.countryCode", "country"),
            ("page.pageType", "page_type"),
            ("page.resourceId", "product_id"),
            ("products[].id", "id"),
            ("products[].handle", "handle"),
            ("products[].title", "title"),
            ("products[].vendor", "vendor"),
            ("products[].url", "url"),
        ],
    )
}

/// Shopify analytics `var meta = {...}`: the product + its variants.
fn meta() -> Structure {
    json_after(
        "script",
        "var meta =",
        "meta",
        vec![
            ("product.id", "id"),
            ("product.gid", "gid"),
            ("product.vendor", "vendor"),
            ("product.handle", "handle"),
            ("product.type", "type"),
            ("product.variants[].id", "variant_id"),
            ("product.variants[].price", "price"),
            ("product.variants[].sku", "sku"),
            ("product.variants[].public_title", "title"),
            ("page.pageType", "page_type"),
            ("page.resourceId", "resource_id"),
        ],
    )
}

/// Shopify analytics `track("Viewed Product", {...})`: the currently viewed
/// product/variant. Note: price here is in dollars (e.g. "975.00"), unlike the
/// cents in `meta`/`xcotton_pp_variants`.
fn viewed_product() -> Structure {
    json_after(
        "script",
        r#"Viewed Product","#,
        "viewed_product",
        vec![
            ("currency", "currency"),
            ("variantId", "variant_id"),
            ("productId", "product_id"),
            ("productGid", "gid"),
            ("name", "name"),
            ("price", "price"),
            ("sku", "sku"),
            ("brand", "brand"),
            ("variant", "variant"),
            ("category", "category"),
        ],
    )
}

/// Product specifications. One row per heading, each holding a `label` and a
/// `values` vector (multi-column charts have several values).
/// `feature_chart::transpose` turns these rows into column-major `features`.
fn feature_chart() -> Structure {
    segment(
        "section.shopify-section--feature-chart",
        "feature_chart",
        vec![
            // The section heading(s), e.g. "Product Specification".
            particle("h1", "h1", vec![("", "value")]),
            particle("h2", "h2", vec![("", "value")]),
            collection(
                ".feature-chart__table-row",
                "rows",
                vec![
                    particle(".feature-chart__heading", "label", vec![("", "value")]),
                    collection(
                        ".feature-chart__value",
                        "values",
                        vec![particle("", "", vec![("", "value")])],
                    ),
                ],
            ),
        ],
    )
}

/// The main product section: title/badge, gallery, variant picker and price.
fn main_product() -> Structure {
    segment(
        "section.shopify-section--main-product div.section-full",
        "xxxx",
        vec![
            // Strip the template-id `form` attribute from every element in the
            // section (in place — runs before anything is componentized).
            scrub("[form]", "id"),
            scrub("[form]", "form"),
            // Trash UI noise (financing widgets, contact form, gallery chrome).
            trash("div.UserForm_box"),
            trash("div.as-paypal-installment"),
            trash("div[paypal-installment]"),
            trash("payment-terms"),
            trash("custom-cursor"),
            trash("scroll-shadow"),
            trash("unit-price"),
            // Title + promo badge.
            particle("h1.product-info__title", "title", vec![("", "value")]),
            particle("p.product_id_b", "badge", vec![("", "value")]),
            segment(
                "product-gallery",
                "gallery",
                vec![
                    particle("product-gallery", "", vec![("form", "")]),
                    particle("media-carousel", "", vec![("id", "")]),
                    // buttons + duplicate of images
                    trash("page-dots"),
                    collection(
                        "div.product-gallery__media",
                        "media",
                        vec![
                            particle("div.product-gallery__media", "", vec![("data-media-id", "")]),
                            particle(
                                "img",
                                "",
                                vec![
                                    ("src", "src"),
                                    ("alt", "alt"),
                                    ("srcset", ""),
                                    ("width", ""),
                                    ("height", ""),
                                    ("sizes", ""),
                                    ("fetchpriority", ""),
                                    ("loading", ""),
                                ],
                            ),
                        ],
                    ),
                ],
            ),
            segment(
                "variant-picker",
                "variants",
                vec![collection(
                    "fieldset",
                    "options",
                    vec![
                        // The option's label, e.g. "CPU:".
                        particle("legend", "label", vec![("", "value")]),
                        // The currently selected value.
                        particle("variant-option-value", "selected", vec![("", "value")]),
                        // Every selectable choice (the radio inputs carry the value).
                        collection("input", "values", vec![particle("", "", vec![("value", "value")])]),
                    ],
                )],
            ),
            trash("noscript"),
            trash("div.product-info__quantity-selector"),
            // Price block. Trash the sr-only "Sale price"/"Regular price" labels
            // so the price particles read just the amount.
            segment(
                "div.product-info__price",
                "price",
                vec![
                    trash("sale-price .sr-only"),
                    trash("compare-at-price .sr-only"),
                    particle("sale-price", "sale_price", vec![("", "value")]),
                    particle("compare-at-price", "compare_at_price", vec![("", "value")]),
                    particle("on-sale-badge", "savings", vec![("", "value")]),
                ],
            ),
            particle("payment-terms", "form_payment_terms", vec![("form", "")]),
            collection(
                "form.shopify-product-form",
                "add_to_cart_boxes",
                vec![trash("form.shopify-product-form")],
            ),
            particle("dd", "x", vec![("", "text")]),
        ],
    )
}

/// Product highlights / contact note box: its text plus any links.
fn describe_box() -> Structure {
    segment(
        "div.Describe_box",
        "describe_box",
        vec![
            particle("", "", vec![("", "text")]),
            collection("a", "links", vec![particle("", "", vec![("href", "href")])]),
        ],
    )
}
