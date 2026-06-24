pub mod minisforum_au;
pub mod minisforum_ca;
pub mod minisforum_eu;
pub mod minisforum_fr;
pub mod minisforum_hk;
pub mod minisforum_jp;
pub mod minisforum_kr;
pub mod minisforum_ru;
pub mod minisforum_uk;
pub mod minisforum_us;

use crate::SitemapConfig;
use crate::retailer::RetailerCode;

/// Resolves a retailer's sitemap configuration.
///
/// Returns `None` for codes that have no configured storefront yet (currently
/// the generic [`RetailerCode::Minisforum`]).
pub fn sitemap_config(code: RetailerCode) -> Option<SitemapConfig> {
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
    };

    if config.sitemap_url.is_empty() {
        return None;
    }

    Some(config)
}
