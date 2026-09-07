//! Page architecture for UGREEN EU (`eu.ugreen.com`).
//!
//! Shopify store with a custom Liquid theme (neither Mathema nor Next.js).
//! Product data comes from `var meta` (Shopify analytics), the JSON-LD schemas,
//! the `web-pixels-manager` initData block, and the `Viewed Product` tracking
//! event.  The DOM uses custom web components (`<product-price>`,
//! `<rte-formatter>`) and standard Shopify `<variant-picker>`.

use crate::parsing::structure::RetailerArchitecture;
use crate::parsing::structure::{collection, json, json_after, particle, segment};

pub fn offer_detail_architecture_v1() -> RetailerArchitecture {
    RetailerArchitecture::new(vec![
        particle("html", "locale", vec![("lang", "value")]),
        // JSON-LD: Organization + Product schemas.
        collection(
            r#"script[type="application/ld+json"]"#,
            "schemas",
            vec![json(
                "script",
                "",
                vec![
                    ("@type", "type"),
                    ("name", "name"),
                    ("description", "description"),
                    ("sku", "sku"),
                    ("gtin", "gtin"),
                    ("image", "images"),
                    ("brand.name", "brand"),
                    ("category", "category"),
                    ("offers.price", "price"),
                    ("offers.priceCurrency", "currency"),
                    ("offers.availability", "availability"),
                    ("offers.url", "offer_url"),
                    ("url", "url"),
                    ("logo", "logo"),
                    ("offers.hasMerchantReturnPolicy.applicableCountry", "return_country"),
                    ("offers.shippingDetails.shippingDestination.addressCountry", "ships_to"),
                ],
            )],
        ),
        // Shopify analytics: product + variants (prices in cents).
        json_after(
            "script",
            "var meta =",
            "meta",
            vec![
                ("product.id", "id"),
                ("product.gid", "gid"),
                ("product.vendor", "vendor"),
                ("product.type", "type"),
                ("product.handle", "handle"),
                ("product.variants[].id", "variant_id"),
                ("product.variants[].price", "price"),
                ("product.variants[].sku", "sku"),
                ("product.variants[].name", "name"),
                ("product.variants[].public_title", "public_title"),
            ],
        ),
        // web-pixels-manager initData: shop info, page type, related products.
        json_after(
            "script",
            "initData:",
            "pixels",
            vec![
                ("shop.name", "shop_name"),
                ("shop.paymentSettings.currencyCode", "currency"),
                ("page.pageType", "page_type"),
                ("page.resourceId", "product_id"),
            ],
        ),
        // Shopify "Viewed Product" analytics event: current product.
        json_after(
            "script",
            r#""Viewed Product","#,
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
        ),
        // Chrome: header, navigation, footer.
        segment("header", "header", vec![]),
        segment("nav", "nav", vec![]),
        segment("footer", "footer", vec![]),
        // judge.me reviews badge data.
        particle(
            ".jdgm-prev-badge",
            "reviews",
            vec![
                ("data-average-rating", "average_rating"),
                ("data-number-of-reviews", "review_count"),
                ("data-number-of-questions", "question_count"),
            ],
        ),
        particle(".jdgm-prev-badge__text", "reviews_text", vec![("", "value")]),
        // The product block: title, price, gallery, description, variant picker.
        segment(
            "main",
            "product",
            vec![
                // Breadcrumbs.
                collection(
                    "nav[aria-label='breadcrumbs'] a",
                    "breadcrumbs",
                    vec![
                        particle("", "name", vec![("", "value")]),
                        particle("", "url", vec![("href", "value")]),
                    ],
                ),
                // Product title (inside rte-formatter).
                particle("rte-formatter h1", "title", vec![("", "value")]),
                // Price block: sale price and compare-at (regular) price.
                particle(".price", "price", vec![("", "value")]),
                particle(".compare-at-price", "compare_at_price", vec![("", "value")]),
                // Discount badge.
                particle(".ug-price-discount-tag", "discount", vec![("", "value")]),
                // Variant picker.
                collection(
                    "variant-picker fieldset",
                    "options",
                    vec![
                        particle("legend", "label", vec![("", "value")]),
                        collection("input", "values", vec![particle("", "", vec![("value", "value")])]),
                    ],
                ),
                // All images.
                collection(
                    "img",
                    "images",
                    vec![particle(
                        "",
                        "",
                        vec![("src", "src"), ("alt", "alt"), ("loading", "loading")],
                    )],
                ),
                // Remaining product links.
                collection(
                    "a[href]",
                    "links",
                    vec![
                        particle("", "label", vec![("", "value")]),
                        particle("", "url", vec![("href", "value")]),
                    ],
                ),
                // All headings.
                collection("h2", "h2_headings", vec![particle("", "text", vec![("", "value")])]),
                collection("h3", "h3_headings", vec![particle("", "text", vec![("", "value")])]),
                // Catch-all product text.
                particle("", "text", vec![("", "value")]),
            ],
        ),
    ])
}
