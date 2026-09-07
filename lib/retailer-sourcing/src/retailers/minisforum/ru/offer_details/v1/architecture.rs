//! Page architecture for MinisForum RU (`ru.minisforum.com`).
//!
//! A MinisForum Shopify store (currency `USD`, locale `en`). It carries none of
//! the optional product scripts, but does have a main-product DOM section that
//! uses a custom theme (not Impact/Express/Dawn). See [`crate::retailers::minisforum::architecture_v1`] for
//! the shared structure.

use crate::parsing::structure::RetailerArchitecture;
use crate::parsing::structure::{collection, json, particle, segment, trash};
use crate::retailers::minisforum::architecture_v1::{Config, Xcotton, offer_detail_architecture_v1 as build};

/// The MinisForum RU page architecture.
pub fn offer_detail_architecture_v1() -> RetailerArchitecture {
    let mut architecture = build(Config {
        tt_product: false,
        xcotton: Xcotton::None,
        const_product: false,
        const_product_variants: false,
    });
    // Custom "Motion" / "Streamline" theme DOM: the main product section uses a
    // `div.product-section` with unique class names not shared by other themes.
    architecture.structures.push(segment(
        "div.product-section",
        "motion_main",
        vec![
            trash("noscript"),
            particle("h1.product-single__title", "title", vec![("", "value")]),
            segment(
                "div.product-block--price",
                "price",
                vec![
                    trash(".visually-hidden"),
                    particle("span.product__price", "sale_price", vec![("", "value")]),
                ],
            ),
            collection(
                ".product-main-slide",
                "media",
                vec![particle(
                    "img.photoswipe__image",
                    "",
                    vec![("src", "src"), ("alt", "alt")],
                )],
            ),
            collection(
                ".variant-wrapper",
                "options",
                vec![
                    particle("label.variant__label", "label", vec![("", "value")]),
                    collection(
                        "input[data-variant-input]",
                        "values",
                        vec![particle("", "", vec![("value", "value")])],
                    ),
                ],
            ),
        ],
    ));
    // Hidden textarea with variant JSON: option1/2/3, price, compare_at_price,
    // available, sku — all in minor units. Equivalent to `const productVariants`
    // that JP/KR have in inline JS.
    architecture.structures.push(json(
        r#"textarea[data-variant-json]"#,
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
        ],
    ));
    architecture
}
