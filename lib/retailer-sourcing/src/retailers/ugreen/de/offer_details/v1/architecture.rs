//! Page architecture for UGREEN DE (`de.ugreen.com`).
//!
//! Shopify store with a custom Liquid theme (neither Mathema nor Next.js).
//! Product data comes from `var meta` (Shopify analytics), the JSON-LD schemas,
//! the `web-pixels-manager` initData block, and the `Viewed Product` tracking
//! event.  The DOM uses custom web components (`<product-price>`,
//! `<rte-formatter>`) and standard Shopify `<variant-picker>`.

use crate::parsing::structure::RetailerArchitecture;
use crate::parsing::structure::{collection, comments, json, json_after, particle, scrub, segment, trash};

pub fn offer_detail_architecture_v1() -> RetailerArchitecture {
    RetailerArchitecture::new(vec![
        particle("html", "locale", vec![("lang", "value")]),
        // `<title>` carries the product name + shop name -- real per-product text.
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
        // chrome segment below never reaches it -- its text ("Direkt zum
        // Inhalt") leaks unblanked otherwise.
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
        // Shopify page-view tracking (`__st`): leaks the product's page URL and
        // numeric resource id if left unblanked.
        json_after(
            "script",
            "var __st=",
            "shopify_tracking",
            vec![("pageurl", "pageurl"), ("rid", "resource_id"), ("p", "page_type")],
        ),
        // Third-party app blocks that embed the current product's id/handle/price
        // as a *JS object literal* (unquoted keys), so `json_after` can't parse
        // fields out of them -- but its blanking side only needs the anchor text
        // to match, so it still fully blanks these and stops the leak.
        json_after("script", "RecentlyViewed.addProduct(", "recently_viewed", vec![]),
        json_after("script", "shopifyLiquidValuesApp7Ext = {", "liquid_values_app", vec![]),
        json_after("script", "window.aiod_product_data = {", "aiod_product_data", vec![]),
        // Klaviyo "Viewed Product" onsite-tracking script (`#viewed_product`):
        // another JS object literal (unquoted keys) carrying Name/ProductID/
        // Price/etc -- same leak-only-blanking rationale as above.
        json_after("script", "var item = {", "klaviyo_viewed_product", vec![]),
        // Free-gift/cart-upsell app bootstrap script: embeds the full current
        // product object (id, variants incl. price/sku/name) further down in
        // plain JS (no leading `{`/`[` right after a stable anchor), so blank on
        // the whole `<script>` via a marker unique to this block.
        json_after("script", "const initFreeGiftData = ", "free_gift_app", vec![]),
        // All remaining build/tracking scripts (Shopify boot, Klaviyo, judge.me,
        // Pandectes cookie consent, Attribuly, AIOD discount widget, GA4, etc.)
        // and inline `<style>`/`<noscript>` blocks (theme `:root`/`@font-face`
        // CSS, floating-buttons CSS, etc.) are pure noise once the JSON blocks
        // above are extracted -- this tenant had NO sweep at all, so all of it
        // (~90KB/file) leaked verbatim into the valueless output. Remove it so
        // the valueless skeleton carries no script/style text. (Ordered after
        // the JSON extractions, which run in list order, so nothing is lost.)
        comments(),
        trash("style"),
        trash("noscript"),
        trash("script"),
        // Chrome: header, footer. NOTE: every genuine site-nav `<nav>` (menu
        // drawer, header search, account popover/drawer) lives *inside* `header`,
        // so it's already carried out with the `header` segment above. A naive
        // top-level `segment("nav", "nav", ...)` would run after `header` has
        // detached those, leaving `nav.breadcrumb-nav` (which sits in `<main>`,
        // not `<header>`) as the only remaining `<nav>` -- wrongly lifting the
        // breadcrumb (and leaking its un-blanked current-page label) as generic
        // chrome. The breadcrumb is handled below, scoped inside `main`, instead.
        // `#header-group` (not the bare `<header>` tag): the top-activity-bar
        // announcement banner ("UGREEN ... Jubiläums-Sale", free-shipping /
        // warranty copy) renders as a *sibling* of `<header>` inside
        // `#header-group`, not nested inside it -- a plain `segment("header",
        // ...)` leaves its text unblanked in the page.
        segment("#header-group", "header", vec![]),
        // Overlay dialogs: the country/region selector, the cart drawer, and
        // the "Where to Buy" channel popup render as top-level `<div
        // class="shopify-section-group-overlay-group">` siblings *after*
        // `</main>` -- neither `header`/`footer` nor `main` reaches them, so
        // their country/language names, cart-empty copy, and "Where to Buy"
        // text leak unblanked otherwise. A `collection` (not `segment`) is
        // required: `segment` only ever lifts its *first* match, silently
        // leaving the other two siblings (same class) untouched.
        collection(".shopify-section-group-overlay-group", "overlay_chrome", vec![]),
        // Search modal: a `<dialog-component id="search-modal">` sibling
        // placed after `</footer>` -- carries generic "Suchen"/"Löschen"/
        // "Alle anzeigen" UI copy that otherwise leaks unblanked.
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
                // Breadcrumbs. The trailing crumb (current page) is a plain
                // `<span>`, not an `<a>` -- include it or its text (the product
                // name) leaks unblanked.
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
                // Variant picker. Each option's `<label>` wraps the `<input>`
                // (radio value/aria-label), a visible text span, and sometimes a
                // swatch `<img>` -- all three carry the option's display value
                // (e.g. "Cosmic Orange"), so all three must be captured/blanked or
                // valueless output re-leaks the variant name via the un-touched
                // siblings of the (separately lifted) `<input>`.
                collection(
                    "variant-picker fieldset",
                    "options",
                    vec![
                        particle("legend", "label", vec![("", "value")]),
                        collection(
                            "label",
                            "values",
                            vec![
                                particle("input", "", vec![("value", "value"), ("aria-label", "aria_label")]),
                                particle(".variant-option__button-label__text", "text", vec![("", "value")]),
                                particle("img", "", vec![("src", "src"), ("alt", "alt")]),
                            ],
                        ),
                    ],
                ),
                // Image responsive-loading attributes duplicate the CDN URL captured
                // by the `images` collection below (`src`/`srcset`/`data_max_resolution`
                // all carry the same image path) -- scrub them BEFORE the collection
                // lifts (and detaches) the `img` elements, otherwise there's nothing
                // left for scrub to match.
                scrub("img", "!srcset"),
                scrub("img", "!data_max_resolution"),
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
