//! Page architecture for MinisForum AU (`au.minisforum.com`).
//!
//! These are Shopify product pages: the stable, machine-readable fields live in
//! the Open Graph / `product:` meta tags in `<head>`, whose value sits in the
//! `content` attribute. The architecture targets those rather than the visible
//! DOM, keying each tag by its `property`.

use super::structure::{RetailerArchitecture, collection};
use crate::html_parser::{json, json_after, particle, scrub, segment, trash};



/// The MinisForum AU page architecture.
pub fn architecture() -> RetailerArchitecture {
    RetailerArchitecture::new(vec![
        // extracts data from <script type="application/ld+json">
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
        ),

        //temporarily remove all scripts
        collection("script", "scripts", vec![trash("script")]),
        //temporarily remove all styles
        collection("style", "styles", vec![trash("style")]),
        // remove all SVGs (minisforum has a lot of them, mostly icons and payment logos)
        trash("svg"),
        // newsletter signup forms (boilerplate)
                trash("form#NewsletterForm"),
        trash("div#notify-button"),

        segment("section.shopify-section--main-product div.section-full","xxxx",vec![
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
                    particle("product-gallery", "", vec![("form", "form")]),
                    particle("media-carousel", "", vec![("id", "id")]),
                    // buttons + duplicate of images
                    trash("page-dots"),

                    collection(
                        "div.product-gallery__media",
                        "media",
                        vec![
                            particle(
                                "div.product-gallery__media",
                                "",
                                vec![("data-media-id", "data-media-id")],
                            ),
                            particle(
                                "img",
                                "",
                                vec![
                                    ("src", "src"),
                                    ("alt", "alt"),
                                    ("srcset", "srcset"),
                                    ("width", "width"),
                                    ("height", "height"),
                                    ("sizes", "sizes"),
                                    ("fetchpriority", "fetchpriority"),
                                    ("loading", "loading"),
                                ],
                            ),
                        ],
                    ),
                ]
            ),


            segment(
                "variant-picker",
                "variants",
                vec![
                    collection(
                    "fieldset",
                    "options",
                    vec![
                        // The option's label, e.g. "CPU:".
                        particle("legend", "label", vec![("", "value")]),
                        // The currently selected value.
                        particle("variant-option-value", "selected", vec![("", "value")]),
                        // Every selectable choice (the radio inputs carry the value).
                        collection(
                            "input",
                            "values",
                            vec![particle("", "", vec![("value", "value")])],
                        ),

                    ],
                ),

                ],
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


            collection("form.shopify-product-form", "add_to_cart_boxes", vec![
                trash("form.shopify-product-form")
                //particle("form.shopify-product-form", "", vec![("", ""),("id", "")]),
            ]),






            particle("dd","x",vec![("","text")]),
        ]),

        // Product highlights / contact note box: its text plus any links.
        segment(
            "div.Describe_box",
            "describe_box",
            vec![
                particle("", "", vec![("", "text")]),
                collection("a", "links", vec![particle("", "", vec![("href", "href")])]),
            ],
        ),

    ])
}


