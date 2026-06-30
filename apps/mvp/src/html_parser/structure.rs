//! The parsing definition model.
//!
//! A retailer's pages are described as a tree of [`Structure`]s (roadmap Stage E,
//! spec `docs/TS001_html-parser.md`). Each structure carries a CSS `selector`
//! locating it on the page and a `name` for the value(s) it yields:
//!
//! - [`Particle`] — a leaf; pulls one or more named values (its `attrs`) from a
//!   matched element.
//! - [`Collection`] — a repeating group; its `selector` matches many elements,
//!   each producing an item from the nested `subs`.
//! - [`Segment`] — a named block; its `selector` scopes the nested `subs`.
//! - [`Trash`] — removes the elements its `selector` matches from the working
//!   tree, so later structures (e.g. a price particle) read clean text. It
//!   yields no output.
//! - [`Json`] — parses a matched `<script>` as JSON and pulls dotted `paths`
//!   (e.g. JSON-LD `offers.price`). Extract-only.
//!
//! Build them with the [`particle`], [`collection`], [`segment`], [`trash`] and
//! [`json`] helpers and collect them into a [`RetailerArchitecture`]. The walker
//! in [`super::extract`] applies the architecture to a parsed page.

/// One value to pull from a matched element.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Attribute {
    /// The HTML attribute to read, or an empty string for the element's text.
    pub key: String,
    /// The name the extracted value is given.
    pub name: String,
}

/// A node in a retailer's page definition.
///
/// Serializes with an internal `"kind"` tag (`segment` / `collection` /
/// `particle` / `trash`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Structure {
    /// A named block scoping nested structures.
    Segment(Segment),
    /// A repeating group of nested structures.
    Collection(Collection),
    /// A leaf yielding named value(s).
    Particle(Particle),
    /// Removes matched elements from the working tree; yields nothing.
    Trash(Trash),
    /// Blanks an attribute on every matched element to `::`; yields nothing.
    Scrub(Scrub),
    /// Parses a matched element's JSON content and pulls dotted paths.
    Json(Json),
}

/// A leaf: pulls named value(s) from the element matched by `selector`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Particle {
    /// The CSS selector locating the element.
    pub selector: String,
    /// What this leaf represents.
    pub name: String,
    /// The values to pull from the matched element.
    pub attrs: Vec<Attribute>,
}

/// A repeating group: `selector` matches many elements, each yielding an item.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Collection {
    /// The CSS selector matching each item element.
    pub selector: String,
    /// What this group represents.
    pub name: String,
    /// The structures extracted from each matched element.
    pub subs: Vec<Structure>,
}

/// A named block: `selector` scopes the nested structures.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Segment {
    /// The CSS selector locating the block.
    pub selector: String,
    /// What this block represents.
    pub name: String,
    /// The structures extracted within the block.
    pub subs: Vec<Structure>,
}

/// Removes the elements matched by `selector` from the working tree.
///
/// Placed before another structure, it strips unwanted nodes (e.g. an sr-only
/// label `<span>`) so the later structure reads clean text. Yields no output.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Trash {
    /// The CSS selector matching the elements to remove.
    pub selector: String,
}

/// Blanks the `attr` attribute to `::` on every element matched by `selector`,
/// in place (no extraction, no lifting). Useful for stripping unique values such
/// as template ids from `form`/`id` attributes across a section.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Scrub {
    /// The CSS selector matching the elements to scrub.
    pub selector: String,
    /// The attribute whose value is blanked (only if present).
    pub attr: String,
}

/// Parses the JSON content of the element(s) matched by `selector` and pulls
/// values at dotted `paths` (e.g. JSON-LD `offers.price`).
///
/// With an empty `anchor` the whole element text is parsed as JSON (JSON-LD).
/// With an `anchor` set (e.g. a JS variable name), the JSON value that follows
/// the anchor is sliced out of surrounding JavaScript and, if it is an array,
/// each element yields one object — `paths` then resolve per element.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Json {
    /// The CSS selector matching the script(s) to parse.
    pub selector: String,
    /// A substring after which the embedded JSON value begins; empty to parse the
    /// whole element text.
    pub anchor: String,
    /// What this block represents; empty merges the values into the parent.
    pub name: String,
    /// The values to pull: each `key` is a dotted JSON path, `name` the output.
    pub paths: Vec<Attribute>,
}

