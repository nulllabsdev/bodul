# Product Processing — Technical Specification

| Field   | Value                                                           |
| ------- | --------------------------------------------------------------- |
| Status  | Draft                                                           |
| Version | 0.1.0                                                           |
| Scope   | `apps/mvp` module `offer_processing` (all 10 MinisForum stores) |
| Roadmap | Stage E                                                         |

Maps a destructured MinisForum AU product page (the output of the `destructure`
binary, see `TS001_html-parser.md`) into a typed, normalized **processed
product**. Driven by the `process_products` binary.

Layout: `src/offer_processing/minisforum_au/` is a folder module with
`destructured.rs` (a strict typed mirror of the destructure JSON) and
`processed.rs` (our own shape). All types are prefixed `MinisForumAu`.

## Scope

- **AU-faithful** model only. `process_products` deserializes `MinisForumAu`
  files into the typed model; the other nine stores stay on untyped
  `serde_json::Value` passthrough until they get their own models.
- `MinisForumAuDestructuredProduct` is strict: every key modelled,
  `#[serde(deny_unknown_fields)]` on every struct, every scalar leaf is a
  `String`, `Option<String>` for not-always-present fields, `#[serde(default)]
  Vec` for absent lists. Absent optionals serialize as `null`/`[]` (kept, not
  skipped).

## Type conventions (processed)

| Concept      | Type                                | Notes                                                                                                                                                                                                                                                                |
| ------------ | ----------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| id           | `MinisForumAuProductId(u64)`        | `#[serde(transparent)]` → JSON number; `from_string` errors if non-numeric.                                                                                                                                                                                          |
| price        | `money::Money` (`lib/money`)        | minor units (cents), currency `AUD`. Custom serde (`money_wire`/`option_money_wire`) → canonical object `{"amount_minor":"<i64>","currency":"AUD"}` (amount as string for exact round-trip). Minimal `lib/money` bodies implemented: `new`/`minor_units`/`currency`. |
| availability | `MinisForumAuProcessedAvailability` | `{Available,Unavailable}`; serde `into/from bool` → JSON `true`/`false`. `from_string` matches `"true"`/`"false"` exactly, errors otherwise.                                                                                                                         |
| product type | `MinisForumAuProductType`           | serialized as the exact source strings via per-variant `rename` (incl. source typo `"Accesorries"`); strict `from_string` errors on unseen values.                                                                                                                   |
| locale       | `MinisForumAuLocale`                | `En` only (AU is English-only); errors on anything but `"en"`. Read from `<html lang>`.                                                                                                                                                                              |
| date         | `chrono::NaiveDate`                 | `price_valid_until`; round-trips to the same ISO string.                                                                                                                                                                                                             |

Price unit handling: product/variant/meta prices are **cents** strings
(`parse_cents`); offer price is a **dollars** string (`dollars_to_cents`, 1–2
fractional digits, errors on >2, no rounding). Every constructor is fallible;
conversions are `TryFrom` so a bad value fails that one file rather than
panicking.

## Mapped fields

Processed root keys: `locale`, `product`, `images`, `features`, `variants`.

- `product` (`MinisForumAuProcessedProductInfo`): `id`, `handle`, `title`,
  `vendor`, `type` (Option), `price`, `compare_at_price` (Option),
  `availability`, `variants`.
- `images: Vec<String>` — from the `xxxx` gallery, **`src` only** (alt dropped).
- `features: Vec<{label, value: Vec<String>}>` — from `feature_chart`,
  column-major chart **flattened**. `value` = raw cell text split on `\n`,
  trimmed, blanks dropped. `collapse_after_colon` collapses whitespace after a
  colon (ASCII `:` or full-width `：`) down to one space, so
  `"Processor：\n  Core"` → `"Processor： Core"`.
- `variants: Vec<ZzzVariant>` — the combined variant (below).

## Combined variants (`make_variants` / `ZzzVariant`)

`make_variants` iterates the product-object variants and, **matching by SKU**,
finds the matching `meta` variant and JSON-LD offer, then combines the three.

Fields **guaranteed identical across sources are lifted to single top-level
fields and guarded** (mismatch = hard error):

- `sku` — equal across product/meta/offer.
- `price` — equal (product cents == meta cents == offer dollars→cents).
- `availability` — equal (product `"true"/"false"` == offer schema.org).
- `title` — = offer name (always present); the meta title must match **when
  present and non-empty**.

