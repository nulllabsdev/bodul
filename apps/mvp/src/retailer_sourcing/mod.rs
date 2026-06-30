//! Retailer sourcing.
//!
//! Initiates product sourcing for active retailers, triggering the sitemap
//! pipeline (roadmap Stage A). Triggered manually in Phase 0.

use crate::retailer_sourcing::retailers::{
    anker_au, anker_ca, anker_com, anker_de, anker_eu, anker_fr, anker_italy_com, anker_japan_com, anker_kr, anker_my,
    anker_nordics_com, anker_nz, anker_pl, anker_uk, anker_vn, mi_com, minisforum_au, minisforum_ca, minisforum_eu,
    minisforum_fr, minisforum_hk, minisforum_jp, minisforum_kr, minisforum_ru, minisforum_uk, minisforum_us, ugreen_ca,
    ugreen_com, ugreen_de, ugreen_eu, ugreen_fr, ugreen_in, ugreen_jp, ugreen_kr, ugreen_nas, ugreen_nas_au,
    ugreen_nas_ca, ugreen_nas_de, ugreen_nas_es, ugreen_nas_eu, ugreen_nas_fr, ugreen_nas_it, ugreen_nas_jp,
    ugreen_nas_uk, ugreen_nl, ugreen_uk, ugreen_us,
};
use shared::SitemapConfig;
use shared::link::LinkKind;
use shared::retailer::RetailerCode;

pub mod model;
pub mod retailers;

/// Resolves a retailer's sitemap configuration.
///
/// Retailers without known sitemap URLs return a config with an empty URL list.
pub fn sitemap_config(code: &RetailerCode) -> Option<SitemapConfig> {
    let config = match code {
        RetailerCode::MinisForumEu => minisforum_eu::sitemap_config(),
        RetailerCode::MinisForumUs => minisforum_us::sitemap_config(),
        RetailerCode::MinisForumUk => minisforum_uk::sitemap_config(),
        RetailerCode::MinisForumFr => minisforum_fr::sitemap_config(),
        RetailerCode::MinisForumCa => minisforum_ca::sitemap_config(),
        RetailerCode::MinisForumAu => minisforum_au::sitemap_config(),
        RetailerCode::MinisForumKr => minisforum_kr::sitemap_config(),
        RetailerCode::MinisForumJp => minisforum_jp::sitemap_config(),
        RetailerCode::MinisForumRu => minisforum_ru::sitemap_config(),
        RetailerCode::MinisForumHk => minisforum_hk::sitemap_config(),
        RetailerCode::MiCom => mi_com::sitemap_config(),
        RetailerCode::UgreenCom => ugreen_com::sitemap_config(),
        RetailerCode::UgreenUs => ugreen_us::sitemap_config(),
        RetailerCode::UgreenCa => ugreen_ca::sitemap_config(),
        RetailerCode::UgreenEu => ugreen_eu::sitemap_config(),
        RetailerCode::UgreenDe => ugreen_de::sitemap_config(),
        RetailerCode::UgreenUk => ugreen_uk::sitemap_config(),
        RetailerCode::UgreenFr => ugreen_fr::sitemap_config(),
        RetailerCode::UgreenNl => ugreen_nl::sitemap_config(),
        RetailerCode::UgreenJp => ugreen_jp::sitemap_config(),
        RetailerCode::UgreenKr => ugreen_kr::sitemap_config(),
        RetailerCode::UgreenIn => ugreen_in::sitemap_config(),
        RetailerCode::UgreenNas => ugreen_nas::sitemap_config(),
        RetailerCode::UgreenNasCa => ugreen_nas_ca::sitemap_config(),
        RetailerCode::UgreenNasEu => ugreen_nas_eu::sitemap_config(),
        RetailerCode::UgreenNasDe => ugreen_nas_de::sitemap_config(),
        RetailerCode::UgreenNasUk => ugreen_nas_uk::sitemap_config(),
        RetailerCode::UgreenNasFr => ugreen_nas_fr::sitemap_config(),
        RetailerCode::UgreenNasEs => ugreen_nas_es::sitemap_config(),
        RetailerCode::UgreenNasIt => ugreen_nas_it::sitemap_config(),
        RetailerCode::UgreenNasAu => ugreen_nas_au::sitemap_config(),
        RetailerCode::UgreenNasJp => ugreen_nas_jp::sitemap_config(),
        RetailerCode::AnkerCom => anker_com::sitemap_config(),
        RetailerCode::AnkerJapanCom => anker_japan_com::sitemap_config(),
        RetailerCode::AnkerKr => anker_kr::sitemap_config(),
        RetailerCode::AnkerItalyCom => anker_italy_com::sitemap_config(),
        RetailerCode::AnkerNordicsCom => anker_nordics_com::sitemap_config(),
        RetailerCode::AnkerUk => anker_uk::sitemap_config(),
        RetailerCode::AnkerCa => anker_ca::sitemap_config(),
        RetailerCode::AnkerEu => anker_eu::sitemap_config(),
        RetailerCode::AnkerDe => anker_de::sitemap_config(),
        RetailerCode::AnkerFr => anker_fr::sitemap_config(),
        RetailerCode::AnkerPl => anker_pl::sitemap_config(),
        RetailerCode::AnkerAu => anker_au::sitemap_config(),
        RetailerCode::AnkerNz => anker_nz::sitemap_config(),
        RetailerCode::AnkerMy => anker_my::sitemap_config(),
        RetailerCode::AnkerVn => anker_vn::sitemap_config(),
    };

    if config.sitemap_url.is_empty() {
        return None;
    }

    Some(config)
}

