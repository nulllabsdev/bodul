//! Page architecture for UGREEN US (`us.ugreen.com`).
//!
//! Shopify store with a custom Liquid theme (neither Mathema nor Next.js).
//! Product data comes from `var meta` (Shopify analytics), the JSON-LD schemas,
//! the `web-pixels-manager` initData block, and the `Viewed Product` tracking
//! event.  The DOM uses custom web components (`<product-price>`,
//! `<rte-formatter>`) and standard Shopify `<variant-picker>`.

use crate::parsing::structure::RetailerArchitecture;
use crate::parsing::structure::{collection, comments, json, json_after, particle, segment, trash};

pub fn offer_detail_architecture_v1() -> RetailerArchitecture {
    RetailerArchitecture::new(vec![
        comments(),
        particle("html", "locale", vec![("lang", "value")]),
        particle("title", "page_title", vec![("", "value")]),
        // Skip-to-content accessibility link: a top-level `<a>` sibling right
        // before `#header-group` (not nested inside it), so the `header`
        // chrome segment below never reaches it -- its text ("Skip to
        // content") leaks unblanked otherwise.
        particle(".skip-to-content-link", "skip_to_content", vec![("", "value")]),
        // Head OpenGraph/Twitter/description meta and canonical/oembed links —
        // duplicate the JSON-LD product fields, but differ per product so they
        // must be captured (not left as skeleton residue).
        particle(r#"meta[property="og:url"]"#, "og_url", vec![("content", "value")]),
        particle(r#"meta[property="og:title"]"#, "og_title", vec![("content", "value")]),
        particle(
            r#"meta[property="og:description"]"#,
            "og_description",
            vec![("content", "value")],
        ),
        particle(r#"meta[property="og:image"]"#, "og_image", vec![("content", "value")]),
        particle(
            r#"meta[property="og:image:secure_url"]"#,
            "og_image_secure_url",
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
            r#"meta[property="og:price:amount"]"#,
            "og_price_amount",
            vec![("content", "value")],
        ),
        particle(
            r#"meta[property="og:price:currency"]"#,
            "og_price_currency",
            vec![("content", "value")],
        ),
        particle(
            r#"meta[name="twitter:title"]"#,
            "twitter_title",
            vec![("content", "value")],
        ),
        particle(
            r#"meta[name="twitter:description"]"#,
            "twitter_description",
            vec![("content", "value")],
        ),
        particle(
            r#"meta[name="description"]"#,
            "meta_description",
            vec![("content", "value")],
        ),
        particle(r#"link[rel="canonical"]"#, "canonical", vec![("href", "value")]),
        particle(
            r#"link[rel="alternate"][type="application/json+oembed"]"#,
            "oembed",
            vec![("href", "value")],
        ),
        // "omega" app meta (product id/type/tags/collections) — per-product, must
        // be captured or it leaks unblanked as an attribute value.
        particle(
            r#"meta[property="omega:product"]"#,
            "omega_product_id",
            vec![("content", "value")],
        ),
        particle(
            r#"meta[property="omega:product_type"]"#,
            "omega_product_type",
            vec![("content", "value")],
        ),
        particle(
            r#"meta[property="omega:tags"]"#,
            "omega_tags",
            vec![("content", "value")],
        ),
        particle(
            r#"meta[property="omega:collections"]"#,
            "omega_collections",
            vec![("content", "value")],
        ),
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
        // All build/tracking scripts (Shopify boot, klaviyo, judge.me, Pandectes
        // cookie consent, cart cache, GA4, etc.) and inline `<style>`/`<noscript>`
        // blocks are pure noise once the JSON blocks above are extracted — remove
        // them so the valueless skeleton carries no script/style text. (Ordered
        // after the JSON extractions, which run in list order, so nothing is lost.)
        trash("style"),
        trash("noscript"),
        trash("script"),
        // Chrome: header, footer. There is no standalone top-level `<nav>` — the
        // only `<nav>` outside `<header>` is `nav.breadcrumb-nav` inside `<main>`
        // (all header/account-menu navs are nested inside `<header>` already); a
        // generic `segment("nav", ...)` here would greedily grab the breadcrumb
        // nav instead (its text has no sub-rules to blank it), leaking the
        // current product's title unblanked into a `nav` lifted segment file.
        // `#header-group` (not the bare `<header>` tag): the top-activity-bar
        // countdown/announcement banner ("Summer Sale | Up to 45% Off",
        // shipping/warranty copy) renders as a *sibling* of `<header>` inside
        // `#header-group`, not nested inside it -- a plain
        // `segment("header", ...)` leaves its text unblanked in the page.
        segment("#header-group", "header", vec![]),
        // Overlay dialogs: the country/region selector, the cart drawer, and
        // the "Where to Buy" channel popup render as top-level `<div
        // class="shopify-section-group-overlay-group">` siblings *after*
        // `</main>` -- neither `header`/`footer` nor `main` reaches them, so
        // their country/language names, cart-empty copy ("Your cart is
        // empty"), and "Where to Buy" text leak unblanked otherwise. A
        // `collection` (not `segment`) is required: `segment` only ever
        // lifts its *first* match, silently leaving the other two siblings
        // (same class) untouched.
        collection(".shopify-section-group-overlay-group", "overlay_chrome", vec![]),
        // Search modal: a `<dialog-component id="search-modal">` sibling
        // placed after `</footer>` -- carries generic "Search"/"Clear"/"View
        // all" UI copy that otherwise leaks unblanked.
        segment("#search-modal", "search_modal", vec![]),
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
                // Breadcrumbs. `nav.breadcrumb-nav[aria-label="Breadcrumb"]` — note
                // singular/title-case "Breadcrumb", not "breadcrumbs" as on eu/uk.
                // Only the "Home" crumb is a real link; the current-page crumb is a
                // plain `<span>` with no href, so it needs its own particle or its
                // text leaks through unblanked.
                collection(
                    "nav.breadcrumb-nav a",
                    "breadcrumbs",
                    vec![
                        particle("", "name", vec![("", "value")]),
                        particle("", "url", vec![("href", "value")]),
                    ],
                ),
                particle("nav.breadcrumb-nav > span", "breadcrumb_current", vec![("", "value")]),
                // Product title (inside rte-formatter).
                particle("rte-formatter h1", "title", vec![("", "value")]),
                // Price block: sale price and compare-at (regular) price.
                particle(".price", "price", vec![("", "value")]),
                particle(".compare-at-price", "compare_at_price", vec![("", "value")]),
                // Discount badge.
                particle(".ug-price-discount-tag", "discount", vec![("", "value")]),
                // Variant picker. Each option value is a `<label>` wrapping the
                // radio `input` plus a visible text span (`Space Gray`, `Grey`…) —
                // both must be captured together or the visible span text leaks
                // through unblanked in the valueless output.
                collection(
                    "variant-picker fieldset",
                    "options",
                    vec![
                        particle("legend", "label", vec![("", "value")]),
                        collection(
                            "label.variant-option__button-label",
                            "values",
                            vec![
                                particle("input", "value", vec![("value", "value"), ("aria-label", "aria_label")]),
                                particle(".variant-option__button-label__text", "text", vec![("", "value")]),
                            ],
                        ),
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
                // `h2.product-name` (sticky product nav) duplicates its text into a
                // `title` attribute — capture both or the attribute leaks unblanked.
                collection(
                    "h2",
                    "h2_headings",
                    vec![particle("", "text", vec![("", "value"), ("title", "title")])],
                ),
                collection("h3", "h3_headings", vec![particle("", "text", vec![("", "value")])]),
                // Catch-all product text.
                particle("", "text", vec![("", "value")]),
            ],
        ),
    ])
}
