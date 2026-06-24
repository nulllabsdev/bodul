//! Page architecture for MinisForum HK (`hk.minisforum.com`).
//!
//! A MinisForum Shopify store (currency `HKD`, locale `zh-TW`). It carries the
//! `tt_product` and `bm_product_variants` scripts. The main-product DOM section
//! uses the Express theme (different selectors from the Impact theme used by
//! AU/JP). See [`super::minisforum`] for the shared structure.

use super::minisforum::{Config, Xcotton, architecture as build};
use super::structure::RetailerArchitecture;
use crate::html_parser::{collection, json, particle, segment, trash};

/// The MinisForum HK page architecture.
pub fn architecture() -> RetailerArchitecture {
    let mut architecture = build(Config {
        tt_product: true,
        xcotton: Xcotton::None,
        const_product: false,
        const_product_variants: false,
    });
    // HK-only: `<script id="bm_product_variants" type="application/json">` is a
    // JSON array of variant objects. Extract each variant's fields.
    architecture.structures.push(json(
        r#"script#bm_product_variants"#,
        "bm_product_variants",
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
            ("public_title", "public_title"),
        ],
    ));
    // Express theme DOM: the main product section uses a `div` (not `section`)
    // with different internal selectors than Impact theme's `main_product()`.
    architecture.structures.push(segment(
        "div.shopify-section--main-product",
        "express_main",
        vec![
            trash("noscript"),
            particle("h1.product-meta__title", "title", vec![("", "value")]),
            segment(
                "product-meta",
                "price",
                vec![particle(
                    ".price.price--large",
                    "sale_price",
                    vec![("", "value")],
                )],
            ),
            segment(
                "product-media",
                "gallery",
                vec![collection(
                    ".product__media-item",
                    "media",
                    vec![particle("img", "", vec![("src", "src"), ("alt", "alt")])],
                )],
            ),
            collection(
                ".product-form__option-selector",
                "options",
                vec![
                    particle(".product-form__option-name", "label", vec![("", "value")]),
                    particle(
                        ".product-form__option-value",
                        "selected",
                        vec![("", "value")],
                    ),
                    collection(
                        "input",
                        "values",
                        vec![particle("", "", vec![("value", "value")])],
                    ),
                ],
            ),
        ],
    ));
    architecture
}
