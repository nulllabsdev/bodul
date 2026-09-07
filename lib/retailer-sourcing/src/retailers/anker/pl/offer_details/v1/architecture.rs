//! Page architecture for Anker PL (`www.anker.com/eu-pl`).
//!
//! Copied from [`crate::retailers::anker::eu::offer_details::v1::architecture`] (`www.anker.com/eu-en`) — same platform
//! check applies, re-verified here against 30 real ankerpl dumps: identical
//! headless Next.js/Shopify storefront, identical DOM markers (`#purchase`,
//! `#productSpecs`, `#faq`, `#reviews`, `Crumbs_root__*`,
//! `ProductOptions_optionsBox__*`, `Text_body__snVk8`,
//! `Manuals_textSmall__*`) and identical `__NEXT_DATA__` JSON shape
//! (`product.price.value`/`price.currencyCode`). Not Mathema and not a
//! Shopify Liquid theme, so it does not share the family scaffold in
//! [`crate::retailers::anker::architecture_v1`]. Product data is carried by `script#__NEXT_DATA__`
//! (`props.pageProps.product`), the JSON-LD blocks and the OpenGraph head
//! meta. The DOM renders the buy box (price only when in stock — sold-out
//! pages show a state label instead), per-product A+ marketing sections with
//! product-specific element ids, specs/FAQ/downloads, a judge.me reviews
//! widget carrying the Shopify product id, and breadcrumbs.
//!
//! Most product content is scoped inside a `main` segment (so anything not
//! given its own selector still collapses the main page to a skeleton), but a
//! few selectors are deliberately top-level instead: on some products,
//! malformed inline markup imported from third-party content (a rich-text
//! editor's `ace-line` blocks, raw Amazon A+ HTML) causes the HTML5 parser to
//! re-parent that content — and everything after it in document order — as a
//! sibling of `<main>` rather than a descendant, so a selector scoped inside
//! `main` silently misses it on exactly those products. See the traps
//! section in `docs/shopify-extraction-report.md`.
//!
//! No PL-specific structural deltas were needed — the ankereu selectors
//! carried over unchanged. Full destructure/valueless/dedupe re-run against
//! all 30 ankerpl dumps found the main valueless pages already leak-free (3
//! distinct skeletons after dedupe, same placeholder-count-only variance
//! pattern as ankereu). A residual scan of the *lifted segment* files (not
//! just the main page) did turn up two real per-product leaks specific to
//! the two MAGGO Qi2 MagSafe charger dumps (`a25m0`, `a25m3`) — both
//! `aria-label` duplicating text already lifted elsewhere by the `labels`
//! collection, matching the "aria-label re-leaks" trap in
//! `docs/EXTRACTION_GUIDE.md` §5.8: the `cricel-card-tab` feature-tab
//! buttons, and a separate mobile-only feature-highlight accordion
//! (`div[role="button"][aria-expanded]`). Both are scrubbed below. The same
//! two leaks exist identically in ankereu's own valueless-segments output
//! (verified against its dumps that carry the same products) — this is a
//! pre-existing gap there too, not a PL-only bug, but `crate::retailers::anker::eu` is out of
//! scope for this pass so it's only fixed here.

use crate::parsing::structure::RetailerArchitecture;
use crate::parsing::structure::{collection, comments, json, particle, scrub, segment, trash};

