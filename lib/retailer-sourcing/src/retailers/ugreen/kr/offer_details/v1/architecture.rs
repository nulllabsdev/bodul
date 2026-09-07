//! Page architecture for UGREEN KR (`kr.ugreen.com`).
//!
//! Shopify store on the stock Dawn theme (not the custom Liquid theme used by
//! `eu`/`uk`: no `<rte-formatter>`/`<variant-picker>` web components — plain
//! Dawn markup instead, e.g. `h1.product__title`, `.price .price-item`,
//! `nav[aria-label='breadcrumbs']`, `.product__media-item img`). Product data
//! comes from the same four universal Shopify sources as `eu`: the JSON-LD
//! schemas, `var meta` (Shopify analytics), the `web-pixels-manager` initData
//! block, and the `Viewed Product` tracking event — field paths are
//! identical. Pricing/text is Korean-locale (₩ won, Korean strings); no
//! judge.me (`.jdgm-*`) or any other review widget was found in any of the 29
//! dumps, and no `<select>`/swatch/variant-option markup was found either
//! (single-variant products in this dump set) — no reviews/variant-options
//! blocks are included. No country-selector dialog was found in any dump
//! (unlike `ugreen_jp`'s `div.ug-select-country-dialog`).

use crate::parsing::structure::RetailerArchitecture;
use crate::parsing::structure::{collection, json, json_after, particle, scrub, segment, trash};

pub fn offer_detail_architecture_v1() -> RetailerArchitecture {
    RetailerArchitecture::new(vec![
        particle("html", "locale", vec![("lang", "value")]),
        // Head: title/meta/canonical/OG/Twitter carry per-product title,
        // description, price and image — not covered by chrome/product
        // segments below, so blank them explicitly to avoid leaks.
        particle("title", "page_title", vec![("", "value")]),
        particle("link[rel='canonical']", "canonical", vec![("href", "value")]),
        particle(
            "meta[name='description']",
            "meta_description",
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
            "meta[property='og:price:amount']",
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
        particle("link[rel='alternate']", "oembed", vec![("href", "value")]),
        // Shopify request-tracking script: carries per-product reqid/pageurl/rid.
        trash("script#__st"),
        // Responsive image srcset: encodes the per-product CDN filename at
        // every breakpoint width — redundant with `src` (captured below) and
        // not otherwise blanked by the per-attribute particle allowlist.
        scrub("img", "!srcset"),
        // Intrinsic image height varies per source photo's aspect ratio
        // (width stays constant); noise, not product data.
        scrub("img", "!height"),
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
        // Shopify analytics: product + variants (prices in cents/jeon).
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
        // Blanket sweep: this (Dawn-derived) theme scatters inline <style>
        // (media-query blocks, `@font-face`) and <script> (`EcomSendApps`
        // tracking, the Shopify module-loader bootstrap, IE-detection, long-
        // animation-frame tracking) throughout <head> and as direct children
        // of `<body>` outside any segment below. None of it is product data,
        // and left alone it leaks raw CSS/JS text into the valueless output.
        // Placed *after* the JSON-LD/meta/pixels/viewed_product structures
        // above (which still need their anchor `<script>` tags intact) but
        // *before* the `header`/`footer`/`main` segments below:
        // `blank.rs::apply()` detaches a top-level segment's subtree as soon
        // as it processes that segment, so a document-wide trash placed here
        // — while the whole document is still attached — is the only way to
        // guarantee it reaches scripts/styles regardless of which chrome
        // region they physically sit in (a trash nested inside one segment
        // can't reach another segment's content, and a trailing top-level
        // trash placed after these segments would find them already detached
        // and lifted, unblanked, into their own segment files).
        trash("script"),
        trash("style"),
        // Chrome: header (both the mobile drawer nav and the desktop inline
        // menu nav are nested *inside* `<header>` on this theme, unlike eu's
        // theme where nav is a header sibling — lifting `header` alone covers
        // them; a separate top-level `nav` selector would instead grab the
        // breadcrumb `<nav>` inside `main`, since `header` (processed first)
        // has already been detached by the time a `nav` selector runs).
        segment("header", "header", vec![]),
        segment("footer", "footer", vec![]),
        // Cart drawer (Shopify Dawn-theme `<cart-drawer>` custom element): a
        // sibling of `<body>`, not nested in header/footer/main. Static UI
        // labels only (empty-cart copy, checkout button text) — no product
        // data.
        segment("cart-drawer", "cart_drawer", vec![]),
        // The product block: title, price, gallery, description, breadcrumbs.
        segment(
            "main",
            "product",
            vec![
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
                particle("h1.product__title", "title", vec![("", "value")]),
                // Price block: sale price and regular (compare-at) price.
                particle(".price .price-item--sale", "price", vec![("", "value")]),
                particle(".price .price-item--regular", "compare_at_price", vec![("", "value")]),
                // All gallery images.
                collection(
                    ".product__media-item img",
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
