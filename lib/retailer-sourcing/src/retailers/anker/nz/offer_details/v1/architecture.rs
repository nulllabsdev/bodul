//! Page architecture for Anker NZ (`www.anker.com/nz`).
//!
//! Headless Next.js storefront backed by Shopify — not Mathema and not a
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
//! Copied from `crate::retailers::anker::eu` (same platform, same DOM markers verified
//! against real dumps) with NZ-specific chrome deltas:
//!
//! - The header wrapper carrying `id="header"` is a plain `<div>` here, not a
//!   `<header>` element like EU/PL, so the inherited `header#header` selector
//!   silently matched nothing (dead code, not a parser trap) until it was
//!   fixed to `div#header` in the `navigation` segment.
//! - The top nav's category menu ("Power Banks", "Chargers", "Bestsellers",
//!   "Deals", "Shop by", "Support", "Anker Prime", "Hubs&Docks") renders
//!   server-side here, inside that same `div#header` (`nav_main`), rather
//!   than being purely client-hydrated. Its `Header_navMain__ORXRz` class
//!   matches TWO elements — a textless icon-only mobile-control duplicate and
//!   the real menu — so `nav_main` is a `collection`, not a `segment`: a
//!   `segment` binds to the first (textless) match only, which is why it
//!   silently produced nothing at all. `nav_main` runs before `navigation` so
//!   its data is captured before `navigation`'s wholesale sweep of the shared
//!   `div#header` wrapper.
//! - The country/region picker (`country_selector`) and the top nav
//!   (`nav_main`, above) share the same duplication pattern: both render
//!   TWICE per page (a mobile/desktop responsive duplicate with identical
//!   content), so both are `collection`s rather than `segment`s — a
//!   `segment` would silently drop the second copy's text onto the main page.
//! - The cookie/feature banner carries real text (`feature_bar`) rather than
//!   being an empty client-hydrated shell.
//! - Per-product A+ marketing modules are sometimes plain `div#Mpp*` siblings
//!   of `<main>` (`MppChargeFaster`/`MppFirstModule`/`MppRotateCricle`/
//!   `MppSecondModule`, seen on 3/30 sampled products) rather than `<section>`
//!   elements — on one of those products the same malformed-markup
//!   displacement described above pushes them out from inside `<main>` to a
//!   sibling of it. Their text (built almost entirely of `Text_body` divs) is
//!   caught by promoting `labels` to a top-level collection (rather than
//!   scoping it inside `main` like `crate::retailers::anker::eu` effectively does), and their
//!   wrapper elements (plain buttons whose `aria-label` duplicates the
//!   visible copy) are swept up by extending the `marketing` catch-all's
//!   selector to also match `div[id^="Mpp"]`.

use crate::parsing::structure::RetailerArchitecture;
use crate::parsing::structure::{collection, json, particle, scrub, segment, trash};

