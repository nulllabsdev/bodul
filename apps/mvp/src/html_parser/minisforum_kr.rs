//! Page architecture for MinisForum KR (`kr.minisforum.com`).
//!
//! A MinisForum Shopify store (currency `KRW`, locale `ko`). It carries
//! `const productVariants` in JS but has no main-product DOM section matching
//! the Impact theme selectors — the KR store uses the Dawn theme with different
//! class names. See [`super::minisforum`] for the shared structure.

use super::minisforum::{Config, Xcotton, architecture as build};
use super::structure::RetailerArchitecture;
use crate::html_parser::{collection, particle, segment, trash};

/// The MinisForum KR page architecture.
pub fn architecture() -> RetailerArchitecture {
    let mut architecture = build(Config {
        tt_product: false,
        xcotton: Xcotton::None,
        const_product: false,
        const_product_variants: true,
    });
    // Dawn theme DOM: the main product section uses `section.product-section`
    // with different internal selectors than Impact theme's `main_product()`.
    architecture.structures.push(segment(
        "section.product-section",
        "dawn_main",
        vec![
            trash("noscript"),
            trash("h1.product__title .visually-hidden"),
            particle("h1.product__title", "title", vec![("", "value")]),
            segment(
                ".price.price--product",
                "price",
                vec![
                    trash(".visually-hidden"),
                    particle(".price-item--sale", "sale_price", vec![("", "value")]),
                    particle(
                        "dd.price__compare s.price-item--regular",
                        "compare_at_price",
                        vec![("", "value")],
                    ),
                ],
            ),
            trash("div.product__media-grid-noscript"),
            collection(
                ".product__media-item",
                "media",
                vec![particle("img", "", vec![("src", "src"), ("alt", "alt")])],
            ),
            collection(
                "variant-radios fieldset.product-form__controls",
                "options",
                vec![
                    particle("legend.product-form__group-name", "label", vec![("", "value")]),
                    collection("input", "values", vec![particle("", "", vec![("value", "value")])]),
                ],
            ),
        ],
    ));
    architecture
}
