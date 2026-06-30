//! Blanks a parsed page against a [`RetailerArchitecture`].
//!
//! This is the `valueless` counterpart to [`super::extract`]: it walks the same
//! definition tree, but instead of reading each targeted value it overwrites it
//! with a placeholder. A named particle's value uses `_name_` (its own name); a
//! nameless particle names each value `%name%` after its attribute. The mutated
//! DOM is then serialized back to HTML.
//!
//! Like extraction, selectors match relative to the parent's matched element,
//! a particle targets its first match, and a collection targets every match. An
//! empty selector targets the context element itself. A named particle names its
//! placeholders after itself; a nameless one names them after each attribute.
//!
//! Collections are "componentized": each matched item is blanked, lifted out of
//! the page (its place taken by a `[name_index]` text placeholder), and returned
//! as a [`Component`] so it can be written to its own file. A JSON structure
//! replaces each matched script's content with a `!name!` placeholder.

use kuchiki::{ElementData, NodeDataRef, NodeRef};

use super::structure::{RetailerArchitecture, Structure};

/// A collection item lifted out of the page during blanking: its collection
/// `name`, its `index` within that collection, and the detached (blanked)
/// element node. The page keeps a `[name_index]` placeholder in its place.
pub struct Component {
    pub name: String,
    pub index: usize,
    pub node: NodeRef,
}

/// Blanks `node` against `architecture` and returns the lifted collection
/// [`Component`]s (nested ones first, in document order).
pub fn apply(node: &NodeRef, architecture: &RetailerArchitecture) -> Vec<Component> {
    let mut components = Vec::new();
    blank_all(node, &architecture.structures, &mut components);
    components
}

/// Blanks each structure relative to `context`.
fn blank_all(context: &NodeRef, structures: &[Structure], components: &mut Vec<Component>) {
    for structure in structures {
        blank_one(context, structure, components);
    }
}

