# HTML Parser — Technical Specification

| Field   | Value                           |
| ------- | ------------------------------- |
| Status  | Draft                           |
| Version | 0.1.0                           |
| Scope   | `apps/mvp` module `html_parser` |
| Roadmap | Stage E                         |

Extracts structured product and offer data from stored product-page HTML.

## Structures

A retailer's pages are described as a tree of **structures**. Each structure
carries a CSS `selector` locating it on the page and a `name` for what it
yields. There are four kinds:

- **Particle** — a leaf; pulls one or more named values (`attrs`) from a matched
  element (e.g. name, price, sku).
- **Collection** — a keyed group; its selector matches many elements, each keyed
  by the value of its `key` attribute and holding the value of its `value`
  attribute (e.g. every `<meta>` keyed by `property`).
- **Segment** — a named block; its selector scopes the nested structures.
- **Trash** — removes the elements its selector matches from the working tree and
  yields nothing. Placed before another structure, it strips unwanted nodes (e.g.
  an sr-only label `<span>`) so the later structure reads clean text.
- **Json** — parses a matched element's JSON content (e.g. a JSON-LD `<script>`)
  and pulls values at dotted `paths` like `offers.price`. Extract-only: it has no
  effect during `valueless`.

Segments hold child structures, so the tree is composite and nests to any depth.

## Model

The three kinds are unified under a `Structure` enum:

```rust
enum Structure {
    Segment(Segment),       // { selector, name, subs:  Vec<Structure> }
    Collection(Collection), // { selector, name, subs:  Vec<Structure> }
    Particle(Particle),     // { selector, name, attrs: Vec<Attribute> }
    Trash(Trash),           // { selector }
    Json(Json),             // { selector, name, paths: Vec<Attribute> }
}
```

An `Attribute` pairs a `key` (the HTML attribute to read, or the reserved
`"text"` key for the element's text content) with the `name` the extracted value
is given. A retailer's whole page model is a `RetailerArchitecture { structures:
Vec<Structure> }`. These types live in `src/html_parser/structure.rs`.

Structures are built with the constructor helpers:

```rust
particle(selector, name, attrs)        // attrs: Vec<(key, name)>
collection(selector, name, subs)
segment(selector, name, subs)
trash(selector)
json(selector, name, paths)            // paths: Vec<(json_path, name)>; array → first element
```

```rust
RetailerArchitecture::new(vec![
    collection(r#"meta[property^="og:"]"#, "header", "property", "content"),
])
```

## Extraction

The same architecture drives different consumers (the `destructure` and
`valueless` binaries). `architecture_for(retailer)` returns it; retailers
without an architecture yet return an empty one. MinisForum AU pages are
Shopify, so its architecture (`src/html_parser/minisforum_au.rs`) targets the
stable Open Graph / `product:` meta tags in `<head>`.

`destructure(html, retailer)` parses the page with `kuchiki` and walks the
architecture, matching each selector **relative to its parent's matched
element**, to produce a `serde_json::Value`:

- a **segment** becomes a nested object,
- a **collection** becomes an object keyed by each item's `key` value,
- a **particle** becomes its named value — a string for a single attribute, or an
  object of `name → value` for several.

Structures that match nothing are omitted. For example:

```json
{
  "header": { "og:title": "Minisforum Mouse Pad", "og:url": "..." },
  "offer":  { "product:price:amount": "15.99", "product:price:currency": "AUD" }
}
```

### Valueless

`valueless(html, retailer)` walks the same architecture but, instead of reading
each targeted value, overwrites it with a `{{name}}` placeholder and serializes
the mutated DOM back to HTML. A particle uses its own name; a collection names
each placeholder after that item's `key` value. The HTML attribute or text
content is replaced in place, and only elements the architecture targets are
touched. For example, `<meta property="og:title" content="Mouse Pad">` becomes
`<meta property="og:title" content="{{og:title}}">`.
