//! Applies a [`RetailerArchitecture`] to a parsed page.
//!
//! Walking the definition tree yields a `serde_json::Value`:
//! - a [`Segment`](super::structure::Segment) becomes a nested object,
//! - a [`Collection`](super::structure::Collection) becomes an array of objects,
//! - a [`Particle`](super::structure::Particle) becomes its named value — a
//!   string for a single attribute, or an object of `name → value` for several.
//!
//! Selectors are matched relative to the current context node, so nested
//! structures search within their parent's matched element. An empty selector
//! targets the context element itself, and a structure with an empty name merges
//! its values into the parent rather than nesting under a key.

use kuchiki::{ElementData, NodeDataRef, NodeRef};
use serde_json::{Map, Value};

use ::retailer_sourcing::parsing::structure::{Attribute, RetailerArchitecture, Structure};

/// Extracts `architecture` from the parsed `node` into a JSON object.
pub fn extract(node: &NodeRef, architecture: &RetailerArchitecture) -> Value {
    Value::Object(extract_all(node, &architecture.structures))
}

/// Extracts each structure relative to `context`, merging the resulting entries
/// into one object. Structures that match nothing contribute nothing.
fn extract_all(context: &NodeRef, structures: &[Structure]) -> Map<String, Value> {
    let mut object = Map::new();
    for structure in structures {
        for (name, value) in extract_one(context, structure) {
            object.insert(name, value);
        }
    }
    object
}

/// The entries a single structure contributes to its parent object. A named
/// structure contributes one `(name, value)` entry; a nameless one contributes
/// its values directly (merged into the parent). Empty when nothing matched.
fn extract_one(context: &NodeRef, structure: &Structure) -> Vec<(String, Value)> {
    match structure {
        Structure::Particle(particle) => {
            let Some(matched) = select_one(context, &particle.selector) else {
                return Vec::new();
            };
            let values = read_attrs(&matched, &particle.attrs);
            if values.is_empty() {
                return Vec::new();
            }
            // A nameless particle merges its values into the parent.
            if particle.name.is_empty() {
                return values
                    .into_iter()
                    .map(|(name, value)| (name, Value::String(value)))
                    .collect();
            }
            let value = match values.len() {
                1 => Value::String(values.into_iter().next().unwrap().1),
                _ => Value::Object(
                    values
                        .into_iter()
                        .map(|(name, value)| (name, Value::String(value)))
                        .collect(),
                ),
            };
            vec![(particle.name.clone(), value)]
        }
        Structure::Segment(segment) => {
            let Some(matched) = select_one(context, &segment.selector) else {
                return Vec::new();
            };
            let inner = extract_all(matched.as_node(), &segment.subs);
            if inner.is_empty() {
                return Vec::new();
            }
            // A nameless segment merges its contents into the parent.
            if segment.name.is_empty() {
                return inner.into_iter().collect();
            }
            vec![(segment.name.clone(), Value::Object(inner))]
        }
        Structure::Collection(collection) => {
            let Ok(matches) = context.select(&collection.selector) else {
                return Vec::new();
            };
            let items: Vec<Value> = matches
                .map(|matched| extract_all(matched.as_node(), &collection.subs))
                .filter(|item| !item.is_empty())
                .map(Value::Object)
                .collect();
            if items.is_empty() {
                return Vec::new();
            }
            vec![(collection.name.clone(), Value::Array(items))]
        }
        Structure::Trash(trash) => {
            detach_all(context, &trash.selector);
            Vec::new()
        }
        // Scrub blanks an attribute for `valueless`; extraction is unaffected.
        Structure::Scrub(_) => Vec::new(),
        Structure::Comments(_) => {
            detach_comments(context);
            Vec::new()
        }
        Structure::Json(json) => {
            let Ok(matches) = context.select(&json.selector) else {
                return Vec::new();
            };
            // Whole-text JSON (JSON-LD) or, with an anchor, the JSON value sliced
            // out of surrounding JavaScript.
            let blocks: Vec<Value> = if json.anchor.is_empty() {
                matches
                    .filter_map(|matched| serde_json::from_str(&matched.as_node().text_contents()).ok())
                    .collect()
            } else {
                matches
                    .filter_map(|matched| json_after(&matched.as_node().text_contents(), &json.anchor))
                    .collect()
            };

            // An array root yields one object per element; an object root yields
            // a single object (scalars plus `[]` sub-lists).
            if matches!(blocks.first(), Some(Value::Array(_))) {
                let items: Vec<Value> = blocks
                    .iter()
                    .filter_map(|block| block.as_array())
                    .flatten()
                    .map(|element| Value::Object(json_object(std::slice::from_ref(element), &json.paths)))
                    .filter(|item| item.as_object().is_some_and(|fields| !fields.is_empty()))
                    .collect();
                if items.is_empty() {
                    return Vec::new();
                }
                return vec![(json.name.clone(), Value::Array(items))];
            }

            let object = json_object(&blocks, &json.paths);
            if object.is_empty() {
                return Vec::new();
            }
            // A nameless json block merges its values into the parent.
            if json.name.is_empty() {
                return object.into_iter().collect();
            }
            vec![(json.name.clone(), Value::Object(object))]
        }
    }
}