/// Classifies a page URL using `code`'s retailer-specific rules.
///
/// Each retailer's rule lives alongside its `sitemap_config` (a `from_location`
/// fn in its module). Retailers without a rule yet fall through to
/// [`LinkKind::Unknown`].
pub fn classify_link(code: RetailerCode, url: &str, source: &str, _image_count: usize) -> LinkKind {
    match code {
        RetailerCode::MinisForumEu => minisforum_eu::from_location(url),
        RetailerCode::MinisForumUs => minisforum_us::from_location(url),
        RetailerCode::MinisForumUk => minisforum_uk::from_location(url),
        RetailerCode::MinisForumFr => minisforum_fr::from_location(url),
        RetailerCode::MinisForumCa => minisforum_ca::from_location(url),
        RetailerCode::MinisForumAu => minisforum_au::from_location(url),
        RetailerCode::MinisForumKr => minisforum_kr::from_location(url),
        RetailerCode::MinisForumJp => minisforum_jp::from_location(url),
        RetailerCode::MinisForumRu => minisforum_ru::from_location(url),
        RetailerCode::MinisForumHk => minisforum_hk::from_location(url),
        RetailerCode::MiCom => mi_com::from_location(url),
        RetailerCode::UgreenCom => ugreen_com::from_location(url),
        RetailerCode::UgreenUs => ugreen_us::from_location(url),
        RetailerCode::UgreenCa => ugreen_ca::from_location(url),
        RetailerCode::UgreenEu => ugreen_eu::from_location(url),
        RetailerCode::UgreenDe => ugreen_de::from_location(url),
        RetailerCode::UgreenUk => ugreen_uk::from_location(url),
        RetailerCode::UgreenFr => ugreen_fr::from_location(url),
        RetailerCode::UgreenNl => ugreen_nl::from_location(url),
        RetailerCode::UgreenJp => ugreen_jp::from_location(url),
        RetailerCode::UgreenKr => ugreen_kr::from_location(url),
        RetailerCode::UgreenIn => ugreen_in::from_location(url),
        RetailerCode::UgreenNas => ugreen_nas::from_location(url),
        RetailerCode::UgreenNasCa => ugreen_nas_ca::from_location(url),
        RetailerCode::UgreenNasEu => ugreen_nas_eu::from_location(url),
        RetailerCode::UgreenNasDe => ugreen_nas_de::from_location(url),
        RetailerCode::UgreenNasUk => ugreen_nas_uk::from_location(url),
        RetailerCode::UgreenNasFr => ugreen_nas_fr::from_location(url),
        RetailerCode::UgreenNasEs => ugreen_nas_es::from_location(url),
        RetailerCode::UgreenNasIt => ugreen_nas_it::from_location(url),
        RetailerCode::UgreenNasAu => ugreen_nas_au::from_location(url),
        RetailerCode::UgreenNasJp => ugreen_nas_jp::from_location(url),
        RetailerCode::AnkerCom => anker_com::from_location(url),
        RetailerCode::AnkerJapanCom => anker_japan_com::from_location(url, source),
        RetailerCode::AnkerKr => anker_kr::from_location(url, source),
        RetailerCode::AnkerItalyCom => anker_italy_com::from_location(url, source),
        RetailerCode::AnkerNordicsCom => anker_nordics_com::from_location(url, source),
        RetailerCode::AnkerUk => anker_uk::from_location(url, source),
        RetailerCode::AnkerCa => anker_ca::from_location(url, source),
        RetailerCode::AnkerEu => anker_eu::from_location(url, source),
        RetailerCode::AnkerDe => anker_de::from_location(url, source),
        RetailerCode::AnkerFr => anker_fr::from_location(url, source),
        RetailerCode::AnkerPl => anker_pl::from_location(url, source),
        RetailerCode::AnkerAu => anker_au::from_location(url, source),
        RetailerCode::AnkerNz => anker_nz::from_location(url, source),
        RetailerCode::AnkerMy => anker_my::from_location(url, source),
        RetailerCode::AnkerVn => anker_vn::from_location(url, source),
    }
}