pub fn offer_detail_architecture_v1() -> RetailerArchitecture {
    RetailerArchitecture::new(vec![
        comments(),
        // Build-hashed Next.js asset references (chunk scripts, css/font
        // preloads) and the head-count bookkeeping meta change with every
        // deploy and per page template — pure build noise, remove it so
        // skeletons compare equal across products and deploys.
        scrub(r#"meta[name="next-head-count"]"#, "content"),
        trash(r#"script[src^="/_next/"]"#),
        trash(r#"link[href^="/_next/"]"#),
        // styled-jsx blocks: their CSS embeds per-product component ids
        // (e.g. `#aplusNavCarousel__1`) — remove them (guide-standard for style).
        trash("style"),
        // Key Features / description content authored via a rich-text editor
        // (`ace-line` divs), as opposed to plain `<li>` markup. Matched at
        // top level, not nested under `main`: on some legacy products a stray
        // tag inside this content (observed: a bare `<meta>` inside an `<li>`)
        // causes the parser to re-parent this block as a sibling of `<main>`
        // rather than a descendant, so a selector scoped inside the `main`
        // segment misses it. A top-level collection catches it either way.
        collection(
            "div[data-docx-has-block-data]",
            "rich_text_blocks",
            vec![particle("", "text", vec![("", "value")])],
        ),
        // The Next.js payload: the full Shopify product record plus shop/SEO
        // context. 1.1 MB — by far the largest value-bearing block on the page.
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
        // JSON-LD blocks: Product, BreadcrumbList, Corporation and (on some
        // products) FAQPage. One combined path set — a scalar path resolves
        // from the first block that carries it.
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
                    // FAQPage
                    ("mainEntity[].name", "question"),
                    ("mainEntity[].acceptedAnswer.text", "answer"),
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
        // Top utility bar: SVG logos linking to the other Anker brand sites.
        segment(
            "div#boardLogo",
            "board_logo",
            vec![collection(
                "a",
                "links",
                vec![particle("", "url", vec![("href", "value")])],
            )],
        ),
        // Promo banner strip (client-rendered; only a gradient shell in dumps).
        segment("div#TopBanner", "top_banner", vec![]),
        // Main navigation. The menu data is client-hydrated from __NEXT_DATA__'s
        // `navCategories`; the server-rendered shell carries only the logo links.
        segment(
            "header#header",
            "navigation",
            vec![collection(
                "a",
                "links",
                vec![particle("", "url", vec![("href", "value")])],
            )],
        ),
        // Footer: marketing claims and link columns.
        segment(
            "footer",
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
        // Cookie-consent shell (client-rendered).
        segment("div#cookie-consent", "cookie_consent", vec![]),
        // Country/region picker dialog — a sibling of <footer>, not nested
        // inside it, so the `footer` segment above misses it. Static content
        // (country/language names), identical on every product.
        segment("div.Footer_dialog_wrap__6u2Wj", "country_selector", vec![]),
        // Specs: label/value pairs. Lifted before the product block so the
        // catch-all marketing collection below cannot collapse its skeleton.
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
        // FAQ accordion: questions and (initially collapsed) answers.
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
        // Downloads: document rows (name / code / date spans + the file link).
        collection(
            "section#productDownloads",
            "downloads",
            vec![collection(
                "a",
                "files",
                vec![
                    particle("", "url", vec![("href", "value")]),
                    collection("span", "fields", vec![particle("", "text", vec![("", "value")])]),
                ],
            )],
        ),
        // Static/client-rendered shells: series intro and product recommends
        // are empty server-side; brand info/recommends carry only fixed brand
        // marketing (stats, press quotes) identical on every product.
        collection(
            "section#seriesIntroduce",
            "series_introduce",
            vec![particle("", "text", vec![("", "value")])],
        ),
        collection("section#brandInfo", "brand_info", vec![]),
        collection("section#brandRecommends", "brand_recommends", vec![]),
        collection("section#productRecommends", "product_recommends", vec![]),
        // judge.me reviews widget: the Shopify product id and title ride on it.
        collection(
            "section#reviews",
            "reviews_widget",
            vec![particle(
                ".jdgm-review-widget",
                "widget",
                vec![("data-id", "product_id"), ("data-product-title", "product_title")],
            )],
        ),
        // Breadcrumbs: Home / collection / current product.
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
        // Floating referral widget (its href embeds the product handle).
        // Top-level, not nested under `main`: the same malformed-markup
        // displacement as `rich_text_blocks` (above) and `marketing` (below)
        // has been observed re-parenting this anchor as a sibling of `<main>`
        // on some products.
        collection(
            "a.IconFixed_link__n9EqW",
            "referral_links",
            vec![
                particle("", "url", vec![("href", "value")]),
                particle("", "text", vec![("", "value")]),
            ],
        ),
        // The product block: buy box, selections drawer and the per-product
        // A+ marketing sections (everything left inside <main>). Kept as one
        // wrapping segment (rather than promoting each sub to top level) so
        // that whatever isn't matched by a specific sub below — stray wrapper
        // divs, anything not yet given a selector — is still lifted out as a
        // whole, collapsing the main page to a minimal skeleton. Only the
        // handful of selectors proven to sometimes escape `main` (above) are
        // top-level; everything else is safe to scope here since the chrome
        // segments earlier in this list already ran and detached header/
        // footer/nav/cookie-consent/country-selector, so these generic-tag
        // selectors (h1/h2/h3/h4/img/source/strong/a[href]) can't match
        // anything inside those regions even though `main` casts a wide net.
        segment(
            "main",
            "product",
            vec![
                // Display price — only rendered when the product is in stock.
                // The buy box is duplicated for the mobile sticky bar, so every
                // price element is a collection, not a single particle.
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
                // Sold-out pages render state labels where the price would be.
                collection(
                    r#"span[class^="text-[24px]"]"#,
                    "availability_labels",
                    vec![particle("", "text", vec![("", "value")])],
                ),
                // Cart CTAs (in-stock pages, duplicated in the mobile bar;
                // `tag` carries an A/B experiment id).
                collection(
                    r#"button[id^="netlify_dtctest"]"#,
                    "cta_buttons",
                    vec![
                        particle("", "", vec![("id", "id"), ("tag", "tag")]),
                        particle("", "label", vec![("", "value")]),
                    ],
                ),
                // judge.me preview badge: rating + review/question counts.
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
                // Every h1 carries product copy (buy-box title, A+ banners).
                collection("h1", "headings", vec![particle("", "text", vec![("", "value")])]),
                // Amazon buy-now widget (sku + widget ids).
                particle(
                    "div#amzn-buy-now",
                    "amazon_widget",
                    vec![
                        ("data-sku", "sku"),
                        ("data-site-id", "site_id"),
                        ("data-widget-id", "widget_id"),
                    ],
                ),
                // "Key Features" accordion entries.
                collection(
                    r#"button[aria-label="Key Features details"] li"#,
                    "key_features",
                    vec![particle("", "text", vec![("", "value")])],
                ),
                // Gallery position counter ("1 / 6" — total varies per product).
                collection(
                    "div.bg-opacity-20 span",
                    "gallery_counter",
                    vec![particle("", "text", vec![("", "value")])],
                ),
                // Gallery overlay badges (discount "18% / OFF" hexagons, present
                // on 13/169 sampled products). Scoped to the badge's own wrapper
                // (`left-[-1px]`) — the previous `div.swiper span` selector
                // also matched the gallery_counter's spans, duplicating them.
                collection(
                    r#"div.absolute.left-\[-1px\] span"#,
                    "gallery_badges",
                    vec![particle("", "text", vec![("", "value")])],
                ),
                // Variant pickers: option name + swatch/text choices.
                collection(
                    "div.ProductOptions_optionsBox__wEred",
                    "options",
                    vec![
                        particle("h2", "name", vec![("", "value")]),
                        collection(
                            "button",
                            "choices",
                            vec![
                                particle("", "", vec![("aria-label", "label"), ("title", "title")]),
                                particle("span", "swatch", vec![("style", "value")]),
                                particle("", "text", vec![("", "value")]),
                            ],
                        ),
                    ],
                ),
                // Sticky page sub-nav: product name + tagline and tab labels.
                segment(
                    "nav.PageNav_navContent__CymnO",
                    "page_nav",
                    vec![collection(
                        "button",
                        "items",
                        vec![particle("", "text", vec![("", "value")])],
                    )],
                ),
                // Promo tags on the buy-box title ("New", sale chips).
                collection(
                    "div.productTags > *",
                    "product_tags",
                    vec![particle("", "text", vec![("", "value")])],
                ),
                // Marketing copy headings outside <section> wrappers (A+ divs,
                // Firework banners, LinkOptions "Discover More ..." headers).
                collection("h2", "h2_headings", vec![particle("", "text", vec![("", "value")])]),
                collection("h3", "h3_headings", vec![particle("", "text", vec![("", "value")])]),
                collection("h4", "h4_headings", vec![particle("", "text", vec![("", "value")])]),
                collection(
                    "strong",
                    "strong_texts",
                    vec![particle("", "text", vec![("", "value")])],
                ),
                // Responsive image sources and images: the gallery, the
                // selections drawer and the A+ sections all use per-product art.
                collection(
                    "source",
                    "sources",
                    vec![particle("", "srcset", vec![("srcset", "value")])],
                ),
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
                // Lightbox anchors around A+ rich images (per-product CDN files).
                collection(
                    r#"a[href^="https://cdn.shopify.com"]"#,
                    "media_links",
                    vec![particle("", "url", vec![("href", "value")])],
                ),
                // Selections-drawer variant subtitle ("Black", "6 ft", …) —
                // utility classes only, so match the exact class string.
                collection(
                    r#"div[class="text-[14px] font-[700] text-[#999]"]"#,
                    "selected_variants",
                    vec![particle("", "text", vec![("", "value")])],
                ),
                // Text labels (drawer product title, quantity chip, static
                // service labels) — Text_body is the storefront's label class.
                collection(
                    "div.Text_body__snVk8",
                    "labels",
                    vec![particle("", "text", vec![("", "value")])],
                ),
                // MAGGO Qi2 charger "cricel-card-tab" feature-tab buttons
                // duplicate their `Text_body` label (already lifted above) as
                // an `aria-label`, with literal `<br>` markup baked into the
                // attribute string — scrub the duplicate so it doesn't
                // re-leak the per-product feature copy.
                scrub("div.cricel-card-tab button", "aria-label"),
                // Mobile-only feature-highlight accordion (same per-product
                // feature copy as the cricel-card-tab tabs, re-rendered as
                // collapsible rows below the gallery on narrow viewports).
                // Its `role="button"` div repeats the already-lifted
                // `Text_body` label as `aria-label` — distinguished from the
                // generic "Close menu" / "Zaufana dostawa" role="button" divs
                // (identical on every product, left alone) by the
                // `aria-expanded` attribute, which only this widget carries.
                scrub(r#"div[role="button"][aria-expanded]"#, "aria-label"),
                // Product-family picker ("Select Your Product"): links to the
                // sibling products. Runs after the sweeps, which have already
                // lifted the pictures/labels inside each link.
                collection(
                    r#"a[href*="select_your_product"]"#,
                    "family_links",
                    vec![particle("", "url", vec![("href", "value")])],
                ),
            ],
        ),
        // Catch-all for the A+ marketing sections (aplusFullVideo,
        // amazonTiles_*, listingBanner_*, per-product one-offs like #tab65w,
        // legacy Amazon-A+-HTML imports like #htmlContainer). Their element
        // ids and entire contents are product-specific: capture id + text,
        // then collapse. Top-level, not nested under the `main` segment:
        // some of these sections contain malformed inline markup (observed:
        // Amazon A+ HTML with raw `<img>`/`<p>`/`<h4>`) that causes the parser
        // to re-parent them as a sibling of `<main>` rather than a descendant,
        // even though they sit textually deep inside `<main>` in the raw
        // markup — the same displacement mechanism as `rich_text_blocks`
        // above. A top-level collection catches them regardless of where
        // they end up in the parsed tree.
        collection(
            r#"section:not(#productSpecs):not(#faq):not(#productDownloads):not(#seriesIntroduce):not(#brandInfo):not(#brandRecommends):not(#productRecommends):not(#reviews)"#,
            "marketing",
            vec![
                particle("", "section_id", vec![("id", "value")]),
                particle("", "text", vec![("", "value")]),
            ],
        ),
    ])
}
