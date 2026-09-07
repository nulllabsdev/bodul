//! Page architecture for UGREEN NAS (`nas.ugreen.com`).
//!
//! Shopify store on the "t4s" (The4/Roller) theme — same broad platform family
//! as UGREEN EU but a *different* theme: no `rte-formatter`, custom `t4s-*`
//! classes, and a Tailwind-styled `h1.product__title`. Product data comes from
//! the Shopify `var meta` analytics object, the `web-pixels-manager` initData
//! block, and the JSON-LD schemas. Unlike ugreeneu there is **no Product
//! JSON-LD** (only Organization/WebPage/WebSite) and the "Viewed Product" event
//! is a Klaviyo `_learnq.push([… , item])` where `item` is a JS object literal
//! with *unquoted* keys — unparseable, so it is only blanked, not extracted.

use crate::parsing::structure::RetailerArchitecture;
use crate::parsing::structure::{collection, comments, json, json_after, particle, scrub, segment};

pub fn offer_detail_architecture_v1() -> RetailerArchitecture {
    RetailerArchitecture::new(vec![
        comments(),
        particle("html", "locale", vec![("lang", "value")]),
        // JSON-LD: Organization + WebPage + WebSite (no Product schema on this
        // store, but keep product-ish paths for regional variants that add one).
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
                    ("url", "url"),
                    ("logo", "logo"),
                    ("sku", "sku"),
                    ("image", "images"),
                    ("brand.name", "brand"),
                    ("offers.price", "price"),
                    ("offers.priceCurrency", "currency"),
                    ("offers.availability", "availability"),
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
        // web-pixels-manager initData: shop info + variant prices/currency.
        json_after(
            "script",
            "initData:",
            "pixels",
            vec![
                ("shop.name", "shop_name"),
                ("shop.paymentSettings.currencyCode", "currency"),
                ("shop.countryCode", "country"),
                ("productVariants[].id", "variant_id"),
                ("productVariants[].price.amount", "price"),
                ("productVariants[].price.currencyCode", "price_currency"),
                ("productVariants[].sku", "sku"),
            ],
        ),
        // Blank the content of every *inline* `<script>` (Shopify analytics,
        // Klaviyo, Glood, the announcement-bar/countdown widgets, the specs
        // component, `var __st`, ShopifyAnalytics.meta, the GA page tracker, the
        // country switcher, etc.). Across the ten NAS stores these scripts vary
        // by which apps are installed, and many embed the product name / handle /
        // id / price — rather than chase each by a brittle per-app anchor, blank
        // them wholesale. `:not([src])` leaves external `<script src>` tags (and
        // all DOM structure) intact; only inline value-bearing bodies collapse to
        // `!inline_script!`. Extraction is unaffected: the `var meta`/`initData`
        // sources above still parse during `destructure` (blanking is a separate
        // pass), and this entry has no paths so it yields nothing there.
        json("script:not([src])", "inline_script", vec![]),
        // Head product meta: blanked so the valueless main page does not leak the
        // product name/price/urls in SEO tags.
        particle("title", "page_title", vec![("", "value")]),
        scrub(r#"meta[property^="og:"]"#, "content"),
        scrub(r#"meta[name^="twitter:"]"#, "content"),
        scrub(r#"meta[name="description"]"#, "content"),
        scrub(r#"link[rel="canonical"]"#, "href"),
        // `omega:*` SEO-app meta (product id, tags, collection ids) carries
        // per-product data; the oembed alternate link carries the handle.
        scrub(r#"meta[property^="omega:"]"#, "content"),
        scrub(r#"link[type="application/json+oembed"]"#, "href"),
        scrub(r#"link[type="text/xml+oembed"]"#, "href"),
        // Multi-region stores (notably nas-eu) emit per-locale `hreflang`
        // alternate links whose urls carry the product handle.
        scrub("link[hreflang]", "href"),
        // Chrome: header, navigation, footer.
        segment("header", "header", vec![]),
        segment("nav", "nav", vec![]),
        segment("footer", "footer", vec![]),
        // The product block: title, price, breadcrumbs, gallery, description.
        segment(
            "main",
            "product",
            vec![
                // Breadcrumbs.
                collection(
                    "nav.t4s-pr-breadcrumb a",
                    "breadcrumbs",
                    vec![
                        particle("", "name", vec![("", "value")]),
                        particle("", "url", vec![("href", "value")]),
                    ],
                ),
                // Product title.
                particle("h1.product__title", "title", vec![("", "value")]),
                // Price block.
                particle(".product__price .price", "price", vec![("", "value")]),
                // Gallery images.
                collection(
                    "img",
                    "images",
                    vec![particle("", "", vec![("src", "src"), ("alt", "alt")])],
                ),
                // Product links.
                collection(
                    "a[href]",
                    "links",
                    vec![
                        particle("", "label", vec![("", "value")]),
                        particle("", "url", vec![("href", "value")]),
                    ],
                ),
                // Headings.
                collection("h2", "h2_headings", vec![particle("", "text", vec![("", "value")])]),
                collection("h3", "h3_headings", vec![particle("", "text", vec![("", "value")])]),
                // Catch-all product text.
                particle("", "text", vec![("", "value")]),
            ],
        ),
    ])
}
