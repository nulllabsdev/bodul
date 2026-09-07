//! Page architecture for Anker Italy (`www.ankeritaly.com`).
//!
//! Headless Next.js storefront backed by Shopify — same family as `crate::retailers::anker::eu`.
//! Product data is carried by `script#__NEXT_DATA__` (`props.pageProps.product`),
//! the JSON-LD blocks and the OpenGraph head meta.  The DOM renders the buy box
//! (price), per-product A+ marketing sections (AplusCarousel slick),
//! a judge.me reviews widget, and breadcrumbs.

use crate::parsing::structure::RetailerArchitecture;
use crate::parsing::structure::{collection, json, particle, segment, trash};

pub fn offer_detail_architecture_v1() -> RetailerArchitecture {
    RetailerArchitecture::new(vec![
        trash("svg"),
        trash(r#"script[src^="/_next/"]"#),
        trash(r#"link[href^="/_next/"]"#),
        trash("style"),
        trash("noscript"),
        // The Next.js payload: full Shopify product record plus shop/SEO context.
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
                ("props.pageProps.product.vendor", "vendor"),
                ("props.pageProps.product.productType", "product_type"),
                ("props.pageProps.product.description", "description"),
                ("props.pageProps.product.availableForSale", "available_for_sale"),
                ("props.pageProps.product.price.amount", "price"),
                ("props.pageProps.product.price.currencyCode", "currency"),
                ("props.pageProps.product.images[].url", "image_url"),
                ("props.pageProps.product.variants[].id", "variant_id"),
                ("props.pageProps.product.variants[].sku", "sku"),
                ("props.pageProps.product.variants[].barcode", "barcode"),
                ("props.pageProps.product.variants[].title", "variant_title"),
                ("props.pageProps.product.variants[].price", "variant_price"),
                ("props.pageProps.product.variants[].compareAtPrice", "compare_at_price"),
                (
                    "props.pageProps.product.variants[].availableForSale",
                    "variant_available",
                ),
                (
                    "props.pageProps.product.variants[].quantityAvailable",
                    "quantity_available",
                ),
                ("props.pageProps.product.options[].id", "option_id"),
                ("props.pageProps.product.options[].name", "option_name"),
            ],
        ),
        // JSON-LD blocks: Product, BreadcrumbList, Corporation.
        collection(
            r#"script[type="application/ld+json"]"#,
            "schemas",
            vec![json(
                "script",
                "",
                vec![
                    ("@type", "type"),
                    // Product
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
                    // Corporation
                    ("legalName", "legal_name"),
                    ("url", "url"),
                    ("email", "email"),
                    ("sameAs", "same_as"),
                    ("logo", "logo"),
                    // BreadcrumbList
                    ("itemListElement[].position", "position"),
                    ("itemListElement[].name", "item_name"),
                    ("itemListElement[].item", "item_url"),
                ],
            )],
        ),
        // Head: title, description/robots meta, OpenGraph tags, canonical,
        // per-product hreflang alternates and preloaded gallery images.
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
        // Brand-link bar (Anker / Solix / Eufy / soundcore logos).
        segment(r#"div[id="__next"] > div:first-child"#, "board_logo", vec![]),
        // Announcement banner with swiper carousel — marketing text, no product data.
        segment("#announcementBar", "announcement_bar", vec![]),
        // Desktop main navigation.
        segment(
            "#DesktopHeader",
            "navigation",
            vec![collection(
                "a",
                "links",
                vec![
                    particle("", "label", vec![("", "value")]),
                    particle("", "url", vec![("href", "value")]),
                ],
            )],
        ),
        // Mobile navigation shell.
        segment("#MobileHeader", "mobile_nav", vec![]),
        // Cart-drawer shell (client-rendered, initially hidden).
        segment(r#"div.z-\[57\].pointer-events-none"#, "cart_drawer", vec![]),
        // Cookie-consent shell (client-rendered).
        segment("#cookie-consent", "cookie_consent", vec![]),
        // judge.me reviews widget carries the Shopify product id and title.
        collection(
            ".jdgm-review-widget",
            "reviews_widget",
            vec![particle(
                "",
                "widget",
                vec![("data-id", "product_id"), ("data-product-title", "product_title")],
            )],
        ),
        // The product block: buy box, gallery, A+ marketing, everything inside <main>.
        segment(
            "main",
            "product",
            vec![
                // Breadcrumbs: Home / collection / current.
                collection(
                    "div.w-safe > div.mb-\\[12px\\] a",
                    "breadcrumbs",
                    vec![
                        particle("", "name", vec![("", "value")]),
                        particle("", "url", vec![("href", "value")]),
                    ],
                ),
                // Gallery: swiper carousel with thumbnails.
                collection(
                    ".swiper-wrapper img",
                    "gallery",
                    vec![particle("", "", vec![("src", "src"), ("alt", "alt")])],
                ),
                // Product title.
                particle("h1", "title", vec![("", "value")]),
                // judge.me preview badge: rating + review counts.
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
                // Description summary (mobile-collapsed bullet list).
                collection(
                    ".a-unordered-list li",
                    "features",
                    vec![particle("", "text", vec![("", "value")])],
                ),
                // Display price.
                collection(
                    "p.text-\\[24px\\].font-semibold",
                    "prices",
                    vec![particle("", "text", vec![("", "value")])],
                ),
                // Variant picker buttons.
                collection(
                    "button[aria-label]",
                    "variant_choices",
                    vec![particle("", "label", vec![("aria-label", "value")])],
                ),
                // A+ marketing carousel: per-product images from Amazon CDN.
                collection(
                    ".AplusCarousel_slider__3tau1 .slick-slide img",
                    "aplus_images",
                    vec![particle("", "src", vec![("src", "value")])],
                ),
                // A+ carousel tab labels.
                collection(
                    ".AplusCarousel_slider__3tau1 button.sliderBtn",
                    "aplus_tabs",
                    vec![particle("", "text", vec![("", "value")])],
                ),
                // All heading text (A+ banners, marketing headings).
                collection("h2", "h2_headings", vec![particle("", "text", vec![("", "value")])]),
                collection("h3", "h3_headings", vec![particle("", "text", vec![("", "value")])]),
                // All images — captures gallery after trashing and A+ content.
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
                // Remaining product buttons.
                collection("button", "buttons", vec![particle("", "label", vec![("", "value")])]),
                // Catch-all product text.
                particle("", "text", vec![("", "value")]),
            ],
        ),
        // Newsletter signup + footer links (div after </main>).
        segment(
            r#".pb-\[48px\].pt-\[72px\]"#,
            "footer",
            vec![
                particle(".text-pretty span", "newsletter_heading", vec![("", "value")]),
                collection(
                    "a[href]",
                    "links",
                    vec![
                        particle("", "label", vec![("", "value")]),
                        particle("", "url", vec![("href", "value")]),
                    ],
                ),
            ],
        ),
    ])
}
