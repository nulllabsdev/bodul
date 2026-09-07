//! Page architecture for Anker Nordics (`www.ankernordics.com`).
//!
//! Headless Next.js storefront backed by Shopify — same broad family as
//! `crate::retailers::anker::eu`/`crate::retailers::anker::italycom`, but a **different theme/build**: none of
//! ankereu's DOM markers (`#purchase`, `#productSpecs`, `#faq`, `#reviews`,
//! `Crumbs_root__*`, `ProductOptions_*`, `Text_body__*`) are present. The DOM
//! theme is the same one `crate::retailers::anker::italycom` targets (swiper gallery,
//! `AplusCarousel`/`AplusFullImage` marketing, a fixed mobile buy bar), so this
//! module mirrors that structured-DOM style rather than ankereu's id-based one.
//!
//! Data sources, in extraction priority:
//!  1. `script#__NEXT_DATA__` (`props.pageProps`) — the full Shopify product
//!     record (title/handle/id/vendor/type/description/price/currency/
//!     availability/images/variants incl. sku+barcode+qty/options) plus shop
//!     context and a rich `seo` block (canonical, languageAlternates,
//!     openGraph). ~12.6 MB of the ~14.7 MB page is inert `buildProps` inside
//!     this script; the whole script is blanked to one placeholder, never mined.
//!  2. JSON-LD blocks (`script[type="application/ld+json"]`): `Corporation`,
//!     `ImageObject` (image licensing), `Product` (thin offers — price/currency/
//!     low/high/offerCount, no sku/availability), `BreadcrumbList`.
//!  3. Head OpenGraph/meta, canonical and hreflang alternates.
//!  4. The product DOM inside `<main>`: breadcrumbs, swiper gallery, title,
//!     judge.me badge (client-hydrated — only `data-id` in the static dump),
//!     price, colour/variant options, A+ marketing images, buy CTAs.
//!
//! Shape deltas vs ankereu (confirmed against real dumps):
//!  - price is `product.price.amount` (not `.value`); `priceRange`/
//!    `compareAtPriceRange` exist; currency comes from `product.price` /
//!    `shop.moneyFormat` (no `shop.paymentSettings`).
//!  - the rich SEO object lives at `props.pageProps.seo` (canonical, alternates,
//!    openGraph); `product.seo` is only a `{title, description}` stub.
//!  - `product.options[].values[].label` and `variants[].selectedOptions[]` are
//!    two-`[]` paths the resolver can't follow, so option values are read from
//!    the DOM buttons, not the JSON.
//!  - `shopify-section`/`jdgm-prev-badge` strings appear only inside escaped
//!    JSON in `__NEXT_DATA__` (embedded `descriptionHtml`/metafields), NOT as
//!    real DOM — this is not a Liquid-theme hybrid.
//!
//! All inline third-party tracking scripts (reddit/vwo/bing/clarity/gtag/
//! admitad/webgains/partnerboost) and `/_next/` build chunks are `trash`ed after
//! the JSON extractions run, so the valueless skeleton carries no script noise.

use crate::parsing::structure::RetailerArchitecture;
use crate::parsing::structure::{collection, json, particle, scrub, segment, trash};

