//! Page architecture for UGREEN NL (`nl.ugreen.com`).
//!
//! Shopify store with a custom Liquid theme (neither Mathema nor Next.js).
//! Product data comes from `var meta` (Shopify analytics), the JSON-LD schemas,
//! the `web-pixels-manager` initData block, and the `Viewed Product` tracking
//! event.  The DOM uses custom web components (`<product-price>`,
//! `<rte-formatter>`) and standard Shopify `<variant-picker>`.
//!
//! Per-store delta vs. `ugreen_eu`: this tenant additionally re-leaks the
//! product name/id/price outside the `main` product block via `<title>` and
//! several third-party app scripts (`shopifyLiquidValuesApp7Ext`, the AIOD
//! discount-countdown app's `window.aiod_product_data`, the theme's
//! `RecentlyViewed.addProduct(...)` call, and a ~75 KB Lai Reviews
//! (`dataShop`) app-settings/review-data blob) — closed below with a head
//! title particle and blank-only `json_after` entries anchored on each
//! script's unique marker (their unquoted-key JS object literals don't parse
//! as JSON, so extraction yields nothing for them — expected, matches the
//! "selector right but empty" pattern; the anchor match alone is enough for
//! `valueless` to blank the whole script).

use crate::parsing::structure::RetailerArchitecture;
use crate::parsing::structure::{collection, comments, json, json_after, particle, segment, trash};

pub fn offer_detail_architecture_v1() -> RetailerArchitecture {
    RetailerArchitecture::new(vec![
        particle("html", "locale", vec![("lang", "value")]),
        // Head <title> (re-leaks the product name outside the product block).
        particle("title", "page_title", vec![("", "value")]),
        // Head OpenGraph/Twitter/description meta and canonical/oembed links --
        // duplicate the JSON-LD product fields, but differ per product (real
        // per-product name/description/price/image), so they must be captured,
        // not left as skeleton residue. This tenant had NONE of these captured
        // -- verified: `og:title`/`og:description`/`og:price:amount`/`og:image`
        // leaked the real product name, description, price, and image URL
        // unblanked into every valueless page.
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
        // Skip-to-content accessibility link: a top-level `<a>` sibling right
        // before `#header-group` (not nested inside it), so the `header`
        // chrome segment below never reaches it -- its text ("Ga direct naar
        // de content") leaks unblanked otherwise.
        particle(".skip-to-content-link", "skip_to_content", vec![("", "value")]),
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
        // Theme app-block script: re-declares the product handle/id and the
        // full collection list as an (unquoted-key) JS object literal.
        // Blank-only — extraction can't parse it, but the anchor closes the leak.
        json_after("script", "shopifyLiquidValuesApp7Ext", "liquid_values_app", vec![]),
        // AIOD discount-countdown app: re-declares id/title/price/compareAtPrice
        // as an (unquoted-key) JS object literal. Blank-only, same reason as above.
        json_after("script", "window.aiod_product_data", "aiod_product_data", vec![]),
        // Theme "recently viewed" tracker: re-leaks the numeric product id.
        // Blank-only (no JSON follows the anchor).
        json_after("script", "RecentlyViewed.addProduct", "recently_viewed", vec![]),
        // Lai Reviews app: a large settings + embedded per-product review-data
        // blob assigned to `dataShop`. Blank-only, same reason as above.
        json_after("script", "var dataShop", "lai_reviews", vec![]),
        // Klaviyo's own "Viewed Product" tracker (`#viewed_product`, distinct
        // from the Shopify-native one captured above): re-declares
        // name/id/categories/price/compare-at-price as an unquoted-key JS
        // object literal. No extractable value beyond what's already captured
        // elsewhere, so just remove it outright.
        trash("script#viewed_product"),
        // All remaining build/tracking scripts -- notably the Shopify page-view
        // tracking blob (`var __st={"pageurl":...,"rid":...}`), which re-leaks
        // the *current product's* page URL and numeric resource id per product
        // if left unblanked -- plus Klaviyo, judge.me, Pandectes cookie
        // consent, Attribuly, AIOD discount widget, GA4, etc., and inline
        // `<style>`/`<noscript>` blocks (theme `:root`/`@font-face` CSS, etc.)
        // are pure noise once the JSON blocks above are extracted -- this
        // tenant had no blanket sweep at all. Remove them so the valueless
        // skeleton carries no script/style text. (Ordered after the JSON
        // extractions, which run in list order, so nothing is lost.)
        comments(),
        trash("style"),
        trash("noscript"),
        trash("script"),
        // Chrome: header, overlay dialogs, search modal, footer.
        //
        // `#header-group` (not the bare `<header>` tag): the top-activity-bar
        // announcement banner ("Prime Day-deals" countdown, free-shipping
        // copy) renders as a *sibling* of `<header>` inside `#header-group`,
        // not nested inside it -- a plain `segment("header", ...)` leaves its
        // text unblanked in the page.
        //
        // NO top-level `nav` segment (removed -- was a real bug): once
        // `#header-group`'s navs (menu drawer, account popover) are lifted,
        // the only `<nav>` left in the page is `nav.breadcrumb-nav` inside
        // `<main>`. A generic `segment("nav", "nav", vec![])` here silently
        // grabbed *that* nav first (its content has no sub-rules to blank),
        // lifting the current product's title -- verified: every
        // `data/offers-valueless-segments/ugreennl/nav/*.html` file contained
        // the raw, per-product breadcrumb `<span>` text unblanked. The
        // breadcrumb is instead captured below, scoped inside `main` (see the
        // corrected selector there), matching the other UGREEN Shopify
        // siblings (`ugreen_de`/`ugreen_fr`/`ugreen_us`).
        segment("#header-group", "header", vec![]),
        // Overlay dialogs: the country/region selector, the cart drawer, and
        // the "Where to Buy" channel popup render as top-level `<div
        // class="shopify-section-group-overlay-group">` siblings *after*
        // `</main>` -- neither `header`/`footer` nor `main` reaches them, so
        // their country/language names, cart-empty copy ("Je winkelwagen is
        // leeg"), and "Where te Kopen" text leak unblanked otherwise. A
        // `collection` (not `segment`) is required: `segment` only ever
        // lifts its *first* match, silently leaving the other two siblings
        // (same class) untouched.
        collection(".shopify-section-group-overlay-group", "overlay_chrome", vec![]),
        // Search modal: a `<dialog-component id="search-modal">` sibling
        // placed after `</footer>` -- carries generic "Zoeken"/"Wissen"/"Alles
        // weergeven" UI copy that otherwise leaks unblanked.
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
                // Breadcrumbs. DOM uses `aria-label="Breadcrumb"` (singular,
                // capitalized) via `nav.breadcrumb-nav` -- the previous
                // `nav[aria-label='breadcrumbs']` selector matched nothing.
                // The trailing crumb (current page) is a plain `<span>`, not
                // an `<a>` -- include it or its text (the product name)
                // leaks unblanked.
                collection(
                    "nav.breadcrumb-nav a, nav.breadcrumb-nav span",
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