pub fn offer_detail_architecture_v1() -> RetailerArchitecture {
    RetailerArchitecture::new(vec![
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
        // NZ-specific: unlike EU/PL, the top nav's category menu ("Power
        // Banks", "Chargers", "Bestsellers", "Deals", "Shop by", "Support",
        // "Anker Prime", "Hubs&Docks") renders server-side, inside `div#header`
        // (the real element carrying that id -- see `navigation` below for why
        // `header#header` never matches this tenant at all). The
        // `Header_navMain__ORXRz` class is reused for TWO different elements
        // in the DOM: a mobile/icon-only duplicate of the logo/search/account/
        // cart controls (no anchors, no button text) and, separately, the
        // actual desktop category menu (anchors carrying real `aria-label`/
        // `href`, buttons carrying real text). A `segment` here (which only
        // ever binds to the *first* selector match) silently locked onto the
        // textless icon-only element every time -- matched-but-empty, so its
        // `inner` map stayed empty and the whole `nav_main` key vanished with
        // no error. A `collection` fixes it: it visits both elements, and the
        // icon-only one still naturally yields no fields and is dropped
        // (`extract_all` filters empty items), leaving just the real menu
        // data. Declared before `navigation` (below) so it runs -- and
        // detaches both elements -- before that segment's wholesale sweep of
        // `div#header`.
        collection(
            "div.Header_navMain__ORXRz",
            "nav_main",
            vec![
                collection(
                    "a",
                    "links",
                    vec![
                        particle("", "label", vec![("aria-label", "value")]),
                        particle("", "url", vec![("href", "value")]),
                    ],
                ),
                collection("button", "buttons", vec![particle("", "text", vec![("", "value")])]),
            ],
        ),
        // Main navigation. NZ-specific: the wrapper here is `div#header`, not
        // a `<header id="header">` element like EU/PL, so the tag-qualified
        // `header#header` selector silently matched nothing on this tenant --
        // not a markup-displacement trap, just the wrong tag, and dead code
        // that never lifted this chrome off the main page at all. Fixed to
        // `div#header`. Besides the logo, this wrapper also holds a textless
        // icon-button top bar (desktop: go home/search/cart/menu) and its
        // mobile duplicate -- both already textless, and `nav_main` (above)
        // has already detached its own two elements from inside here -- so
        // the generic `a` sweep below is enough to clear what's left before
        // the whole wrapper is lifted.
        segment(
            "div#header",
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
        // (country/language names), identical on every product. NZ-specific:
        // unlike EU (mostly one instance), this renders TWICE per page (a
        // mobile/desktop responsive duplicate, same content) — a `segment`
        // only lifts the first, leaving the second to leak onto the main
        // page. A `collection` lifts both.
        collection("div.Footer_dialog_wrap__6u2Wj", "country_selector", vec![]),
        // Cookie/feature banner — NZ-specific: unlike EU/PL, this renders with
        // real static text server-side (`div#cookie-consent` above is always
        // an empty shell here; this is the actual banner).
        segment("div.FeatureBar_root__gYqi1", "feature_bar", vec![]),
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
        // Text labels (drawer product title, quantity chip, static service
        // labels, and — on products where a rich-text/A+ import triggers the
        // malformed-markup displacement described above — entire per-product
        // A+ marketing modules, e.g. `div#MppChargeFaster`/`#MppFirstModule`/
        // `#MppSecondModule` sibling divs, which are built almost entirely out
        // of `Text_body` divs). Top-level, not nested under `main`: on the one
        // sampled product where those A+ modules got displaced to a sibling of
        // `<main>`, a `main`-scoped `labels` sweep missed all of their text
        // (headings, captions, stat callouts) even though `Text_body` is the
        // storefront's universal label class everywhere else on the page.
        collection(
            "div.Text_body__snVk8",
            "labels",
            vec![particle("", "text", vec![("", "value")])],
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
        // legacy Amazon-A+-HTML imports like #htmlContainer), plus NZ-specific
        // `div#Mpp*` modules (`MppChargeFaster`/`MppFirstModule`/
        // `MppRotateCricle`/`MppSecondModule`, seen on 3/30 sampled products —
        // per-product marketing widgets built almost entirely of nested
        // `Text_body` divs, already covered individually by the top-level
        // `labels` collection above, but whose own wrapper elements — plain
        // buttons with duplicate copy baked into their `aria-label` attribute,
        // observed on one product with literal unescaped `<`/`>` inside the
        // quoted attribute value — aren't text nodes `labels` can reach).
        // Their element ids and entire contents are product-specific: capture
        // id + text, then collapse whole. Top-level, not nested under the
        // `main` segment: some of these sections contain malformed inline
        // markup (observed: Amazon A+ HTML with raw `<img>`/`<p>`/`<h4>`) that
        // causes the parser to re-parent them as a sibling of `<main>` rather
        // than a descendant, even though they sit textually deep inside
        // `<main>` in the raw markup — the same displacement mechanism as
        // `rich_text_blocks` above. A top-level collection catches them
        // regardless of where they end up in the parsed tree — and, being
        // declared after `labels`, sweeps up whatever wrapper structure is
        // left once `labels` has already pulled the text out.
        collection(
            r#"section:not(#productSpecs):not(#faq):not(#productDownloads):not(#seriesIntroduce):not(#brandInfo):not(#brandRecommends):not(#productRecommends):not(#reviews), div[id^="Mpp"]"#,
            "marketing",
            vec![
                particle("", "section_id", vec![("id", "value")]),
                particle("", "text", vec![("", "value")]),
            ],
        ),
    ])
}
