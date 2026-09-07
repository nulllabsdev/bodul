//! Page architecture for UGREEN JP (`ugreen.jp`).
//!
//! Shopify store running (a variant of) the stock "Dawn" theme — NOT the
//! custom `<rte-formatter>`/`<variant-picker>` theme used by `ugreen_eu`.
//! Confirmed by dump audit: no `rte-formatter`/`variant-picker` anywhere in
//! the 30 dumps; instead `h1.product__title`, `.price` (Dawn price block),
//! `<variant-radios>`/`fieldset` (Dawn's server-rendered swatches), and a
//! `nav.breadcrumb` with `aria-label="breadcrumbs"`. No judge.me widget found
//! (0/30 dumps have `jdgm-*` markup) — reviews are skipped.
//!
//! Product data comes from the same four universal Shopify sources as
//! `ugreen_eu`: JSON-LD (Organization + Product, 2 blocks/page here), `var
//! meta` (Shopify analytics), the `web-pixels-manager` initData block, and
//! the `Viewed Product` tracking event. Field paths are identical to EU's;
//! the JSON-LD `offers` here is an array (`[{...}]`) rather than a bare
//! object, but the dotted-path resolver already steps into the first element
//! of any array it encounters, so the same `offers.price` style paths work
//! unchanged.
//!
//! Chrome extras versus EU/CA: a country/region-selector dialog
//! (`div.ug-select-country-dialog`) sits as a sibling *between* `</header>`
//! and `<main>` — not nested inside either — so it isn't covered by the
//! `header` segment below and, left alone, leaks the full list of country/
//! language names ("Canada", "Deutsch", "Deutschland", "English" ×4,
//! "Europe", ...) as text onto the main valueless page. It renders once per
//! page (not duplicated like some other tenants' pickers), so it's lifted as
//! its own top-level `segment`, not a `collection`.

use crate::parsing::structure::RetailerArchitecture;
use crate::parsing::structure::{collection, json, json_after, particle, segment, trash};

pub fn offer_detail_architecture_v1() -> RetailerArchitecture {
    RetailerArchitecture::new(vec![
        particle("html", "locale", vec![("lang", "value")]),
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
        // web-pixels-manager initData: shop info, page type, current product.
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
        // Noise: a Klaviyo "Viewed Product" tracking script (unquoted JS
        // object literal, not JSON-parseable) and Shopify's `__st` session
        // beacon — both re-leak product name/price/id, already captured by
        // the four universal sources above.
        trash("script#viewed_product"),
        trash("script#__st"),
        // Blanket sweep: this (Dawn-derived) theme scatters inline <style>
        // (media-query blocks for `header-drawer`/`.menu-drawer-container`/
        // `.list-menu`, `@font-face`) and <script> (`EcomSendApps` tracking,
        // the Shopify module-loader bootstrap, IE-detection, long-animation-
        // frame tracking) throughout <head> and as direct children of
        // `<body>` outside any segment below. None of it is product data,
        // and left alone it leaks raw CSS/JS text into the valueless output.
        // Placed *after* the JSON-LD/meta/pixels/viewed_product/noise
        // structures above (which still need their anchor `<script>` tags
        // intact) but *before* every segment below (`head_meta`, `header`,
        // `footer`, `main`, `country_selector`): `blank.rs::apply()` detaches
        // a top-level segment's subtree as soon as it processes that
        // segment, so a document-wide trash placed here — while the whole
        // document is still attached — is the only way to guarantee it
        // reaches scripts/styles regardless of which chrome region they
        // physically sit in (a trash nested inside one segment can't reach
        // another segment's content, and a trailing top-level trash placed
        // after these segments would find them already detached and lifted,
        // unblanked, into their own segment files).
        trash("script"),
        trash("style"),
        // Head: title, description, OpenGraph tags, Twitter card, canonical.
        segment(
            "head",
            "head_meta",
            vec![
                particle("title", "title", vec![("", "value")]),
                particle(r#"meta[name="description"]"#, "description", vec![("content", "value")]),
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
                    r#"meta[name="twitter:card"]"#,
                    "twitter_card",
                    vec![("content", "value")],
                ),
                particle(
                    r#"meta[name="twitter:site"]"#,
                    "twitter_site",
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
                    "oembed",
                    vec![("href", "value")],
                ),
            ],
        ),
        // Chrome: header (includes the inline nav menu and menu-drawer nav
        // as nested content) and footer.
        segment("header", "header", vec![]),
        segment("footer", "footer", vec![]),
        // Cart drawer (Shopify Dawn-theme `<cart-drawer>` custom element): a
        // sibling of `<body>`, not nested in header/footer/main. Static UI
        // labels only (empty-cart copy, checkout button text) — no product
        // data.
        segment("cart-drawer", "cart_drawer", vec![]),
        // Country/region-selector dialog: a sibling *between* `</header>` and
        // `<main>`, not nested in either, so it survives both chrome segments
        // above untouched. Renders once per page (unlike some other tenants'
        // duplicated pickers), so a plain `segment` (not `collection`) is
        // enough. Static, locale-list content — identical across products.
        segment("div.ug-select-country-dialog", "country_selector", vec![]),
        // The product block: title, price, gallery, description, specs, variants.
        segment(
            "main",
            "product",
            vec![
                // Recommended-products carousel: leaks other products' names/images/links.
                trash("section.ug-home-NewProduct"),
                // Breadcrumbs.
                collection(
                    "nav[aria-label='breadcrumbs'] a",
                    "breadcrumbs",
                    vec![
                        particle("", "name", vec![("", "value")]),
                        particle("", "url", vec![("href", "value")]),
                    ],
                ),
                // Product title.
                particle(".product__title", "title", vec![("", "value")]),
                // Price block: current (sale) price and original (compare-at) price.
                particle(".price-item--sale", "price", vec![("", "value")]),
                particle(
                    ".price__compare .price-item--regular",
                    "compare_at_price",
                    vec![("", "value")],
                ),
                // Description (rich text editor block).
                particle(".product__description", "description", vec![("", "value")]),
                // Variant swatches/options (Dawn's server-rendered radio fieldsets).
                collection(
                    "variant-radios fieldset",
                    "options",
                    vec![
                        particle("legend", "label", vec![("", "value")]),
                        collection("input", "values", vec![particle("", "", vec![("value", "value")])]),
                        collection("label", "value_labels", vec![particle("", "", vec![("", "value")])]),
                    ],
                ),
                // Spec table.
                collection(
                    "table.ug-product-specs tr",
                    "specs",
                    vec![
                        particle("td:first-child", "label", vec![("", "value")]),
                        particle("td:last-child", "value", vec![("", "value")]),
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