/// A [`Particle`] matched by `selector`, named `name`, pulling `attrs` as
/// `(key, name)` pairs.
pub fn particle(selector: &str, name: &str, attrs: Vec<(&str, &str)>) -> Structure {
    Structure::Particle(Particle {
        selector: selector.to_string(),
        name: name.to_string(),
        attrs: attrs
            .into_iter()
            .map(|(key, name)| Attribute {
                key: key.to_string(),
                name: name.to_string(),
            })
            .collect(),
    })
}

/// A [`Collection`] matched by `selector`, named `name`, with nested `subs`.
pub fn collection(selector: &str, name: &str, subs: Vec<Structure>) -> Structure {
    Structure::Collection(Collection {
        selector: selector.to_string(),
        name: name.to_string(),
        subs,
    })
}

/// A [`Segment`] matched by `selector`, named `name`, with nested `subs`.
pub fn segment(selector: &str, name: &str, subs: Vec<Structure>) -> Structure {
    Structure::Segment(Segment {
        selector: selector.to_string(),
        name: name.to_string(),
        subs,
    })
}

/// A [`Trash`] that removes the elements matched by `selector`.
pub fn trash(selector: &str) -> Structure {
    Structure::Trash(Trash {
        selector: selector.to_string(),
    })
}

/// A [`Scrub`] that blanks `attr` to `::` on every element matched by `selector`.
pub fn scrub(selector: &str, attr: &str) -> Structure {
    Structure::Scrub(Scrub {
        selector: selector.to_string(),
        attr: attr.to_string(),
    })
}

/// A [`Json`] that parses `selector`'s content and pulls `paths` as
/// `(path, name)` pairs, under `name` (empty merges into the parent).
pub fn json(selector: &str, name: &str, paths: Vec<(&str, &str)>) -> Structure {
    json_value(selector, "", name, paths)
}

/// A [`Json`] that slices the JSON value following `anchor` (a JS variable name)
/// out of `selector`'s script. An array yields one object per element.
pub fn json_after(selector: &str, anchor: &str, name: &str, paths: Vec<(&str, &str)>) -> Structure {
    json_value(selector, anchor, name, paths)
}

fn json_value(selector: &str, anchor: &str, name: &str, paths: Vec<(&str, &str)>) -> Structure {
    Structure::Json(Json {
        selector: selector.to_string(),
        anchor: anchor.to_string(),
        name: name.to_string(),
        paths: paths
            .into_iter()
            .map(|(key, name)| Attribute {
                key: key.to_string(),
                name: name.to_string(),
            })
            .collect(),
    })
}

/// A retailer's page architecture: the top-level [`Structure`]s of its pages.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RetailerArchitecture {
    /// The top-level structures, applied in order.
    pub structures: Vec<Structure>,
}

impl RetailerArchitecture {
    /// An architecture from a list of structures.
    pub fn new(structures: Vec<Structure>) -> Self {
        Self { structures }
    }
}

#[cfg(test)]
mod tests {
    use super::{Structure, collection, particle, segment};

    #[test]
    fn builds_a_definition_tree() {
        let reviews = collection("li.review", "reviews", vec![particle("p", "body", vec![("", "value")])]);
        let Structure::Collection(reviews) = reviews else {
            panic!("collection() should build a Collection");
        };
        assert_eq!(reviews.selector, "li.review");
        assert_eq!(reviews.subs.len(), 1);
        assert!(matches!(reviews.subs[0], Structure::Particle(_)));
    }

    #[test]
    fn particle_maps_attr_tuples() {
        // An empty key denotes the element's text content.
        let Structure::Particle(link) = particle("a", "link", vec![("href", "url"), ("", "label")]) else {
            panic!("particle() should build a Particle");
        };
        assert_eq!(link.attrs.len(), 2);
        assert_eq!(link.attrs[0].key, "href");
        assert_eq!(link.attrs[0].name, "url");
        assert_eq!(link.attrs[1].key, "");
        assert_eq!(link.attrs[1].name, "label");
    }

    #[test]
    fn segment_scopes_subs() {
        let Structure::Segment(head) =
            segment("head", "product", vec![particle("title", "title", vec![("", "value")])])
        else {
            panic!("segment() should build a Segment");
        };
        assert_eq!(head.name, "product");
        assert_eq!(head.subs.len(), 1);
    }
}
