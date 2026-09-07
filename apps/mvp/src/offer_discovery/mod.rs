// Stage D — offer discovery: the `offers` and `raw_offers` persistence layer.
//
// Only the schema mapping lives here for now; the discovery subscriber and the
// download handler that fill these tables arrive with the feature.

pub mod download;
pub mod model;

pub mod io {
    pub use super::download::io::{
        DOWNLOAD_OFFER_PAGE_COMMAND, DownloadOfferPage, DownloadOfferPageHandler, OFFER_PAGE_DOWNLOAD_SKIPPED_EVENT,
        OFFER_PAGE_WAS_DOWNLOADED_EVENT, OfferPageDownloadSkipped, OfferPageWasDownloaded, OfferRepository,
        RawOfferRepository,
    };
}