/// The MinisForum AU page architecture.
pub fn old2_architecture() -> RetailerArchitecture {
    RetailerArchitecture::new(vec![

        collection(
            r#"script[type="application/ld+json"]"#,
            "schemas",
            vec![json(
                r#"script"#,
                "schema",
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
        ),

        //temporarily remove all scripts
        collection("script", "scripts", vec![trash("script")]),
        //temporarily remove all styles
        collection("style", "styles", vec![trash("style")]),

        // not interseted in annoucement bar, a marquee banner
        trash(".announcement-bar"),
        // The site header section: collect its navigation links.
        segment(
            "header#shopify-section-header",
            "header_menu",
            vec![collection(
                "a",
                "links",
                vec![particle("", "", vec![("href", "href"), ("", "text")])],
            )],
        ),
        // The predictive search drawer: collect its quick/support links.
        segment(
            "predictive-search",
            "search",
            vec![collection(
                "a",
                "links",
                vec![particle("", "", vec![("href", "href"), ("", "text")])],
            )],
        ),
        // remove all SVGs (minisforum has a lot of them, mostly icons and payment logos)
        trash("svg"),
        // The product gallery
        segment(
            "product-gallery",
            "gallery",
            vec![
                particle("product-gallery", "", vec![("form", "form")]),
                particle("media-carousel", "", vec![("id", "id")]),
                trash("page-dots.page-dots"),
                collection(
                    "div.product-gallery__media",
                    "media",
                    vec![
                        particle(
                            "div.product-gallery__media",
                            "",
                            vec![("data-media-id", "data-media-id")],
                        ),
                        particle(
                            "img",
                            "",
                            vec![
                                ("src", "src"),
                                ("alt", "alt"),
                                ("srcset", "srcset"),
                                ("width", "width"),
                                ("height", "height"),
                                ("sizes", "sizes"),
                                ("fetchpriority", "fetchpriority"),
                                ("loading", "loading"),
                            ],
                        ),
                    ],
                ),
                // TBD: is this needed? They are same as images in media collection
                collection(
                    "page-dots.product-gallery__thumbnail-list > button",
                    "thumbnails",
                    vec![
                        particle(
                            "button",
                            "",
                            vec![
                                ("aria-label", "aria-label"),
                                ("aria-current", "aria-current"),
                            ],
                        ),
                        particle(
                            "img",
                            "",
                            vec![
                                ("src", "src"),
                                ("alt", "alt"),
                                ("srcset", "srcset"),
                                ("width", "width"),
                                ("height", "height"),
                                ("sizes", "sizes"),
                            ],
                        ),
                    ],
                ),
                particle(
                    "page-dots.product-gallery__thumbnail-list",
                    "",
                    vec![("aria-controls", "")],
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
                    collection(
                        "input",
                        "values",
                        vec![particle("", "", vec![("value", "value")])],
                    ),
                ],
            )],
        ),
        // Shopify's `const product = {...}` JS object — the full product:
        // pricing, availability, variants and media (prices are in cents). Use
        // the `const` form; the `window.Rivo.common.product = {…}` copy uses
        // unquoted JS keys and is not valid JSON.
        json_after(
            "script",
            "const product =",
            "product",
            vec![
                ("id", "id"),
                ("title", "title"),
                ("handle", "handle"),
                ("vendor", "vendor"),
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
            ],
        ),
        // Product specifications. One row per heading, each holding a `label`
        // and a `values` vector (multi-column charts have several values).
        // `feature_chart::transpose` turns these rows into column-major `features`.
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
        ),




        // Slideshow sections (promo banners — image only, no links or text).
        // One item per slideshow, each holding its slide images.
        collection(
            "section.shopify-section--slideshow",
            "slideshows",
            vec![
                // Strip the section's template-id `id` (blanked to `::`).
                particle("", "", vec![("id", "")]),
                // Strip the carousel's template-id `id` too.
                particle("slideshow-carousel", "", vec![("id", "")]),
                collection(
                    "img",
                    "images",
                    vec![particle(
                        "img",
                        "",
                        vec![
                            ("src", "src"),
                            ("alt", "alt"),
                            ("width", "width"),
                            ("height", "height"),
                            ("srcset", "srcset"),
                            ("sizes", "sizes"),
                            ("fetchpriority", "fetchpriority"),
                        ],
                    )],
                ),
            ],
        ),


        // Rich-text sections (marketing copy — styled paragraphs, no links/imgs).
        // One item per section, each holding its paragraphs of text.
        collection(
            "section.shopify-section--rich-text",
            "richtexts",
            vec![
                // Strip the section's template-id `id` (blanked to `::`).
                particle("", "", vec![("id", "")]),
                // Strip the wrapper's template `data-section-id`.
                particle("div.rich-text.justify-center", "", vec![("data-section-id", "")]),
                collection(
                    "p",
                    "paragraphs",
                    vec![particle("", "", vec![("", "value")])],
                ),
            ],
        ),
        // Image-with-text-overlay section: styled paragraphs + images. NOTE: a
        // page can have several of these; as a segment, only the first matches.
        segment(
            "section.shopify-section--image-with-text-overlay",
            "image_with_text",
            vec![
                // Strip the section's template-id `id` (blanked to `::`).
                particle("", "", vec![("id", "")]),
                // The styled title; blank its inline `style` (template colors).
                particle("h1", "title", vec![("", "value"), ("style", "")]),
                collection(
                    "p",
                    "paragraphs",
                    vec![particle("", "", vec![("", "value")])],
                ),
                collection(
                    "img",
                    "images",
                    vec![particle(
                        "img",
                        "",
                        vec![
                            ("src", "src"),
                            ("alt", "alt"),
                            ("width", "width"),
                            ("height", "height"),
                            ("srcset", "srcset"),
                            ("sizes", "sizes"),
                            ("loading", "loading"),
                            ("fetchpriority", "fetchpriority"),
                        ],
                    )],
                ),
            ],
        ),



        // "Images and text scrolling" sections: feature highlights combining
        // styled paragraphs and images (mobile + desktop copies, so deduped
        // downstream). One item per section.
        collection(
            "section.shopify-section--images-and-text-scrolling",
            "text_and_images",
            vec![
                // Strip the section's template-id `id` (blanked to `::`).
                particle("", "", vec![("id", "")]),
                collection(
                    "p",
                    "paragraphs",
                    vec![particle("", "", vec![("", "value")])],
                ),
                collection(
                    "img",
                    "images",
                    vec![particle(
                        "img",
                        "",
                        vec![
                            ("src", "src"),
                            ("alt", "alt"),
                            ("width", "width"),
                            ("height", "height"),
                            ("srcset", "srcset"),
                            ("sizes", "sizes"),
                            ("loading", "loading"),
                            ("fetchpriority", "fetchpriority"),
                        ],
                    )],
                ),
            ],
        ),


        //
        //         collection("div.shopify-section","todos",vec![
        //              particle("div.shopify-section","item",vec![("id","id")]),
        //         ]),







        // FAQ section (on limited products only)
        segment(
            "section.shopify-section--faq",
            "faq",
            vec![collection(
                ".accordion.group",
                "items",
                vec![
                    particle(".accordion__toggle", "question", vec![("", "value")]),
                    particle(".accordion__content", "answer", vec![("", "value")]),
                ],
            )],
        ),
        // Nothing interesting in static text section.
        trash("section#shopify-section-static-text-with-icons"),
        // Nothing interesting in the footer section.
        trash("footer"),
    ])
}