pub fn offer_detail_architecture_v1() -> RetailerArchitecture {
    RetailerArchitecture::new(vec![
        // Per-deploy build bookkeeping — blank so skeletons compare equal.
        scrub(r#"meta[name="next-head-count"]"#, "content"),
        // The Next.js payload: full Shopify product record plus shop/SEO context.
        json(
            "script#__NEXT_DATA__",
            "next_data",
            vec![
                ("locale", "locale"),
                ("buildId", "build_id"),
                ("props.pageProps.type", "page_type"),
                ("props.pageProps.shop.name", "shop_name"),
                ("props.pageProps.shop.moneyFormat", "money_format"),
                ("props.pageProps.shop.primaryDomain.host", "shop_host"),
                ("props.pageProps.seo.title", "seo_title"),
                ("props.pageProps.seo.description", "seo_description"),
                ("props.pageProps.seo.canonical", "canonical"),
                ("props.pageProps.seo.thumbnail", "seo_thumbnail"),
                ("props.pageProps.seo.openGraph.type", "og_type"),
                ("props.pageProps.seo.languageAlternates[].hrefLang", "alt_hreflang"),
                ("props.pageProps.seo.languageAlternates[].href", "alt_href"),
                ("props.pageProps.product.id", "product_id"),
                ("props.pageProps.product.handle", "handle"),
                ("props.pageProps.product.path", "path"),
                ("props.pageProps.product.title", "title"),
                ("props.pageProps.product.vendor", "vendor"),
                ("props.pageProps.product.productType", "product_type"),
                ("props.pageProps.product.description", "description"),
                ("props.pageProps.product.availableForSale", "available_for_sale"),
                ("props.pageProps.product.updatedAt", "updated_at"),
                ("props.pageProps.product.tags", "tags"),
                ("props.pageProps.product.price.amount", "price"),
                ("props.pageProps.product.price.currencyCode", "currency"),
                ("props.pageProps.product.priceRange.minVariantPrice.amount", "price_min"),
                ("props.pageProps.product.priceRange.maxVariantPrice.amount", "price_max"),
                (
                    "props.pageProps.product.compareAtPriceRange.minVariantPrice.amount",
                    "compare_at_min",
                ),
                ("props.pageProps.product.images[].url", "image_url"),
                ("props.pageProps.product.options[].id", "option_id"),
                ("props.pageProps.product.options[].name", "option_name"),
                ("props.pageProps.product.variants[].id", "variant_id"),
                ("props.pageProps.product.variants[].sku", "sku"),
                ("props.pageProps.product.variants[].barcode", "barcode"),
                ("props.pageProps.product.variants[].title", "variant_title"),
                ("props.pageProps.product.variants[].price.amount", "variant_price"),
                (
                    "props.pageProps.product.variants[].compareAtPrice.amount",
                    "variant_compare_at",
                ),
                (
                    "props.pageProps.product.variants[].availableForSale",
                    "variant_available",
                ),
                (
                    "props.pageProps.product.variants[].currentlyNotInStock",
                    "variant_backorder",
                ),
                (
                    "props.pageProps.product.variants[].quantityAvailable",
                    "quantity_available",
                ),
            ],
        ),
        // JSON-LD blocks: Corporation, ImageObject, Product, BreadcrumbList.
        // Each script is parsed on its own, so a scalar path resolves from that
        // block only (no cross-block merge).
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
                    ("image", "images"),
                    ("brand.name", "brand"),
                    ("offers.price", "price"),
                    ("offers.priceCurrency", "currency"),
                    ("offers.lowPrice", "low_price"),
                    ("offers.highPrice", "high_price"),
                    ("offers.offerCount", "offer_count"),
                    // Corporation / shared url
                    ("url", "url"),
                    ("legalName", "legal_name"),
                    ("email", "email"),
                    ("sameAs", "same_as"),
                    ("logo", "logo"),
                    // ImageObject (image licensing)
                    ("contentUrl", "content_url"),
                    ("creditText", "credit_text"),
                    ("license", "license"),
                    ("creator", "creator"),
                    ("acquireLicensePage", "acquire_license_page"),
                    // BreadcrumbList
                    ("itemListElement[].position", "position"),
                    ("itemListElement[].name", "item_name"),
                    ("itemListElement[].item", "item_url"),
                ],
            )],
        ),
        // Head: title, description/robots meta, OpenGraph tags, canonical,
        // per-product hreflang alternates and the preloaded gallery image.
        segment(
            "head",
            "head_meta",
            vec![
                particle("title", "title", vec![("", "value")]),
                particle(r#"meta[name="description"]"#, "description", vec![("content", "value")]),
                particle(r#"meta[name="robots"]"#, "robots", vec![("content", "value")]),
                particle(r#"meta[name="thumbnail"]"#, "thumbnail", vec![("content", "value")]),
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
        // All build chunks, styled-jsx and third-party tracking scripts are pure
        // noise once the two JSON blocks above are extracted — remove them so the
        // valueless skeleton carries no script/style text. (Ordered after the
        // JSON extractions, which run in list order, so nothing is lost.)
        trash("style"),
        trash("noscript"),
        trash("script"),
        // Top brand bar: SVG logos linking to the sibling Anker-family sites.
        segment(r#"div[id="__next"] > div:first-child"#, "board_logo", vec![]),
        // Announcement banner (swiper carousel of promo slides).
        segment("#announcementBar", "announcement_bar", vec![]),
        // Main navigation (mega-menu labels + logo links).
        segment(
            "header#header",
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
        // Footer: marketing note, service nav and brand link columns.
        segment(
            "footer",
            "footer",
            vec![collection(
                "a",
                "links",
                vec![
                    particle("", "label", vec![("", "value")]),
                    particle("", "url", vec![("href", "value")]),
                ],
            )],
        ),
        // Cookie-consent shell (client-rendered, empty in dumps).
        segment("#cookie-consent", "cookie_consent", vec![]),
        // Fixed overlays that are direct children of #__next: the mobile
        // cart/menu drawer, the country/region picker, the "$10 Gift Card"
        // popup and the scroll-to-top button. All static/empty chrome — lift
        // each so its (identical-across-products) text can't leak.
        collection(r#"div[id="__next"] > div.fixed"#, "overlays", vec![]),
        // The product block: buy box, gallery, options and the A+ marketing
        // sections — everything inside <main>. Specific collections feed rich
        // destructured output and are lifted as components; the trailing
        // catch-all text particle then collapses whatever remains of <main> to a
        // single `_text_`, so every product's main page blanks to one skeleton.
        segment(
            "main",
            "product",
            vec![
                // Breadcrumbs: Home / collection / current.
                collection(
                    r#"div.mb-\[12px\] a"#,
                    "breadcrumbs",
                    vec![
                        particle("", "name", vec![("", "value")]),
                        particle("", "url", vec![("href", "value")]),
                    ],
                ),
                particle(
                    r#"div.mb-\[12px\] > div.text-\[12px\]"#,
                    "current_crumb",
                    vec![("", "value")],
                ),
                // Gallery: swiper thumbnails + main images.
                collection(
                    ".swiper-wrapper img",
                    "gallery",
                    vec![particle("", "", vec![("src", "src"), ("alt", "alt")])],
                ),
                // Product title.
                particle("h1", "title", vec![("", "value")]),
                // judge.me preview badge (client-hydrated: only the Shopify
                // product id is present server-side).
                particle(".jdgm-preview-badge", "reviews_badge", vec![("data-id", "product_id")]),
                // judge.me full reviews widget carries the id + product title.
                particle(
                    ".jdgm-review-widget",
                    "reviews_widget",
                    vec![("data-id", "product_id"), ("data-product-title", "product_title")],
                ),
                // Display price(s) — the buy box and the fixed mobile bar each
                // render one, so this is a collection, not a particle.
                collection(
                    r#"p.text-\[24px\].font-bold"#,
                    "prices",
                    vec![particle("", "text", vec![("", "value")])],
                ),
                // Variant/colour option buttons (`title` = value label).
                collection(
                    "button[title]",
                    "variant_choices",
                    vec![particle("", "", vec![("aria-label", "aria_label"), ("title", "title")])],
                ),
                // A+ marketing carousel images (per-product Amazon CDN art).
                collection(
                    ".AplusCarousel_slider__3tau1 img",
                    "aplus_images",
                    vec![particle("", "src", vec![("src", "value")])],
                ),
                // A+ full-bleed banner images.
                collection(
                    ".AplusFullImage_AplusBgImage__4zjwf img",
                    "aplus_banners",
                    vec![particle("", "src", vec![("src", "value")])],
                ),
                // Marketing/section headings.
                collection("h2", "h2_headings", vec![particle("", "text", vec![("", "value")])]),
                collection("h3", "h3_headings", vec![particle("", "text", vec![("", "value")])]),
                // Responsive image sources and images across gallery + A+ content.
                collection(
                    "source",
                    "sources",
                    vec![particle("", "srcset", vec![("srcset", "value")])],
                ),
                collection(
                    "img",
                    "images",
                    vec![particle("", "", vec![("src", "src"), ("alt", "alt")])],
                ),
                // Remaining product links and buttons.
                collection(
                    "a[href]",
                    "links",
                    vec![
                        particle("", "label", vec![("", "value")]),
                        particle("", "url", vec![("href", "value")]),
                    ],
                ),
                collection("button", "buttons", vec![particle("", "label", vec![("", "value")])]),
                // Catch-all: collapse anything left inside <main> to one token.
                particle("", "text", vec![("", "value")]),
            ],
        ),
    ])
}