/// Blanks a single structure relative to `context`.
fn blank_one(context: &NodeRef, structure: &Structure, components: &mut Vec<Component>) {
    match structure {
        Structure::Particle(particle) => {
            let Some(matched) = select_one(context, &particle.selector) else {
                return;
            };
            for attr in &particle.attrs {
                let replacement = if !attr.key.is_empty() && attr.name.is_empty() {
                    // Keyed but unnamed: a placeholder-less blank.
                    "::".to_string()
                } else if particle.name.is_empty() {
                    // Nameless particle: each value is named after its attribute.
                    format!("%{}%", attr.name)
                } else {
                    // Named particle: its own value.
                    format!("_{}_", particle.name)
                };
                set_value(&matched, &attr.key, &replacement);
            }
        }
        Structure::Segment(segment) => {
            if let Some(matched) = select_one(context, &segment.selector) {
                blank_all(matched.as_node(), &segment.subs, components);
            }
        }
        Structure::Collection(collection) => {
            let Ok(matches) = context.select(&collection.selector) else {
                return;
            };
            for (index, matched) in matches.collect::<Vec<_>>().into_iter().enumerate() {
                let node = matched.as_node().clone();
                // Blank the item's contents (lifting any nested collections), then
                // replace the item with a `[name_index]` placeholder and lift it.
                blank_all(&node, &collection.subs, components);
                node.insert_before(NodeRef::new_text(format!("[{}_{index}]", collection.name)));
                node.detach();
                components.push(Component {
                    name: collection.name.clone(),
                    index,
                    node,
                });
            }
        }
        Structure::Trash(trash) => {
            if let Ok(matches) = context.select(&trash.selector) {
                for matched in matches.collect::<Vec<_>>() {
                    matched.as_node().detach();
                }
            }
        }
        Structure::Scrub(scrub) => {
            // Blank the attribute to `::` on every match (only if present).
            if let Ok(matches) = context.select(&scrub.selector) {
                for matched in matches.collect::<Vec<_>>() {
                    if let Some(value) = matched.attributes.borrow_mut().get_mut(scrub.attr.as_str()) {
                        *value = "::".to_string();
                    }
                }
            }
        }
        Structure::Json(json) => {
            // JSON can't be templated field-by-field; replace each matched
            // script's whole content with a `!name!` placeholder. With an anchor,
            // only the script(s) actually containing it are blanked.
            if let Ok(matches) = context.select(&json.selector) {
                for matched in matches.collect::<Vec<_>>() {
                    if json.anchor.is_empty() || matched.as_node().text_contents().contains(&json.anchor) {
                        set_value(&matched, "", &format!("!{}!", json.name));
                    }
                }
            }
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

/// Overwrites `key` on `matched` with `replacement`: its text content for an
/// empty key, otherwise the named HTML attribute (only if already present, so
/// blanking targets the same values extraction would read).
fn set_value(matched: &NodeDataRef<ElementData>, key: &str, replacement: &str) {
    if key.is_empty() {
        let node = matched.as_node();
        for child in node.children().collect::<Vec<_>>() {
            child.detach();
        }
        node.append(NodeRef::new_text(replacement));
    } else if let Some(value) = matched.attributes.borrow_mut().get_mut(key) {
        *value = replacement.to_string();
    }
}

#[cfg(test)]
mod tests {
    use kuchiki::traits::*;

    use super::apply;
    use crate::html_parser::structure::{RetailerArchitecture, collection, json, particle, segment};

    fn blanked_html(architecture: &RetailerArchitecture, html: &str) -> String {
        let document = kuchiki::parse_html().one(html);
        let _ = apply(&document, architecture);
        serialize(&document)
    }

    /// Blanks `html` and returns the page plus each lifted component's HTML.
    fn blank_components(architecture: &RetailerArchitecture, html: &str) -> (String, Vec<String>) {
        let document = kuchiki::parse_html().one(html);
        let components = apply(&document, architecture);
        let page = serialize(&document);
        let component_html = components.iter().map(|c| serialize(&c.node)).collect();
        (page, component_html)
    }

    fn serialize(node: &kuchiki::NodeRef) -> String {
        let mut out = Vec::new();
        node.serialize(&mut out).expect("serializes");
        String::from_utf8(out).expect("utf8")
    }

    #[test]
    fn replaces_attribute_values_with_placeholders() {
        let architecture = RetailerArchitecture::new(vec![segment(
            "head",
            "product",
            vec![particle(
                r#"meta[property="og:title"]"#,
                "name",
                vec![("content", "value")],
            )],
        )]);
        let html = r#"<html><head><meta property="og:title" content="Mouse Pad"></head><body></body></html>"#;

        let blanked = blanked_html(&architecture, html);

        assert!(blanked.contains(r#"content="_name_""#), "got: {blanked}");
        assert!(!blanked.contains("Mouse Pad"));
    }

    #[test]
    fn keyed_but_unnamed_attribute_becomes_double_colon() {
        // The `content` attr is keyed but unnamed in the definition.
        let architecture = RetailerArchitecture::new(vec![segment(
            "head",
            "product",
            vec![particle(
                r#"meta[property="og:title"]"#,
                "name",
                vec![("property", "prop"), ("content", "")],
            )],
        )]);
        let html = r#"<html><head><meta property="og:title" content="Mouse Pad"></head><body></body></html>"#;

        let blanked = blanked_html(&architecture, html);

        // content is blanked to `::`; the named `property` still gets a placeholder.
        assert!(blanked.contains(r#"content="::""#), "got: {blanked}");
        assert!(blanked.contains(r#"property="_name_""#), "got: {blanked}");
        assert!(!blanked.contains("Mouse Pad"));
    }

    #[test]
    fn replaces_text_content_with_placeholders() {
        let architecture = RetailerArchitecture::new(vec![particle("h1", "heading", vec![("", "value")])]);
        let html = r#"<html><body><h1>Hello</h1></body></html>"#;

        let blanked = blanked_html(&architecture, html);

        assert!(blanked.contains("<h1>_heading_</h1>"), "got: {blanked}");
    }

    #[test]
    fn componentizes_each_collection_item_with_placeholders() {
        let architecture = RetailerArchitecture::new(vec![collection(
            "meta",
            "metas",
            vec![particle("", "", vec![("content", "value")])],
        )]);
        let html = r#"<html><head>
            <meta property="og:title" content="Mouse Pad">
            <meta property="og:url" content="/p/1"></head><body></body></html>"#;

        let (page, components) = blank_components(&architecture, html);

        // Each item is replaced by a numbered placeholder and lifted out.
        assert!(page.contains("[metas_0]") && page.contains("[metas_1]"), "got: {page}");
        assert!(!page.contains("Mouse Pad") && !page.contains("/p/1"));
        assert_eq!(components.len(), 2);
        assert_eq!(
            components.iter().filter(|c| c.contains(r#"content="%value%""#)).count(),
            2
        );
    }

    #[test]
    fn json_content_becomes_a_name_placeholder() {
        let architecture = RetailerArchitecture::new(vec![json(
            r#"script[type="application/ld+json"]"#,
            "schema",
            vec![("sku", "sku")],
        )]);
        let html =
            r#"<html><head><script type="application/ld+json">{"sku":"MS01"}</script></head><body></body></html>"#;

        let blanked = blanked_html(&architecture, html);

        assert!(
            blanked.contains("<script type=\"application/ld+json\">!schema!</script>"),
            "got: {blanked}"
        );
        assert!(!blanked.contains("MS01"));
    }

    #[test]
    fn lifts_collection_item_content_into_components() {
        let architecture = RetailerArchitecture::new(vec![collection(
            "li.review",
            "reviews",
            vec![particle("span", "body", vec![("", "value")])],
        )]);
        let html = r#"<html><body><ul>
            <li class="review"><span>Great</span></li>
            <li class="review"><span>Fast</span></li></ul></body></html>"#;

        let (page, components) = blank_components(&architecture, html);

        assert!(
            page.contains("[reviews_0]") && page.contains("[reviews_1]"),
            "got: {page}"
        );
        assert!(!page.contains("Great") && !page.contains("Fast"));
        assert_eq!(components.len(), 2);
        assert!(
            components.iter().all(|c| c.contains("<span>_body_</span>")),
            "got: {components:?}"
        );
    }
}