pub fn old_architecture() -> RetailerArchitecture {
    RetailerArchitecture::new(vec![
        segment(
            "head",
            "head",
            vec![
                collection(
                    r#"meta"#,
                    "meta_tags",
                    vec![particle(
                        "meta",
                        "",
                        vec![
                            ("property", "property"),
                            ("content", "content"),
                            ("name", "name"),
                        ],
                    )],
                ),
                collection(
                    r#"link"#,
                    "links",
                    vec![particle(
                        "link",
                        "",
                        vec![
                            ("href", "href"),
                            ("rel", "rel"),
                            ("as", "as"),
                            ("type", "type"),
                        ],
                    )],
                ),
                collection(
                    r#"script"#,
                    "scripts",
                    vec![particle(
                        "script",
                        "",
                        vec![
                            ("", "content"),
                            ("src", "src"),
                            ("type", "type"),
                            ("id", "id"),
                        ],
                    )],
                ),
                collection(
                    r#"style"#,
                    "styles",
                    vec![particle("style", "", vec![("", "content"), ("id", "id")])],
                ),
                particle("title", "", vec![("", "title")]),
            ],
        ),
        segment(
            "body",
            "body",
            vec![
                collection(
                    r#"script"#,
                    "scripts",
                    vec![particle(
                        "script",
                        "",
                        vec![
                            ("", "content"),
                            ("src", "src"),
                            ("type", "type"),
                            ("id", "id"),
                        ],
                    )],
                ),
                collection(
                    r#"style"#,
                    "styles",
                    vec![particle("style", "", vec![("", "content"), ("id", "id")])],
                ),
                particle("title", "", vec![("", "title")]),
            ],
        ),
        segment(
            ".announcement-bar",
            "announcement_bar",
            vec![collection(
                "a",
                "links",
                vec![particle("", "", vec![("href", "href"), ("", "text")])],
            )],
        ),
        segment(
            "div.header__wrapper",
            "header",
            vec![collection(
                "a",
                "links",
                vec![particle("", "", vec![("href", "href"), ("", "text")])],
            )],
        ),
        // The site footer (`<footer>` itself is an empty wrapper; the content
        // lives in the footer section). Extract all its links.
        segment(
            ".shopify-section--footer",
            "footer",
            vec![
                trash("svg"),
                collection(
                    "a",
                    "links",
                    vec![particle("", "", vec![("href", "href"), ("", "text")])],
                ),
            ],
        ),
        // The first <section> in <main>. Extract all its links.
        segment(
            "main section",
            "main_section",
            vec![collection(
                "a",
                "links",
                vec![particle("", "", vec![("href", "href"), ("", "text")])],
            )],
        ),
        // The static "text with icons" section (an id, not a class). Collect the
        // text of each item.
        segment(
            "#shopify-section-static-text-with-icons",
            "text_with_icons",
            vec![collection(
                ".text-with-icons__item",
                "items",
                vec![particle("", "", vec![("", "value")])],
            )],
        ),
        // Remove all SVGs inside the payment icon box.
        trash("div.payment_box_svg svg"),
        // The feature/spec chart: one row per heading, each holding a `label`
        // and a `values` vector (multi-column charts have several values).
        segment(
            "feature-chart",
            "feature_chart",
            vec![collection(
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
            )],
        ),
        segment(
            "product-gallery",
            "gallery",
            vec![
                particle("product-gallery", "", vec![("form", "form")]),
                trash("svg"),
                collection(
                    "div.product-gallery__media",
                    "media",
                    vec![particle(
                        "div.product-gallery__media",
                        "",
                        vec![("data-media-id", "data-media-id")],
                    )],
                ),
                collection(
                    r#"img"#,
                    "images",
                    vec![particle(
                        "img",
                        "",
                        vec![
                            ("src", "src"),
                            ("alt", "alt"),
                            ("srcset", "srcset"),
                            ("width", "width"),
                            ("height", "height"),
                        ],
                    )],
                ),
            ],
        ),
        segment(
            ".product-info",
            "product",
            vec![
                particle("h1.product-info__title", "title", vec![("", "value")]),
                // Drop the sr-only "Sale price" / "Regular price" labels so the
                // price particles read just the amount.
                trash("sale-price .sr-only"),
                trash("compare-at-price .sr-only"),
                particle("sale-price", "sale_price", vec![("", "value")]),
                particle("compare-at-price", "compare_at_price", vec![("", "value")]),
                particle("on-sale-badge", "savings", vec![("", "value")]),
                particle("input[name=\"id\"]", "variant_id", vec![("value", "value")]),
                collection(
                    r#".variant-picker__option"#,
                    "variants",
                    vec![
                        particle("legend", "label", vec![("", "value")]),
                        collection(
                            "variant-option-value",
                            "values",
                            vec![particle("", "", vec![("", "value")])],
                        ),
                    ],
                ),
            ],
        ),
        json(
            r#"script[type="application/ld+json"]"#,
            "schema",
            vec![
                ("sku", "sku"),
                ("productID", "product_id"),
                ("brand.name", "brand"),
                ("offers.price", "price"),
                ("offers.priceCurrency", "currency"),
                ("offers.availability", "availability"),
                ("offers.priceValidUntil", "price_valid_until"),
                ("offers.name", "offer_name"),
            ],
        ),
        // Each <form> as a separate section: its action plus its inputs.
        collection(
            "form",
            "forms",
            vec![
                particle("", "", vec![("action", "action")]),
                collection(
                    "input",
                    "inputs",
                    vec![particle("", "", vec![("name", "name"), ("value", "value")])],
                ),
            ],
        ),
        // Every `page-dots` carousel control: extract its `aria-controls`.
        collection(
            "page-dots",
            "page_dots",
            vec![
                particle("", "", vec![("aria-controls", "value")]),
                trash("button"),
            ],
        ),
        collection(
            "media-carousel",
            "media-carousel",
            vec![particle(
                "",
                "",
                vec![("id", "id"), ("data-media-id", "data-media-id")],
            )],
        ),
        // Shopify stamps a per-page template id into many attributes as
        // `template--<id>__…` (the number differs per page). Anchor on the
        // stable `template--` substring to find those places generically; one
        // collection per attribute (CSS has no "any attribute contains X").

        // collection(
        //     r#"[id*="template--"]"#,
        //     "ref_id",
        //     vec![particle("", "", vec![("id", "value")])],
        // ),
        // collection(
        //     r#"[form*="template--"]"#,
        //     "ref_form",
        //     vec![particle("", "", vec![("form", "value")])],
        // ),
        // collection(
        //     r#"[value*="template--"]"#,
        //     "ref_value",
        //     vec![particle("", "", vec![("value", "value")])],
        // ),
        // collection(
        //     r#"[for*="template--"]"#,
        //     "ref_for",
        //     vec![particle("", "", vec![("for", "value")])],
        // ),
        // collection(
        //     r#"[aria-controls*="template--"]"#,
        //     "ref_aria_controls",
        //     vec![particle("", "", vec![("aria-controls", "value")])],
        // ),
        // collection(
        //     r#"[data-section-id*="template--"]"#,
        //     "ref_data_section_id",
        //     vec![particle("", "", vec![("data-section-id", "value")])],
        // ),
        // collection(
        //     r#"[class*="template--"]"#,
        //     "ref_class",
        //     vec![particle("", "", vec![("class", "value")])],
        // ),
        // collection(
        //     r#"[data-target*="template--"]"#,
        //     "ref_data_target",
        //     vec![particle("", "", vec![("data-target", "value")])],
        // ),

        //
    ])
}
