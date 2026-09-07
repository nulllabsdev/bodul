//! HTML parsing.
//!
//! Extracts product and offer fields from stored product-page HTML using
//! retailer-specific rules (roadmap Stage E). Consumed by `offer_processing`.
//!
//! Each retailer defines a [`RetailerArchitecture`] — a tree of [`Structure`]
//! definitions built with [`structure::particle`], [`structure::collection`] and
//! [`structure::segment`]. The same architecture drives different consumers (the
//! `destructure` and `valueless` binaries); [`architecture_for`] returns it and
//! [`destructure`] applies it to extract values.

use kuchiki::traits::*;
use retailer_sourcing::architecture_for;
use shared::retailer::RetailerCode;

mod blank;
mod extract;
mod feature_chart;

pub use ::retailer_sourcing::parsing::structure::{
    Attribute, Collection, Json, Particle, RetailerArchitecture, Scrub, Segment, Structure, Trash, collection, json,
    json_after, particle, scrub, segment, trash,
};

/// Parses `html` and extracts `retailer`'s architecture into a JSON tree.
pub fn destructure(html: &str, retailer: RetailerCode) -> serde_json::Value {
    let document = kuchiki::parse_html().one(html);
    let mut value = extract::extract(&document, &architecture_for(retailer));
    feature_chart::transpose(&mut value);
    value
}

/// The blanked outputs of one page.
pub struct Valueless {
    /// The whole page, blanked.
    pub page: String,
    /// Each top-level segment's blanked HTML, as `(name, html)`.
    pub sections: Vec<(String, String)>,
    /// Each lifted collection item, as `(collection_name, index, html)`.
    pub components: Vec<(String, usize, String)>,
}

/// Parses `html`, blanks it with `retailer`'s architecture, and returns the page,
/// each top-level section's HTML, and every lifted collection [`component`].
///
/// Collections are componentized: each item is replaced in the page by a
/// `[name_index]` placeholder and returned separately.
pub fn valueless(html: &str, retailer: RetailerCode) -> Result<Valueless, std::io::Error> {
    let document = kuchiki::parse_html().one(html);
    let architecture = architecture_for(retailer);
    let lifted = blank::apply(&document, &architecture);

    let page = serialize_node(&document)?;

    let mut sections = Vec::new();
    collect_sections(&document, &architecture.structures, &mut sections)?;

    let mut components = Vec::new();
    for component in lifted {
        components.push((component.name, component.index, serialize_node(&component.node)?));
    }

    Ok(Valueless {
        page,
        sections,
        components,
    })
}

/// Collects a section file for every [`Segment`] in `structures`, at any depth.
///
/// A segment's element is selected relative to `context`, then its own subs are
/// searched (within the matched element) for further nested segments.
fn collect_sections(
    context: &kuchiki::NodeRef,
    structures: &[Structure],
    sections: &mut Vec<(String, String)>,
) -> Result<(), std::io::Error> {
    for structure in structures {
        let Structure::Segment(segment) = structure else {
            continue;
        };
        let Ok(matched) = context.select_first(&segment.selector) else {
            continue;
        };
        sections.push((segment.name.clone(), serialize_node(matched.as_node())?));
        collect_sections(matched.as_node(), &segment.subs, sections)?;
    }
    Ok(())
}

/// Serializes a node (including itself) to an HTML string.
fn serialize_node(node: &kuchiki::NodeRef) -> Result<String, std::io::Error> {
    let mut out = Vec::new();
    node.serialize(&mut out)?;
    Ok(String::from_utf8_lossy(&out).into_owned())
}
