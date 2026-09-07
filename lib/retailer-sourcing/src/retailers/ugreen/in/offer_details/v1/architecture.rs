//! Page architecture for UGREEN India (`ugreenindia.com`).
//!
//! Shopify store on a stock Shopify theme built around `#product-info` /
//! `.product-info__block` markup (title `h1.product-title`, price
//! `.product-info__price`, gallery `#gallery-viewer`) — a different theme
//! family from `ugreen_eu`'s custom `<rte-formatter>`/`<variant-picker>`
//! build (neither element exists on this store). Product data comes from the
//! same four universal Shopify sources as EU: JSON-LD, `var meta`
//! (Shopify analytics), the `web-pixels-manager` `initData` block, and the
//! `"Viewed Product"` tracking event — but `initData`'s shape differs here:
//! there is no top-level `page` key, and the current variant/price live under
//! `productVariants[]` instead of `page.resourceId`. Dumps span three
//! locales (`bn-`, `hi-`, and unprefixed `en`), so chrome nav/breadcrumb
//! `aria-label`s are locale-translated text — selectors below key on class
//! names, never on `aria-label` values. None of the 30 dumps render a
//! `variant-picker`/`fieldset`/`option-selector` — every product here is
//! single-variant, so no variant-options block is modelled.

use crate::parsing::structure::RetailerArchitecture;
use crate::parsing::structure::{collection, json, json_after, particle, scrub, segment, trash};

