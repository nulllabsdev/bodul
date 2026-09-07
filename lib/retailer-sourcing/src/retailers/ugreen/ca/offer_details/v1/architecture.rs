//! Page architecture for UGREEN Canada (`ca.ugreen.com`).
//!
//! Shopify store on the **T4S theme** (`t4s-*` class prefix everywhere) — a
//! well-known premium Shopify theme, and a different theme family from both
//! `ugreen_eu`'s custom `<rte-formatter>`/`<variant-picker>` build and
//! `ugreen_in`'s stock-Shopify `#product-info` build. Product data comes from
//! the same four universal Shopify sources as the other tenants: JSON-LD
//! (`Product` schema, alongside `Organization`/`WebPage`/`WebSite` schemas
//! that carry no product-unique fields), `var meta` (Shopify analytics), the
//! `web-pixels-manager` `initData` block (has a top-level `page` key here,
//! like EU — unlike IN), and the `"Viewed Product"` tracking event. Locale
//! prefixes (`fr-`, `en-`, ...) appear in dump filenames but chrome/product
//! selectors below key on T4S's stable class names, never on locale text.
//! None of the 30 dumps render a description tab/rte block — this theme's
//! product template simply doesn't have one — so no `description` field is
//! modelled.

use crate::parsing::structure::RetailerArchitecture;
use crate::parsing::structure::{collection, comments, json, json_after, particle, scrub, segment, trash};