/// Builds an object from `blocks` and `paths`: scalar paths resolve directly
/// (first block that contains them); a path with `[]` (e.g. `variants[].sku`)
/// groups under its array key as a list of objects.
fn json_object(blocks: &[Value], paths: &[Attribute]) -> Map<String, Value> {
    let mut object = Map::new();
    let mut lists: Vec<(String, Vec<(String, String)>)> = Vec::new();
    for path in paths {
        if let Some((array_path, rest)) = path.key.split_once("[]") {
            let sub_path = rest.strip_prefix('.').unwrap_or(rest).to_string();
            match lists.iter_mut().find(|(p, _)| p == array_path) {
                Some((_, subs)) => subs.push((path.name.clone(), sub_path)),
                None => lists.push((array_path.to_string(), vec![(path.name.clone(), sub_path)])),
            }
        } else if let Some(value) = blocks.iter().find_map(|block| resolve(block, &path.key)) {
            object.insert(path.name.clone(), value);
        }
    }

    for (array_path, subs) in lists {
        let Some(elements) = blocks.iter().find_map(|block| resolve_array(block, &array_path)) else {
            continue;
        };
        let items: Vec<Value> = elements
            .iter()
            .map(|element| {
                let fields: Map<String, Value> = subs
                    .iter()
                    .filter_map(|(name, sub_path)| resolve(element, sub_path).map(|value| (name.clone(), value)))
                    .collect();
                Value::Object(fields)
            })
            .filter(|item| item.as_object().is_some_and(|fields| !fields.is_empty()))
            .collect();
        if !items.is_empty() {
            let key = array_path.rsplit('.').next().unwrap_or(&array_path);
            object.insert(key.to_string(), Value::Array(items));
        }
    }

    object
}

/// Slices the JSON value that follows `anchor` (e.g. a JS variable name) out of
/// surrounding JavaScript and parses it. The first `[` or `{` after the anchor
/// starts the value; a streaming parser stops at its end, ignoring any trailing
/// `;`/code. `None` if the anchor is absent or the value doesn't parse.
fn json_after(text: &str, anchor: &str) -> Option<Value> {
    let after = &text[text.find(anchor)? + anchor.len()..];
    let start = after.find(['[', '{'])?;
    serde_json::Deserializer::from_str(&after[start..])
        .into_iter::<Value>()
        .next()?
        .ok()
}

/// Resolves a dotted `path` to an array of element values: descends like
/// [`resolve`] but the leaf must be an array (returned as-is) or a single object
/// (normalized to a one-element list). `None` otherwise.
fn resolve_array<'a>(value: &'a Value, path: &str) -> Option<Vec<&'a Value>> {
    let mut current = value;
    for key in path.split('.') {
        while let Value::Array(items) = current {
            current = items.first()?;
        }
        current = current.get(key)?;
    }
    match current {
        Value::Array(items) => Some(items.iter().collect()),
        object @ Value::Object(_) => Some(vec![object]),
        _ => None,
    }
}

