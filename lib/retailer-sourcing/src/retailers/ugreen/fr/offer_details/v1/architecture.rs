//! Page architecture for UGREEN FR (`fr.ugreen.com`).
//!
//! Shopify store with a custom Liquid theme (neither Mathema nor Next.js).
//! Product data comes from `var meta` (Shopify analytics), the JSON-LD schemas,
//! the `web-pixels-manager` initData block, and the `Viewed Product` tracking
//! event.  The DOM uses custom web components (`<product-price>`,
//! `<rte-formatter>`) and standard Shopify `<variant-picker>`.
//!
//! Deltas vs. the `ugreen_eu` seed (verified against 30 real dumps):
//! - Breadcrumbs use `aria-label="Breadcrumb"` (singular, capital B) via
//!   `nav.breadcrumb-nav`, and only the first crumb ("Home") is an `<a>` — the
//!   current-page crumb is a bare `<span>` with no `href`. The eu-seeded
//!   selector `nav[aria-label='breadcrumbs'] a` matched nothing.
//! - The eu seed had no `trash("script")`/`trash("style")`/`comments()` at
//!   all, so every non-anchor-matched `<script>`/`<style>` (theme JS, judge.me
//!   CSS, Klaviyo, Pandectes cookie banner, Attribuly, and — critically —
//!   several Shopify app-embed blocks that inline the *current* product's
//!   handle/id/variant into `<script>` JSON, e.g. `shopifyLiquidValuesApp7Ext`,
//!   `hextom_usb`, `_ReStockConfig`) leaked wholesale into the valueless
//!   output (~350KB/file). Added a blanket `trash("script")` + `trash("style")`
//!   sweep (after all extraction) plus `comments()` to actually skeletonize.

use crate::parsing::structure::RetailerArchitecture;
use crate::parsing::structure::{collection, json, json_after, particle, segment, trash};

pub fn offer_detail_architecture_v1() -> RetailerArchitecture {
    RetailerArchitecture::new(vec![
        particle("html", "locale", vec![("lang", "value")]),
        // Skip-to-content accessibility link: a top-level `<a>` sibling right
        // before `#header-group` (not nested inside it), so the `header`
        // chrome segment below never reaches it -- its text ("Ignorer et
        // passer au contenu") leaks unblanked otherwise.
        particle(".skip-to-content-link", "skip_to_content", vec![("", "value")]),
        // Head meta: `<title>` and OG tags leak the current product's name,
        // description and price if left uncaptured (verified: `<title>` text
        // differs per dump and was un-blanked before this was added).
        particle("title", "page_title", vec![("", "value")]),
        particle(r#"meta[name="description"]"#, "description", vec![("content", "value")]),
        particle(r#"meta[property="og:title"]"#, "og_title", vec![("content", "value")]),
        particle(
            r#"meta[property="og:description"]"#,
            "og_description",
            vec![("content", "value")],
        ),
        particle(r#"meta[property="og:url"]"#, "og_url", vec![("content", "value")]),
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
            "og_price",
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
        particle(r#"link[rel="canonical"]"#, "canonical", vec![("href", "value")]),
        particle(
            r#"link[rel="alternate"][type="application/json+oembed"]"#,
            "oembed_url",
            vec![("href", "value")],
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
        // Chrome: header, navigation, footer.
        // Note: on ugreen_fr every <nav> lives either inside <header>
        // (mobile menu-drawer, account-actions) or inside <main> (the
        // breadcrumb, handled below in the product segment) — there is no
        // separate top-level site-nav element, so no standalone nav segment.
        // `#header-group` (not the bare `<header>` tag): the top-activity-bar
        // announcement banner renders as a *sibling* of `<header>` inside
        // `#header-group`, not nested inside it -- a plain
        // `segment("header", ...)` leaves its text unblanked in the page.
        segment("#header-group", "header", vec![]),
        // Overlay dialogs: the country/region selector, the cart drawer, and
        // the "Where to Buy" channel popup render as top-level `<div
        // class="shopify-section-group-overlay-group">` siblings *after*
        // `</main>` -- neither `header`/`footer` nor `main` reaches them, so
        // their country/language names, cart-empty copy ("Vous possédez un
        // compte", "Continuer les achats"), and "Where to Buy" text leak
        // unblanked otherwise. A `collection` (not `segment`) is required:
        // `segment` only ever lifts its *first* match, silently leaving the
        // other two siblings (same class) untouched.
        collection(".shopify-section-group-overlay-group", "overlay_chrome", vec![]),
        // Search modal: a `<dialog-component id="search-modal">` sibling
        // placed after `</footer>` -- carries generic "Rechercher"/"Effacer"/
        // "Tout afficher" UI copy that otherwise leaks unblanked.
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
                // Breadcrumbs: only the leading "Home" crumb is a real link;
                // the current-page crumb is a bare `<span>` (no `href`).
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
                // Remaining product links. Related-product tiles
                // (`.ug-related-product-item`) also carry the recommended
                // product's name/price/id/category in `title` and
                // `data-ga4-product` attributes — extract & blank those too,
                // or they leak past the `label`/`href` capture.
                collection(
                    "a[href]",
                    "links",
                    vec![
                        particle("", "label", vec![("", "value")]),
                        particle("", "url", vec![("href", "value")]),
                        particle("", "title", vec![("title", "value")]),
                        particle("", "ga4_product", vec![("data-ga4-product", "value")]),
                    ],
                ),
                // All headings.
                collection("h2", "h2_headings", vec![particle("", "text", vec![("", "value")])]),
                collection("h3", "h3_headings", vec![particle("", "text", vec![("", "value")])]),
                // Catch-all product text.
                particle("", "text", vec![("", "value")]),
            ],
        ),
        // Strip every comment, then sweep all scripts/styles left anywhere in
        // the page (theme JS, judge.me/Klaviyo/Pandectes/Attribuly boilerplate,
        // and Shopify app-embed blocks that inline the current product's
        // handle/id/variant into JSON) — all product-relevant data has already
        // been captured by the json/json_after/schemas entries above.
        trash("script"),
        trash("style"),
    ])
}