pub fn offer_detail_architecture_v1() -> RetailerArchitecture {
    RetailerArchitecture::new(vec![
        comments(),
        particle("html", "locale", vec![("lang", "value")]),
        // JSON-LD: Organization/WebPage/WebSite (no product-unique fields) +
        // the Product schema (sku, mpn, productID, brand, offers).
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
                    ("mpn", "mpn"),
                    ("productID", "product_id"),
                    ("image", "images"),
                    ("brand.name", "brand"),
                    ("offers.sku", "offer_sku"),
                    ("offers.price", "price"),
                    ("offers.priceCurrency", "currency"),
                    ("offers.availability", "availability"),
                    ("offers.itemCondition", "condition"),
                    ("offers.priceValidUntil", "price_valid_until"),
                    ("offers.url", "offer_url"),
                    ("url", "url"),
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
                ("page.pageType", "page_type"),
                ("page.resourceId", "resource_id"),
            ],
        ),
        // web-pixels-manager initData: shop info, page type, current variant.
        json_after(
            "script",
            "initData:",
            "pixels",
            vec![
                ("shop.name", "shop_name"),
                ("shop.paymentSettings.currencyCode", "currency"),
                ("page.pageType", "page_type"),
                ("page.resourceId", "product_id"),
                ("productVariants[].price.amount", "variant_price"),
                ("productVariants[].price.currencyCode", "variant_currency"),
                ("productVariants[].product.title", "product_title"),
                ("productVariants[].product.vendor", "vendor"),
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
        // Blanket sweep: this theme scatters per-block inline <style> (custom-
        // property overrides, badge colours, the sticky-header sentinel) and
        // <script> (swiper carousels, `$(document).ready` handlers, countdown
        // timers, judge.me/Rebuy app blocks) throughout <head>, chrome and the
        // product column — much of it as direct children of <body> outside
        // any segment below (e.g. the `#t4s-hsticky__sentinel` `<style>`
        // between `<body>` and `<header>`, and several `<script>` tags
        // between `</footer>` and `</body>`). None of it is product data, and
        // left alone it leaks raw CSS/JS text into the valueless output.
        // Placed *after* the JSON-LD/meta/pixels/viewed_product structures
        // above (which still need their anchor `<script>` tags intact) but
        // *before* every segment below (`head`, `header`, `nav`, `footer`,
        // `main`): `blank.rs::apply()` detaches a top-level segment's subtree
        // as soon as it processes that segment, so a document-wide trash
        // placed here — while the whole document is still attached — is the
        // only way to guarantee it reaches scripts/styles regardless of which
        // chrome region they physically sit in (a trash nested inside one
        // segment can't reach another segment's content, and a trailing
        // top-level trash placed after these segments would find them already
        // detached and lifted, unblanked, into their own segment files).
        trash("script"),
        trash("style"),
        // Noise: the localization form's `return_to` field echoes the current
        // page URL (differs every product) — scrub rather than model.
        scrub("input[name='return_to']", "value"),
        // Shopify app-block wrapper divs (Zendesk chat snippet, Loox reviews
        // widget loader, Rebuy) — once their inline <script> is trashed above
        // they're empty shells, but Shopify renders them in non-deterministic
        // order per page load, so even empty they'd make every valueless page
        // byte-different from every other. Drop the wrappers entirely; their
        // (fixed, constant) ids and order carry no product data.
        trash(r#"div[id^="shopify-block-"]"#),
        // <head> meta/OG/Twitter tags: title, description, canonical, price,
        // gallery image.
        segment(
            "head",
            "head",
            vec![
                particle("title", "title", vec![("", "value")]),
                particle("link[rel='canonical']", "canonical", vec![("href", "value")]),
                particle("meta[name='description']", "description", vec![("content", "value")]),
                particle(
                    "meta[property='og:site_name']",
                    "og_site_name",
                    vec![("content", "value")],
                ),
                particle("meta[property='og:url']", "og_url", vec![("content", "value")]),
                particle("meta[property='og:title']", "og_title", vec![("content", "value")]),
                particle(
                    "meta[property='og:description']",
                    "og_description",
                    vec![("content", "value")],
                ),
                particle("meta[property='og:image']", "og_image", vec![("content", "value")]),
                particle(
                    "meta[property='og:image:secure_url']",
                    "og_image_secure_url",
                    vec![("content", "value")],
                ),
                particle(
                    "meta[property='og:image:width']",
                    "og_image_width",
                    vec![("content", "value")],
                ),
                particle(
                    "meta[property='og:image:height']",
                    "og_image_height",
                    vec![("content", "value")],
                ),
                particle(
                    "meta[property='product:price:amount']",
                    "og_price_amount",
                    vec![("content", "value")],
                ),
                particle(
                    "meta[name='twitter:title']",
                    "twitter_title",
                    vec![("content", "value")],
                ),
                particle(
                    "meta[name='twitter:description']",
                    "twitter_description",
                    vec![("content", "value")],
                ),
                // Products with a multi-image gallery repeat the
                // og:image/secure_url/width/height meta group once per photo
                // (the particles above only capture the *first* repeat) —
                // scrub the rest so extra gallery URLs don't leak.
                scrub("meta[property='og:image']", "content"),
                scrub("meta[property='og:image:secure_url']", "content"),
                scrub("meta[property='og:image:width']", "content"),
                scrub("meta[property='og:image:height']", "content"),
                // `keywords` embeds the product title (comma-joined with the
                // shop name/domain) — pure duplicate of `title`/`meta.name`.
                scrub("meta[name='keywords']", "content"),
                // EasyGift app config: `itemid` is a base64 `shop_$_<request
                // timestamp>` token that differs on every render (not
                // per-product) — pure noise.
                scrub("meta#easygift-shop", "itemid"),
                scrub("meta#easygift-shop", "content"),
                scrub("link[rel='alternate']", "href"),
            ],
        ),
        // Chrome: site header, primary nav, footer.
        segment("header#shopify-section-header-inline", "header", vec![]),
        segment("nav.t4s-navigation", "nav", vec![]),
        segment("footer#t4s-footer", "footer", vec![]),
        // Skip-to-content accessibility link — a sibling of `<body>`, before
        // the header, so none of the segments above reach it.
        trash("a.skip-to-content-link"),
        // Top promo bar (seasonal sale banner) + the current-country display
        // it hosts (`#ug-top-current-country`) — both siblings of `<body>`
        // before the header. Static per-campaign copy, not per-product.
        segment("div.t4s-top-bar", "top_bar", vec![]),
        // Mobile mega-menu: a sibling of `<body>` *after* the footer (not
        // nested in header/footer/main), so none of the segments above lift
        // it. Static, identical-across-products category/"recommended
        // products" list (same set/prices baked server-side on every page) —
        // acceptable skeleton, but lifted out to declutter the main page.
        segment("div#shopify-section-ug-mb-mega-menu", "mobile_mega_menu", vec![]),
        // The product block: title, price, gallery, breadcrumbs, variant
        // swatches. `main#MainContent` wraps the breadcrumb section, the
        // product info column and any related-product sections.
        segment(
            "main#MainContent",
            "product",
            vec![
                // "Compare similar products" widget (present on some charger/
                // hub products): a table of *other* products' names, images,
                // prices and spec cells ("3 USB-C + 1 USB-A", "✔", ...) — not
                // this product's own data, and the comparison set differs per
                // product page. Drop it whole, same treatment as a
                // recommended-products carousel.
                trash("div#ug-products-compare"),
                // loox's hidden inline reviews cache (`#loox-inline-reviews`):
                // real customer names/review bodies as visible text (`display:
                // none` in the static dump, but present in the DOM) — varies
                // per product and isn't modelled at depth B; the `reviews`
                // aggregate-rating particle above already captures the rating/
                // count summary this tenant exposes.
                trash("div#loox-inline-reviews"),
                // Hidden native `<select>` fallback for the variant picker:
                // its `<option>` text repeats the swatch value names (already
                // captured by the `options`/`values` collection below).
                trash("select.t4s-product__select"),
                // Breadcrumbs: linked crumbs, plus the trailing `<span>` that
                // marks the current page (the product title again, unlinked)
                // — without it, that span's text re-leaks the title outside
                // the `a`-only collection above.
                collection(
                    "nav.t4s-pr-breadcrumb a",
                    "breadcrumbs",
                    vec![
                        particle("", "name", vec![("", "value")]),
                        particle("", "url", vec![("href", "value")]),
                    ],
                ),
                particle("nav.t4s-pr-breadcrumb span", "breadcrumb_current", vec![("", "value")]),
                // Product title.
                particle("h1.t4s-product__title", "title", vec![("", "value")]),
                // Short marketing tagline rendered directly below the title
                // (stable `ts4-pr_shipping__wrap` class; the wrapping div's
                // own class carries a per-block random hash, so it isn't a
                // usable selector).
                particle("div.ts4-pr_shipping__wrap", "tagline", vec![("", "value")]),
                // Price block: sale price (<ins>) and regular price (<del>)
                // when the product is discounted. Non-discounted products
                // render the price as plain text directly inside the div
                // instead (no <ins>/<del> at all) — the fallback particle
                // below catches that case; when <ins>/<del> *do* exist it
                // still runs (harmlessly re-collapsing the already-
                // placeholder'd children into one placeholder) since it's
                // declared last.
                particle("div.t4s-product-price del", "compare_at_price", vec![("", "value")]),
                particle("div.t4s-product-price ins", "price", vec![("", "value")]),
                particle("div.t4s-product-price", "price_raw", vec![("", "value")]),
                // Add-to-cart button label ("Add to cart" / "Out of stock" /
                // "Pre-order", locale-translated).
                particle("span.t4s-btn-atc_text", "add_to_cart_label", vec![("", "value")]),
                // Discount-percentage flag on the gallery image (e.g. "38% OFF").
                particle("span.ug-discount-value", "discount_percent", vec![("", "value")]),
                // loox reviews rating badge.
                particle(
                    ".loox-rating",
                    "reviews",
                    vec![
                        ("data-id", "product_id"),
                        ("data-rating", "average_rating"),
                        ("data-raters", "review_count"),
                    ],
                ),
                // Variant swatches (colour/size options).
                collection(
                    "div.t4s-swatch__option",
                    "options",
                    vec![
                        particle("h4.t4s-swatch__title", "label", vec![("", "value")]),
                        collection(
                            "div[data-swatch-item]",
                            "values",
                            vec![particle("", "value", vec![("data-value", "value")])],
                        ),
                    ],
                ),
                // Short description (bullet-list feature highlights, shown
                // above the fold) and the full description (rich-text
                // "Overview" tab, below it).
                particle("div.ug-product-short-desc", "short_description", vec![("", "value")]),
                particle("div#ug-product-overview", "description", vec![("", "value")]),
                // Sticky tab-nav bar re-renders the product title as plain
                // text (`.product-nav .product-name`) once the shopper
                // scrolls into the tabs section — duplicate of `title` above.
                particle(
                    "div.product-nav .product-name",
                    "product_nav_title",
                    vec![("", "value")],
                ),
                // Spec table (label/value rows).
                collection(
                    "table.ug-product-specs tr",
                    "specs",
                    vec![
                        particle("td:first-child", "label", vec![("", "value")]),
                        particle("td:last-child", "value", vec![("", "value")]),
                    ],
                ),
                // Gallery images: main slider + thumbnail strip.
                collection(
                    "img",
                    "images",
                    vec![particle(
                        "",
                        "",
                        vec![
                            ("src", "src"),
                            ("data-src", "data_src"),
                            ("srcset", "srcset"),
                            ("alt", "alt"),
                        ],
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
                // NOTE: deliberately no catch-all `particle("", "text", ...)`
                // here — an empty-selector particle targets the segment's own
                // root element, and blank.rs's empty-key `set_value` detaches
                // *all* children before inserting its placeholder; placed
                // last it would wipe out every placeholder already produced
                // by the breadcrumbs/swatches/images/links/headings above.
            ],
        ),
    ])
}