/// Resolves a dotted `path` against `value`, descending into the first element
/// of any array encountered. Returns the leaf as a string (numbers/bools via
/// their JSON form); `None` if the path is absent or the leaf is not a scalar.
fn resolve(value: &Value, path: &str) -> Option<Value> {
    let mut current = value;
    for key in path.split('.') {
        while let Value::Array(items) = current {
            current = items.first()?;
        }
        current = current.get(key)?;
    }
    while let Value::Array(items) = current {
        current = items.first()?;
    }
    match current {
        Value::String(text) => (!text.is_empty()).then(|| Value::String(text.clone())),
        Value::Number(number) => Some(Value::String(number.to_string())),
        Value::Bool(flag) => Some(Value::String(flag.to_string())),
        _ => None,
    }
}

/// Detaches every HTML comment node in `context` (itself and its descendants).
fn detach_comments(context: &NodeRef) {
    for node in context.inclusive_descendants().collect::<Vec<_>>() {
        if node.as_comment().is_some() {
            node.detach();
        }
    }
}

/// Detaches every element matched by `selector` relative to `context`.
fn detach_all(context: &NodeRef, selector: &str) {
    if let Ok(matches) = context.select(selector) {
        for matched in matches.collect::<Vec<_>>() {
            matched.as_node().detach();
        }
    }
}

/// The element matched by `selector` relative to `context`, or the context
/// element itself when `selector` is empty.
fn select_one(context: &NodeRef, selector: &str) -> Option<NodeDataRef<ElementData>> {
    if selector.is_empty() {
        context.clone().into_element_ref()
    } else {
        context.select_first(selector).ok()
    }
}

/// Reads each attribute from `matched`, dropping absent or empty values. An
/// attribute keyed but unnamed is a valueless-only blank, so it is skipped here.
fn read_attrs(matched: &kuchiki::NodeDataRef<kuchiki::ElementData>, attrs: &[Attribute]) -> Vec<(String, String)> {
    attrs
        .iter()
        .filter(|attr| attr.key.is_empty() || !attr.name.is_empty())
        .filter_map(|attr| read_value(matched, &attr.key).map(|value| (attr.name.clone(), value)))
        .collect()
}

/// Reads `key` from `matched`: its text content for an empty key, otherwise the
/// named HTML attribute. Empty values are discarded.
fn read_value(matched: &kuchiki::NodeDataRef<kuchiki::ElementData>, key: &str) -> Option<String> {
    let value = if key.is_empty() {
        matched.as_node().text_contents().trim().to_string()
    } else {
        matched.attributes.borrow().get(key)?.trim().to_string()
    };

    (!value.is_empty()).then_some(value)
}

#[cfg(test)]
mod tests {
    use kuchiki::traits::*;
    use serde_json::json;

    use super::extract;
    use ::retailer_sourcing::parsing::structure::{
        RetailerArchitecture, collection, comments, json, json_after, particle, segment, trash,
    };

