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
use shared::retailer::RetailerCode;

mod anker;
mod anker_au;
mod anker_ca;
mod anker_com;
mod anker_de;
mod anker_eu;
mod anker_fr;
mod anker_italy_com;
mod anker_japan_com;
mod anker_kr;
mod anker_my;
mod anker_nordics_com;
mod anker_nz;
mod anker_pl;
mod anker_uk;
mod anker_vn;
mod blank;
mod extract;
mod feature_chart;
mod mi_com;
mod minisforum;
mod minisforum_au;
mod minisforum_ca;
mod minisforum_eu;
mod minisforum_fr;
mod minisforum_hk;
mod minisforum_jp;
mod minisforum_kr;
mod minisforum_ru;
mod minisforum_uk;
mod minisforum_us;
mod structure;
mod ugreen_ca;
mod ugreen_com;
mod ugreen_de;
mod ugreen_eu;
mod ugreen_fr;
mod ugreen_in;
mod ugreen_jp;
mod ugreen_kr;
mod ugreen_nas;
mod ugreen_nas_au;
mod ugreen_nas_ca;
mod ugreen_nas_de;
mod ugreen_nas_es;
mod ugreen_nas_eu;
mod ugreen_nas_fr;
mod ugreen_nas_it;
mod ugreen_nas_jp;
mod ugreen_nas_uk;
mod ugreen_nl;
mod ugreen_uk;
mod ugreen_us;

pub use structure::{
    Attribute, Collection, Json, Particle, RetailerArchitecture, Scrub, Segment, Structure, Trash, collection, json,
    json_after, particle, scrub, segment, trash,
};

/// The page architecture for `retailer`.
///
/// Retailers without an architecture yet return an empty one, yielding empty
/// output.
pub fn architecture_for(retailer: RetailerCode) -> RetailerArchitecture {
    match retailer {
        RetailerCode::MinisForumAu => minisforum_au::architecture(),
        RetailerCode::MinisForumCa => minisforum_ca::architecture(),
        RetailerCode::MinisForumEu => minisforum_eu::architecture(),
        RetailerCode::MinisForumUs => minisforum_us::architecture(),
        RetailerCode::MinisForumUk => minisforum_uk::architecture(),
        RetailerCode::MinisForumFr => minisforum_fr::architecture(),
        RetailerCode::MinisForumKr => minisforum_kr::architecture(),
        RetailerCode::MinisForumJp => minisforum_jp::architecture(),
        RetailerCode::MinisForumRu => minisforum_ru::architecture(),
        RetailerCode::MinisForumHk => minisforum_hk::architecture(),
        RetailerCode::MiCom => mi_com::architecture(),
        RetailerCode::UgreenCom => ugreen_com::architecture(),
        RetailerCode::UgreenUs => ugreen_us::architecture(),
        RetailerCode::UgreenCa => ugreen_ca::architecture(),
        RetailerCode::UgreenEu => ugreen_eu::architecture(),
        RetailerCode::UgreenDe => ugreen_de::architecture(),
        RetailerCode::UgreenUk => ugreen_uk::architecture(),
        RetailerCode::UgreenFr => ugreen_fr::architecture(),
        RetailerCode::UgreenNl => ugreen_nl::architecture(),
        RetailerCode::UgreenJp => ugreen_jp::architecture(),
        RetailerCode::UgreenKr => ugreen_kr::architecture(),
        RetailerCode::UgreenIn => ugreen_in::architecture(),
        RetailerCode::UgreenNas => ugreen_nas::architecture(),
        RetailerCode::UgreenNasCa => ugreen_nas_ca::architecture(),
        RetailerCode::UgreenNasEu => ugreen_nas_eu::architecture(),
        RetailerCode::UgreenNasDe => ugreen_nas_de::architecture(),
        RetailerCode::UgreenNasUk => ugreen_nas_uk::architecture(),
        RetailerCode::UgreenNasFr => ugreen_nas_fr::architecture(),
        RetailerCode::UgreenNasEs => ugreen_nas_es::architecture(),
        RetailerCode::UgreenNasIt => ugreen_nas_it::architecture(),
        RetailerCode::UgreenNasAu => ugreen_nas_au::architecture(),
        RetailerCode::UgreenNasJp => ugreen_nas_jp::architecture(),
        RetailerCode::AnkerCom => anker_com::architecture(),
        RetailerCode::AnkerJapanCom => anker_japan_com::architecture(),
        RetailerCode::AnkerKr => anker_kr::architecture(),
        RetailerCode::AnkerItalyCom => anker_italy_com::architecture(),
        RetailerCode::AnkerNordicsCom => anker_nordics_com::architecture(),
        RetailerCode::AnkerUk => anker_uk::architecture(),
        RetailerCode::AnkerCa => anker_ca::architecture(),
        RetailerCode::AnkerEu => anker_eu::architecture(),
        RetailerCode::AnkerDe => anker_de::architecture(),
        RetailerCode::AnkerFr => anker_fr::architecture(),
        RetailerCode::AnkerPl => anker_pl::architecture(),
        RetailerCode::AnkerAu => anker_au::architecture(),
        RetailerCode::AnkerNz => anker_nz::architecture(),
        RetailerCode::AnkerMy => anker_my::architecture(),
        RetailerCode::AnkerVn => anker_vn::architecture(),
    }
}

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
