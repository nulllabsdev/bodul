//! Page architecture for Anker France (`www.anker.com/fr`).
//!
//! Headless Next.js storefront backed by Shopify — same family as `crate::retailers::anker::eu`.
//! Product data is carried by `script#__NEXT_DATA__` (`props.pageProps.product`),
//! the JSON-LD blocks and the OpenGraph head meta.  Differs from the EU layout:
//! uses `<div id="header">` (not `<header>`) and section-based footer; lacks
//! `#reviews`, `#brandInfo`, `#brandRecommends`, and `#productRecommends`
//! sections on sampled products.

use crate::parsing::structure::RetailerArchitecture;
use crate::parsing::structure::{collection, comments, json, particle, scrub, segment, trash};

pub fn offer_detail_architecture_v1() -> RetailerArchitecture {
    RetailerArchitecture::new(vec![
        comments(),
        scrub(r#"meta[name="next-head-count"]"#, "content"),
        trash(r#"script[src^="/_next/"]"#),
        trash(r#"link[href^="/_next/"]"#),
        trash("style"),
        json(
            "script#__NEXT_DATA__",
            "next_data",
            vec![
                ("locale", "locale"),
                ("buildId", "build_id"),
                ("props.pageProps.slug", "slug"),
                ("props.pageProps.shop.name", "shop_name"),
                ("props.pageProps.shop.paymentSettings.currencyCode", "shop_currency"),
                ("props.pageProps.shop.primaryDomain.host", "shop_host"),
                ("props.pageProps.seo.title", "seo_title"),
                ("props.pageProps.seo.description", "seo_description"),
                ("props.pageProps.seo.canonical", "canonical"),
                ("props.pageProps.product.id", "product_id"),
                ("props.pageProps.product.handle", "handle"),
                ("props.pageProps.product.title", "title"),
                ("props.pageProps.product.name", "name"),
                ("props.pageProps.product.vendor", "vendor"),
                ("props.pageProps.product.productType", "product_type"),
                ("props.pageProps.product.description", "description"),
                ("props.pageProps.product.availableForSale", "available_for_sale"),
                ("props.pageProps.product.totalInventory", "total_inventory"),
                ("props.pageProps.product.publishedAt", "published_at"),
                ("props.pageProps.product.price.value", "price"),
                ("props.pageProps.product.price.currencyCode", "currency"),
                ("props.pageProps.product.listPrice", "list_price"),
                ("props.pageProps.product.images[].url", "image_url"),
                ("props.pageProps.product.variants[].id", "variant_id"),
                ("props.pageProps.product.variants[].sku", "sku"),
                ("props.pageProps.product.variants[].barcode", "barcode"),
                ("props.pageProps.product.variants[].name", "variant_name"),
                ("props.pageProps.product.variants[].price", "variant_price"),
                ("props.pageProps.product.variants[].listPrice", "variant_list_price"),
                (
                    "props.pageProps.product.variants[].availableForSale",
                    "variant_available",
                ),
                (
                    "props.pageProps.product.variants[].quantityAvailable",
                    "quantity_available",
                ),
                ("props.pageProps.product.collections.nodes[].title", "collection_title"),
                (
                    "props.pageProps.product.collections.nodes[].handle",
                    "collection_handle",
                ),
            ],
        ),
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
                    ("offers.url", "offer_url"),
                    ("offers.itemCondition", "condition"),
                    ("offers.availability", "availability"),
                    ("offers.price", "price"),
                    ("offers.priceCurrency", "currency"),
                    ("legalName", "legal_name"),
                    ("url", "url"),
                    ("email", "email"),
                    ("sameAs", "same_as"),
                    ("logo", "logo"),
                    ("itemListElement[].position", "position"),
                    ("itemListElement[].name", "item_name"),
                    ("itemListElement[].item", "item_url"),
                    ("mainEntity[].name", "question"),
                    ("mainEntity[].acceptedAnswer.text", "answer"),
                ],
            )],
        ),
        segment(
            "head",
            "head_meta",
            vec![
                particle("title", "title", vec![("", "value")]),
                particle(r#"meta[name="description"]"#, "description", vec![("content", "value")]),
                particle(r#"meta[name="robots"]"#, "robots", vec![("content", "value")]),
                particle(r#"meta[property="og:title"]"#, "og_title", vec![("content", "value")]),
                particle(
                    r#"meta[property="og:description"]"#,
                    "og_description",
                    vec![("content", "value")],
                ),
                particle(r#"meta[property="og:url"]"#, "og_url", vec![("content", "value")]),
                particle(r#"meta[property="og:type"]"#, "og_type", vec![("content", "value")]),
                particle(r#"meta[property="og:image"]"#, "og_image", vec![("content", "value")]),
                particle(
                    r#"meta[property="og:image:alt"]"#,
                    "og_image_alt",
                    vec![("content", "value")],
                ),
                particle(
                    r#"meta[property="og:image:width"]"#,
                    "og_image_width",
                    vec![("content", "value")],
                ),
                particle(
                    r#"meta[property="og:image:height"]"#,
                    "og_image_height",
                    vec![("content", "value")],
                ),
                particle(
                    r#"meta[property="og:site_name"]"#,
                    "og_site_name",
                    vec![("content", "value")],
                ),
                particle(r#"link[rel="canonical"]"#, "canonical", vec![("href", "value")]),
                collection(
                    r#"link[rel="alternate"]"#,
                    "alternates",
                    vec![particle("", "", vec![("hreflang", "hreflang"), ("href", "url")])],
                ),
                collection(
                    r#"link[as="image"]"#,
                    "preload_images",
                    vec![particle("", "url", vec![("href", "value")])],
                ),
            ],
        ),
        segment(
            "div#boardLogo",
            "board_logo",
            vec![collection(
                "a",
                "links",
                vec![particle("", "url", vec![("href", "value")])],
            )],
        ),
        segment("div#TopBanner", "top_banner", vec![]),
        segment("div.Sidebar_root__85r2g", "sidebar", vec![]),
        segment(
            "#header",
            "navigation",
            vec![collection(
                "a",
                "links",
                vec![particle("", "url", vec![("href", "value")])],
            )],
        ),
        segment(
            "div.footerContent",
            "footer",
            vec![collection(
                "a",
                "links",
                vec![
                    particle("", "name", vec![("", "value")]),
                    particle("", "url", vec![("href", "value")]),
                ],
            )],
        ),
        segment("div#cookie-consent", "cookie_consent", vec![]),
        collection(
            "section#productSpecs",
            "specs",
            vec![
                particle("h2", "heading", vec![("", "value")]),
                collection(
                    "div.flex-wrap > div",
                    "rows",
                    vec![
                        particle("p.Manuals_textSmall__Gu4Xc", "label", vec![("", "value")]),
                        particle("p.Manuals_subTextSmall__KBOX4", "value", vec![("", "value")]),
                    ],
                ),
            ],
        ),
        collection(
            "section#faq",
            "faq",
            vec![
                collection(
                    "h3.Manuals_question__5qUm_",
                    "questions",
                    vec![particle("", "text", vec![("", "value")])],
                ),
                collection(
                    r#"div[id^="faq-answer"]"#,
                    "answers",
                    vec![particle("", "text", vec![("", "value")])],
                ),
            ],
        ),
        collection(
            "div.Crumbs_root__C3P0U",
            "breadcrumbs",
            vec![
                collection(
                    "div.Crumbs_block__CjXCL a",
                    "crumbs",
                    vec![
                        particle("", "name", vec![("", "value")]),
                        particle("", "url", vec![("href", "value")]),
                    ],
                ),
                particle("div.Crumbs_last__pVHPS", "current", vec![("", "value")]),
            ],
        ),
        segment(
            "main",
            "product",
            vec![
                collection(
                    ".salePrice",
                    "sale_prices",
                    vec![particle("", "text", vec![("", "value")])],
                ),
                collection(
                    ".savePrice",
                    "save_badges",
                    vec![particle("", "text", vec![("", "value")])],
                ),
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
                scrub(".jdgm-prev-badge__stars", "aria-label"),
                scrub(".jdgm-prev-badge__stars", "data-score"),
                collection("h1", "headings", vec![particle("", "text", vec![("", "value")])]),
                particle(
                    "div#amzn-buy-now",
                    "amazon_widget",
                    vec![
                        ("data-sku", "sku"),
                        ("data-site-id", "site_id"),
                        ("data-widget-id", "widget_id"),
                    ],
                ),
                collection(
                    "div.Text_body__snVk8",
                    "labels",
                    vec![particle("", "text", vec![("", "value")])],
                ),
                collection("h2", "h2_headings", vec![particle("", "text", vec![("", "value")])]),
                collection("h3", "h3_headings", vec![particle("", "text", vec![("", "value")])]),
                collection(
                    "img",
                    "images",
                    vec![particle(
                        "",
                        "",
                        vec![
                            ("src", "src"),
                            ("srcset", "srcset"),
                            ("data-src", "data_src"),
                            ("alt", "alt"),
                        ],
                    )],
                ),
                collection(
                    r#"a[href^="https://cdn.shopify.com"]"#,
                    "media_links",
                    vec![particle("", "url", vec![("href", "value")])],
                ),
            ],
        ),
        collection(
            r#"section:not(#productSpecs):not(#faq)"#,
            "marketing",
            vec![
                particle("", "section_id", vec![("id", "value")]),
                particle("", "text", vec![("", "value")]),
            ],
        ),
    ])
}