    #[test]
    fn extracts_segments_and_particles() {
        let architecture = RetailerArchitecture::new(vec![segment(
            "head",
            "product",
            vec![
                particle(r#"meta[property="og:title"]"#, "name", vec![("content", "value")]),
                particle(
                    r#"meta[property="product:price:amount"]"#,
                    "price",
                    vec![("content", "value")],
                ),
            ],
        )]);
        let html = r#"<html><head>
            <meta property="og:title" content="Mouse Pad">
            <meta property="product:price:amount" content="15.99"></head><body></body></html>"#;
        let node = kuchiki::parse_html().one(html);

        let value = extract(&node, &architecture);

        assert_eq!(value, json!({ "product": { "name": "Mouse Pad", "price": "15.99" } }));
    }

    #[test]
    fn emits_an_object_for_multi_attribute_particles() {
        let architecture =
            RetailerArchitecture::new(vec![particle("a.more", "link", vec![("href", "url"), ("", "label")])]);
        let html = r#"<html><body><a class="more" href="/p/1">Details</a></body></html>"#;
        let node = kuchiki::parse_html().one(html);

        let value = extract(&node, &architecture);

        assert_eq!(value, json!({ "link": { "url": "/p/1", "label": "Details" } }));
    }

    #[test]
    fn collects_repeated_matches_into_an_array() {
        let architecture = RetailerArchitecture::new(vec![collection(
            "li.review",
            "reviews",
            vec![particle("span", "body", vec![("", "value")])],
        )]);
        let html = r#"<html><body><ul>
            <li class="review"><span>Great</span></li>
            <li class="review"><span>Fast</span></li></ul></body></html>"#;
        let node = kuchiki::parse_html().one(html);

        let value = extract(&node, &architecture);

        assert_eq!(value, json!({ "reviews": [{ "body": "Great" }, { "body": "Fast" }] }));
    }

    #[test]
    fn nameless_self_particle_merges_attrs_into_each_item() {
        // Empty selector targets each matched <meta> itself; the nameless
        // particle merges its `property`/`content` into the item object.
        let architecture = RetailerArchitecture::new(vec![collection(
            "meta",
            "metas",
            vec![particle("", "", vec![("property", "name"), ("content", "value")])],
        )]);
        let html = r#"<html><head>
            <meta property="og:title" content="Mouse Pad">
            <meta property="og:url" content="/p/1"></head><body></body></html>"#;
        let node = kuchiki::parse_html().one(html);

        let value = extract(&node, &architecture);

        assert_eq!(
            value,
            json!({ "metas": [
                { "name": "og:title", "value": "Mouse Pad" },
                { "name": "og:url", "value": "/p/1" },
            ] })
        );
    }

    #[test]
    fn trash_strips_elements_before_later_structures_read_text() {
        // The label span is trashed first, so the price particle reads just the
        // amount instead of "Sale priceAU$959.00".
        let architecture = RetailerArchitecture::new(vec![segment(
            ".product-info",
            "product",
            vec![
                trash("sale-price .sr-only"),
                particle("sale-price", "sale_price", vec![("", "value")]),
            ],
        )]);
        let html = r#"<html><body><div class="product-info">
            <sale-price><span class="sr-only">Sale price</span>AU$959.00</sale-price>
            </div></body></html>"#;
        let node = kuchiki::parse_html().one(html);

        let value = extract(&node, &architecture);

        assert_eq!(value, json!({ "product": { "sale_price": "AU$959.00" } }));
    }

    #[test]
    fn json_pulls_dotted_paths_with_array_descent() {
        let architecture = RetailerArchitecture::new(vec![json(
            r#"script[type="application/ld+json"]"#,
            "schema",
            vec![
                ("sku", "sku"),
                ("brand.name", "brand"),
                ("offers.price", "price"),
                ("offers.availability", "availability"),
            ],
        )]);
        // Two blocks: a Product (offers is an array) and an unrelated block.
        let html = r#"<html><head>
            <script type="application/ld+json">{"@type":"BreadcrumbList","itemListElement":[]}</script>
            <script type="application/ld+json">{"@type":"Product","sku":"MS01","brand":{"name":"Minisforum"},
                "offers":[{"price":959.0,"availability":"https://schema.org/InStock"}]}</script>
            </head><body></body></html>"#;
        let node = kuchiki::parse_html().one(html);

        let value = extract(&node, &architecture);

        assert_eq!(
            value,
            json!({ "schema": {
                "sku": "MS01",
                "brand": "Minisforum",
                "price": "959.0",
                "availability": "https://schema.org/InStock",
            } })
        );
    }

    #[test]
    fn json_groups_array_paths_into_a_list_of_objects() {
        let architecture = RetailerArchitecture::new(vec![json(
            r#"script[type="application/ld+json"]"#,
            "schema",
            vec![
                ("sku", "sku"),
                ("offers[].price", "price"),
                ("offers[].availability", "stock"),
            ],
        )]);
        let html = r#"<html><head><script type="application/ld+json">{"sku":"P1",
            "offers":[{"price":439.0,"availability":"InStock"},{"price":639.9,"availability":"OutOfStock"}]}</script>
            </head><body></body></html>"#;
        let node = kuchiki::parse_html().one(html);

        let value = extract(&node, &architecture);

        assert_eq!(
            value,
            json!({ "schema": {
                "sku": "P1",
                "offers": [
                    { "price": "439.0", "stock": "InStock" },
                    { "price": "639.9", "stock": "OutOfStock" },
                ]
            } })
        );
    }