pub fn offer_detail_architecture_v1() -> RetailerArchitecture {
    RetailerArchitecture::new(vec![
        particle("html", "locale", vec![("lang", "value")]),
        // JSON-LD: Organization + BreadcrumbList + Product schemas.
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
                    ("itemListElement[].name", "breadcrumb_name"),
                    ("itemListElement[].item", "breadcrumb_item"),
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
        // web-pixels-manager initData: shop info + the current product's
        // variant (this tenant has no top-level `page` key, unlike EU).
        json_after(
            "script",
            "initData:",
            "pixels",
            vec![
                ("shop.name", "shop_name"),
                ("shop.paymentSettings.currencyCode", "currency"),
                ("productVariants[].id", "variant_id"),
                ("productVariants[].sku", "sku"),
                ("productVariants[].price.amount", "price"),
                ("productVariants[].price.currencyCode", "price_currency"),
                ("productVariants[].product.id", "product_id"),
                ("productVariants[].product.title", "product_title"),
                ("productVariants[].product.vendor", "vendor"),
                ("productVariants[].product.handle", "handle"),
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
        // Noise: account links carry a per-request signed `buyer_flags` JWT
        // (timestamp-bound, differs every dump), the header/footer localization
        // forms (`#nav-localization`, `#footer-localization`) echo the current
        // page URL, and a large inline theme-translations <script> ends with a
        // "Recently viewed products" localStorage snippet embedding the raw
        // product id (`const item = <id>;`) — none of this is product data, but
        // left alone it causes spurious per-product diffs in otherwise-identical
        // chrome. The script is blanked whole via its anchor (it isn't valid
        // JSON, so this intentionally contributes nothing to `extract`).
        scrub("a.secondary-nav__item", "href"),
        scrub("input[name='return_to']", "value"),
        json_after("script", "cc-recently-viewed", "recently_viewed", vec![]),
        // judge.me embeds full review payloads (author names, review bodies)
        // for *related* products' preview badges in inline scripts — not this
        // product's own data, and the set of related products (so the exact
        // reviews shown) varies per page. Blanked whole, same as above.
        json_after("script", "jdgm.data.reviewWidget[", "related_review_widgets", vec![]),
        // Blanket sweep: this theme scatters inline <style>/<script> (the
        // announcement bar's CSS custom-property block, `var mid = '...'`
        // Snapmint config, quick-add-drawer scripts, `EcomSendApps`/Shopify
        // module-loader/IE-detection/long-animation-frame tracking JS) as
        // direct children of `<body>` outside any segment below — e.g. the
        // announcement-bar `<style>` sits between `<body>` and `<header>`,
        // and several `<script>` tags sit between the quick-add-drawer and
        // `</body>`. None of it is product data, and left alone it leaks raw
        // CSS/JS text into the valueless output. Placed *after* the
        // JSON-LD/meta/pixels/viewed_product/noise structures above (which
        // still need their anchor `<script>` tags intact) but *before* every
        // segment below (`head`, `header.header__grid`, `main-menu`,
        // `footer.footer`, `main`): `blank.rs::apply()` detaches a top-level
        // segment's subtree as soon as it processes that segment, so a
        // document-wide trash placed here — while the whole document is still
        // attached — is the only way to guarantee it reaches scripts/styles
        // regardless of which chrome region they physically sit in (a trash
        // nested inside one segment can't reach another segment's content,
        // and a trailing top-level trash placed after these segments would
        // find them already detached and lifted, unblanked, into their own
        // segment files).
        trash("script"),
        trash("style"),
        // <head> meta/OG/Twitter tags: title, description, canonical, price,
        // gallery image — same values as JSON-LD/meta but rendered separately
        // in <head>, so left uncaptured they'd leak per-product text/prices
        // straight into the valueless page (outside the `main` product segment
        // entirely). hreflang/oembed <link> hrefs carry the product URL too;
        // scrubbed rather than modelled since they're pure duplicates of `url`.
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
                    "meta[property='og:price:amount']",
                    "og_price_amount",
                    vec![("content", "value")],
                ),
                particle(
                    "meta[property='og:price:currency']",
                    "og_price_currency",
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
                scrub("link[rel='alternate']", "href"),
            ],
        ),
        // Chrome: site header, primary/secondary nav, footer. Selectors are
        // scoped to the real site chrome (not the cart/quick-add drawer
        // headers/footers that share the bare `<header>`/`<footer>` tags).
        segment("header.header__grid", "header", vec![]),
        segment("main-menu", "nav", vec![]),
        segment("footer.footer", "footer", vec![]),
        // Cart drawer (Shopify Dawn-theme `<cart-drawer>` custom element): a
        // sibling of `<body>`, not nested in header/footer/main. Static UI
        // labels only (empty-cart copy, checkout button text) — no product
        // data.
        segment("cart-drawer", "cart_drawer", vec![]),
        // Announcement bar (welcome message, help-center/order-tracking
        // links): a sibling of `<body>` before `<header>`. Static marketing
        // copy, identical across products.
        segment("announcement-bar", "announcement_bar", vec![]),
        // judge.me reviews badge data (present on a minority of dumps).
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
        // The product block: title, price, gallery, breadcrumbs, description.
        segment(
            "main",
            "product",
            vec![
                // The quick-add drawer's <template id="quick-buy-template"> repeats
                // the whole product-info block (title/vendor/sku/price) inert in
                // the DOM; drop it before the catch-all text/link particles below
                // re-leak it.
                trash("template"),
                // judge.me's rendered review-widget block (#judgeme_product_reviews)
                // embeds real customer review bodies/author names/other-product
                // links as visible text — genuinely varies dump to dump (unlike
                // the site-wide "recent reviews" carousel elsewhere on the page),
                // so it must be dropped rather than left as assumed-identical
                // residue. The `related_review_widgets`/`reviews`/`reviews_text`
                // structures above already capture the aggregate rating data this
                // tenant exposes; the individual review text itself isn't
                // modelled at depth B.
                trash("#judgeme_product_reviews"),
                // judge.me's server-rendered reviews TAB (a *separate* element
                // from `#judgeme_product_reviews` above, `section.jdgm-revs-
                // tab__wrapper`): the rating-distribution histogram embeds
                // shop-wide review counts/percentages, and the review list
                // below it embeds real customer names, review bodies and
                // links to *other* products' pages — none of it is this
                // product's own data, and it isn't modelled at depth B.
                trash("section.jdgm-revs-tab__wrapper"),
                // Breadcrumbs (class-based: aria-label text is locale-translated).
                collection(
                    "nav.breadcrumbs ol.breadcrumbs-list a.breadcrumbs-list__link",
                    "breadcrumbs",
                    vec![
                        particle("", "name", vec![("", "value")]),
                        particle("", "url", vec![("href", "value")]),
                    ],
                ),
                // Vendor / SKU line. Scoped under `.product-info--sticky` — the
                // same `product-vendor`/`product-sku`/`product-title`/
                // `product-info__price` classes also appear, empty, inside the
                // inert <template id="quick-buy-template"> used by the quick-add
                // drawer; scoping avoids a false duplicate match there.
                particle(
                    ".product-info--sticky span.product-vendor",
                    "vendor",
                    vec![("", "value")],
                ),
                particle(
                    ".product-info--sticky span.product-sku__value",
                    "sku",
                    vec![("", "value")],
                ),
                // Product title.
                particle(".product-info--sticky h1.product-title", "title", vec![("", "value")]),
                // Price block: current price and was-price (compare-at).
                particle(
                    ".product-info--sticky .price__current .js-value",
                    "price",
                    vec![("", "value")],
                ),
                particle(
                    ".product-info--sticky .price__was .js-value",
                    "compare_at_price",
                    vec![("", "value")],
                ),
                // Sale/sold-out badge (present on ~8/30 dumps; absent = in stock).
                particle(
                    ".product-info--sticky .product-label--sold-out",
                    "sold_out_label",
                    vec![("", "value")],
                ),
                particle(
                    ".product-info--sticky .product-label--sale",
                    "sale_label",
                    vec![("", "value")],
                ),
                // Add-to-cart button label ("Add to cart" vs "Sold out").
                particle(
                    ".product-info__add-button button",
                    "add_to_cart_label",
                    vec![("", "value")],
                ),
                // Amazon price-box widget re-embeds barcode/brand/sku/title as
                // data-* attributes — duplicates already captured elsewhere.
                scrub("div.amazon-price-box", "data-barcode"),
                scrub("div.amazon-price-box", "data-brand"),
                scrub("div.amazon-price-box", "data-sku"),
                scrub("div.amazon-price-box", "data-title"),
                // Sticky add-to-cart bar re-renders the title/price/image as
                // plain text once the shopper scrolls past the main block —
                // pure duplicate UI, not worth modelling; drop it whole.
                trash("sticky-atc-panel"),
                // Gallery images: the main slider viewer AND the separate
                // thumbnail strip (`.media-thumbs`) both carry their own <img>
                // tags with the product's alt text — both live under
                // `.media-gallery`, so one selector covers both.
                collection(
                    ".media-gallery img",
                    "images",
                    vec![particle(
                        "",
                        "",
                        vec![("src", "src"), ("srcset", "srcset"), ("alt", "alt")],
                    )],
                ),
                // Description.
                particle("div.rte.product-description", "description", vec![("", "value")]),
                // "Smart Bot" product-highlights badges (short per-product tags
                // like "Fast Data", "Secure Signal").
                collection(
                    "div.smart-bot-product-highlights__item",
                    "highlights",
                    vec![particle("", "text", vec![("", "value")])],
                ),
                // Spec table (dimensions, length, cable type, weight, ...).
                collection(
                    "li.product-spec__item",
                    "specs",
                    vec![
                        particle("div.product-spec__label", "label", vec![("", "value")]),
                        particle("div.product-spec__value", "value", vec![("", "value")]),
                    ],
                ),
                // judge.me's inline review widget re-embeds the product id/title
                // as data-* attributes (duplicates of `title`/`meta.id` above).
                scrub("div.jdgm-widget", "data-id"),
                scrub("div.jdgm-widget", "data-product-id"),
                scrub("div.jdgm-widget", "data-product-title"),
                scrub("div.jdgm-widget", "data-image-url"),
                scrub("div.jdgm-widget", "data-updated-at"),
                scrub("product-recommendations", "data-product-id"),
                scrub("product-recommendations", "data-url"),
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
                // NOTE: deliberately no catch-all `particle("", "text", ...)` here.
                // An empty-selector particle targets the segment's own root
                // element, and blank.rs's empty-key `set_value` detaches *all*
                // children before inserting its placeholder — placed last, it
                // would silently wipe out every placeholder already produced by
                // the breadcrumbs/images/links/headings above, collapsing the
                // whole lifted `product` section to a single `_text_` node.
            ],
        ),
    ])
}
