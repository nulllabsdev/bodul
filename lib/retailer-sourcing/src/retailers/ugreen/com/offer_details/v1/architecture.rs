//! Page architecture for UGREEN (`www.ugreen.com`).
//!
//! Shopify store with a custom Liquid theme (neither Mathema nor Next.js).
//! Product data comes from `var meta` (Shopify analytics), the JSON-LD schemas,
//! the `web-pixels-manager` initData block, and the `Viewed Product` tracking
//! event.
//!
//! Deltas vs. the `ugreen_eu` seed (verified against 30 real dumps):
//! - `www.ugreen.com` is the brandsite (`ugreen-brandsite.myshopify.com`), not
//!   a checkout-capable storefront: no dump renders `<variant-picker>`,
//!   `.price`/`.compare-at-price`, or a discount badge — the "Shop Now"
//!   buy-buttons div is always empty in the static HTML (AJAX/JS, redirects
//!   off-site). Those selectors are kept (harmless, match nothing) for parity
//!   with the checkout-capable siblings (`ugreen_eu`/`ugreen_us`/`ugreen_fr`).
//! - The product title is a plain `h1.product__title` (two duplicate copies —
//!   mobile/desktop — both inside `<main>`), never inside `<rte-formatter>`;
//!   the eu-seeded `rte-formatter h1` selector matched nothing.
//! - No breadcrumbs nav exists in any dump; the eu-seeded breadcrumbs selector
//!   matches nothing (harmless).
//! - No judge.me badge (`.jdgm-prev-badge`) in any dump — the reviews section
//!   is a Shopify section shell with 0 rendered content (AJAX-loaded).
//! - Like `ugreen_fr`/`ugreen_us`: the eu seed had no `trash("script")`/
//!   `trash("style")`/`comments()`, so a second Klaviyo `var item = {...}`
//!   tracking block (unquoted-key JS object literal, re-embeds the product
//!   name/price/id), a `<script id="__st">` Shopify session/tracking blob
//!   (embeds the product id/url), and generic Shopify boot/theme JS all
//!   leaked verbatim into the valueless output. Head `<title>`/OG/Twitter meta
//!   also duplicate the product name/description/price per-product and were
//!   entirely unswept. Added head-meta particles, a `var item = ` json_after,
//!   and a blanket script/style/noscript sweep (after all extraction).

use crate::parsing::structure::RetailerArchitecture;
use crate::parsing::structure::{collection, comments, json, json_after, particle, segment, trash};

pub fn offer_detail_architecture_v1() -> RetailerArchitecture {
    RetailerArchitecture::new(vec![
        particle("html", "locale", vec![("lang", "value")]),
        // Skip-to-content accessibility link: a top-level `<a>` sibling right
        // before `#header-group` (not nested inside it), so the `header`
        // chrome segment below never reaches it -- its text ("Skip to
        // content") leaks unblanked otherwise.
        particle(".skip-to-content-link", "skip_to_content", vec![("", "value")]),
        // Head OpenGraph/Twitter/description meta and canonical/oembed links —
        // duplicate the JSON-LD product fields, but differ per product so they
        // must be captured (not left as skeleton residue).
        particle("title", "page_title", vec![("", "value")]),
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
        // hreflang locale-switcher links: each carries the product URL (with a
        // locale path prefix), so they differ per product and must be swept.
        collection(
            r#"link[rel="alternate"][hreflang]"#,
            "alternate_locales",
            vec![particle("", "", vec![("hreflang", "locale"), ("href", "value")])],
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
        // Klaviyo's second "Viewed Product" tracking block: an unquoted-key JS
        // object literal (`var item = { Name: ..., Price: ..., ... }`) that
        // can't be parsed as JSON, but still needs its whole script blanked so
        // the product name/price/id it re-embeds doesn't leak into valueless.
        json_after("script", "var item = {", "klaviyo_item", vec![]),
        // All build/tracking scripts (Shopify boot, `<script id="__st">`
        // session/tracking blob embedding the product id/url, Klaviyo,
        // Pandectes cookie consent, GA4, etc.) and inline `<style>`/
        // `<noscript>` blocks are pure noise once the JSON blocks above are
        // extracted — remove them so the valueless skeleton carries no
        // script/style text. (Ordered after the JSON extractions, which run
        // in list order, so nothing is lost.)
        trash("style"),
        trash("noscript"),
        trash("script"),
        // Chrome: header, overlay dialogs (country selector / "Where to Buy"
        // popup), search modal, footer.
        //
        // `#header-group` (not the bare `<header>` tag) is used: the
        // top-activity-bar / announcement-bar sections render as *siblings*
        // of `<header>` inside `#header-group`, not nested inside it -- a
        // plain `segment("header", ...)` leaves their text (promo banner,
        // country/language names) unblanked in the page.
        //
        // No top-level `nav` segment: on this tenant every `<nav>` already
        // lives inside `#header-group` (verified: 0 dumps have a
        // `nav.breadcrumb-nav`), so a generic nav segment here would just be
        // dead code once `#header-group` is lifted first -- and, per the
        // trap seen on `ugreen_nl`, a stray top-level nav segment silently
        // grabs *any* other `<nav>` a future template change might add
        // outside the header (e.g. a breadcrumb), leaking its text
        // unblanked. Omitted defensively, matching the other UGREEN Shopify
        // siblings.
        segment("#header-group", "header", vec![]),
        // Overlay dialogs: the country/region selector and the "Where to
        // Buy" channel popup render as top-level `<div class="shopify-
        // section-group-overlay-group">` siblings *after* `</main>` (this
        // brandsite has no cart drawer, unlike the checkout-capable
        // siblings) -- neither `header` nor `main` reaches them, so their
        // country/language names and "Where to Buy" copy leak unblanked
        // otherwise. A `collection` (not `segment`) is required: `segment`
        // only ever lifts its *first* match, silently leaving any later
        // sibling with the same class untouched.
        collection(".shopify-section-group-overlay-group", "overlay_chrome", vec![]),
        // Search modal: a `<dialog-component id="search-modal">` sibling
        // placed after `</footer>` -- carries generic "Search"/"Clear"/"View
        // all" UI copy that otherwise leaks unblanked.
        segment("#search-modal", "search_modal", vec![]),
        segment("footer", "footer", vec![]),
        // The product block: title, price, gallery, description, variant picker.
        segment(
            "main",
            "product",
            vec![
                // Breadcrumbs (no dump has a breadcrumbs nav; kept for parity
                // with the checkout-capable siblings — matches nothing here).
                collection(
                    "nav[aria-label='breadcrumbs'] a",
                    "breadcrumbs",
                    vec![
                        particle("", "name", vec![("", "value")]),
                        particle("", "url", vec![("href", "value")]),
                    ],
                ),
                // Product title: plain `h1.product__title` (two duplicate
                // mobile/desktop copies) — never inside `<rte-formatter>` on
                // this brandsite.
                particle("h1.product__title", "title", vec![("", "value")]),
                // Price block (never rendered on this brandsite — the "Shop
                // Now" buttons redirect off-site; kept for parity, matches
                // nothing here).
                particle(".price", "price", vec![("", "value")]),
                particle(".compare-at-price", "compare_at_price", vec![("", "value")]),
                // Discount badge (same: never rendered here).
                particle(".ug-price-discount-tag", "discount", vec![("", "value")]),
                // Variant picker (same: never rendered here).
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
        // Strip every comment, then sweep any script/style left anywhere else
        // in the page — all product-relevant data has already been captured
        // by the particle/json/json_after entries above.
        comments(),
    ])
}