    #[test]
    fn json_after_slices_an_array_from_embedded_javascript() {
        let architecture = RetailerArchitecture::new(vec![json_after(
            "script",
            "productVariants =",
            "product_variants",
            vec![("sku", "sku"), ("available", "available"), ("price", "price")],
        )]);
        let html = r#"<html><body><script>(function(){var x=1;
            const productVariants = [{"sku":"A","available":true,"price":97500},
            {"sku":"B","available":false,"price":211900}];
            window.foo = productVariants;})();</script></body></html>"#;
        let node = kuchiki::parse_html().one(html);

        let value = extract(&node, &architecture);

        assert_eq!(
            value,
            json!({ "product_variants": [
                { "sku": "A", "available": "true", "price": "97500" },
                { "sku": "B", "available": "false", "price": "211900" },
            ] })
        );
    }

    #[test]
    fn comments_removes_comment_nodes_from_the_working_tree() {
        // The comment is detached first, so the heading particle reads clean text
        // rather than a tree still carrying comment noise.
        let architecture = RetailerArchitecture::new(vec![comments(), particle("h1", "heading", vec![("", "value")])]);
        let html = r#"<html><body><!-- tracking --><h1>Hello</h1><!-- more --></body></html>"#;
        let node = kuchiki::parse_html().one(html);

        let value = extract(&node, &architecture);

        assert_eq!(value, json!({ "heading": "Hello" }));
        // No comment nodes remain in the tree.
        assert_eq!(
            node.inclusive_descendants()
                .filter(|n| n.as_comment().is_some())
                .count(),
            0
        );
    }

    #[test]
    fn json_normalizes_a_single_object_to_a_one_element_list() {
        let architecture = RetailerArchitecture::new(vec![json(
            r#"script[type="application/ld+json"]"#,
            "schema",
            vec![("offers[].price", "price")],
        )]);
        let html = r#"<html><head><script type="application/ld+json">{"offers":{"price":99.0}}</script></head><body></body></html>"#;
        let node = kuchiki::parse_html().one(html);

        let value = extract(&node, &architecture);

        assert_eq!(value, json!({ "schema": { "offers": [{ "price": "99.0" }] } }));
    }

    #[test]
    fn json_omits_absent_paths_and_unparseable_blocks() {
        let architecture = RetailerArchitecture::new(vec![json(
            r#"script[type="application/ld+json"]"#,
            "schema",
            vec![("sku", "sku"), ("missing.path", "nope")],
        )]);
        let html = r#"<html><head>
            <script type="application/ld+json">not json</script>
            <script type="application/ld+json">{"sku":"MS01"}</script>
            </head><body></body></html>"#;
        let node = kuchiki::parse_html().one(html);

        let value = extract(&node, &architecture);

        assert_eq!(value, json!({ "schema": { "sku": "MS01" } }));
    }

    #[test]
    fn skips_keyed_but_unnamed_attributes() {
        // ("content", "") is a valueless-only blank; extraction skips it.
        let architecture = RetailerArchitecture::new(vec![particle(
            r#"meta[property="og:title"]"#,
            "",
            vec![("property", "prop"), ("content", "")],
        )]);
        let html = r#"<html><head><meta property="og:title" content="Mouse Pad"></head><body></body></html>"#;
        let node = kuchiki::parse_html().one(html);

        let value = extract(&node, &architecture);

        assert_eq!(value, json!({ "prop": "og:title" }));
    }

    #[test]
    fn omits_structures_that_match_nothing() {
        let architecture = RetailerArchitecture::new(vec![particle("title", "title", vec![("", "value")])]);
        let html = r#"<html><head></head><body></body></html>"#;
        let node = kuchiki::parse_html().one(html);

        let value = extract(&node, &architecture);

        assert_eq!(value, json!({}));
    }
}