Single-source fields, de-prefixed: `compare_at_price` (product),
`price_valid_until` (offer), `option1/2/3` (product). Still prefixed:
`meta_variant_id` (meta's analytics variant id, distinct from SKU).

## Guards & expected failures

Hard errors (fail that file): variant source **length mismatch**, **sku**,
**price**, **availability**, and **title** mismatches.

**Expected baseline: `done: 799 processed, 3 failed`.** The three failures —
`products-product-protection`, `products-shipping-protection`,
`products-price-difference-2` — are insurance / price-adjustment SKUs where
`product.price` disagrees with `meta`/`offer` (which agree with each other).
**Decision: keep failing** — price disagreement is a real error, not a
regression.

## Intentionally not mapped (with reasons)

- `product.price_min` / `price_max` — not interested in the variant price range.
- `product.media` — images come from the gallery (`images`), not `product.media`.
- `tt_product` — redundant (`id`=`product.id`, `title`=`product.title`,
  `image_url` is just a gallery image).
- `pixels` — shop/page info + a related-products list; not needed. The
  `pixels.products` are *related* products (cross-sell), not this product — we
  are not interested in product relations at the moment.
- `viewed_product` — values are duplicates/derivable.
- `meta` (block) — only `gid`/`page_type` were non-derivable. `meta.variants`
  is still a **source** for `variants[]`.
- `schema` (block) — the first schema's `offers` are still a **source** for
  `variants[]`; breadcrumb (name/url), the "Home" crumb, and any 3rd schema are
  not used.
- `xxxx`: `title`/`price`/`variants` — duplicate data; only the gallery is used.

## Deferred (no decision yet)

- `describe_box` (whole block: `text` + `links[].href`, ~4/32 pages).
- `xxxx.badge` (promo flag "HOT"/"NEW").
- `feature_chart.h1` / `h2` (chart headings).
- Feature **label normalization** — 69 distinct labels / 319 rows, with many
  case/spacing/synonym variants and a source bug `"Pre-installed OSOS Support"`.

## All 10 retailers (implemented)

Every store now has its own `offer_processing/minisforum_{xx}/` module (strict
`destructured.rs` + adapted `processed.rs`, self-contained duplicated helpers),
wired into `process_products` via a per-retailer dispatch. `lib/money` gained
`GBP`, `JPY`, `KRW`, `HKD` to cover all stores.

Per-store divergences discovered during implementation (all best-effort, per the
plan):

- **Product source**: AU = `const product`; Ca/Us = both `product` + `xcotton`
  (use `product`); Eu/Uk/Fr = `xcotton_pp_variants` only; Jp/Kr/Ru/Hk = none →
  product core derived from `meta` (+ `schemas`).
- **No `xxxx`** (Us/Kr/Ru/Hk) → `images`/`features` omitted or empty.
- **Degenerate JSON-LD offers** on Eu/Us/Fr (no per-variant `name`/`sku`, count
  ≠ variant count) → variant combine is a 2-source product+meta join there;
  offers surfaced separately or dropped. Jp/Kr/Ru/Hk join meta+offer (2-source).
- **Multi-locale**: Us (en+es), Eu (de+en), Fr (en+fr) — their `Locale` enums
  carry multiple variants. Us currency is USD **and** CAD (read per page).
- **availability URL scheme**: most non-AU stores use `http://schema.org/...`
  (AU uses `https://`).

Current baseline (full run): **`done: 774 processed, 28 failed`**. All 28
failures are guard rejections — 26 cross-source variant price mismatches (mostly
"refurbished" pages where the product/xcotton price disagrees with `meta`) and 2
"missing product schema with an offer". These are expected (keep-failing), not
bugs. 71 lib tests pass (each store's strict deserialize test over all its
files).

## Applying to a further retailer

This AU model is the template; each store becomes a sibling module (e.g.
`offer_processing/minisforum_ca/`). What varies per retailer (per the
html_parser architecture):

- **currency** and **locale** — make per-retailer rather than hardcoding
  `AUD`/`En`.
- **which blocks exist** — `tt_product` (AU/CA/UK/HK); `xcotton_pp_variants` as
  `<script id>` (CA/EU/US) vs `var __xcotton_pp_variants__` (UK/FR) vs none
  (AU/KR/JP/RU/HK); `const product` (AU/CA/US); main-product DOM `xxxx` (absent
  on US/KR/RU); `feature_chart` (varies). The variant-combine sources may differ
  per store.
- **product type enum values** and **feature labels** are store-specific —
  re-survey before reusing AU's.
- Decide whether shared logic (Money/availability/date helpers, `make_variants`,
  guards) is factored into a shared module vs copied per retailer.

## Operational notes

- `process_products` does **not** clear `data/pages-processed/{Retailer}/` on a
  run, so a file that now fails leaves its stale prior output behind (currently
  product-protection, shipping-protection). Analyses over `pages-processed` can
  be polluted by these. Consider clearing the output dir at the start of a run.

## Verification

- `cargo build` + `cargo test --lib`.
- `cargo run --bin process_products` → expect `done: 799 processed, 3 failed`
  (the three known price-mismatch products).
