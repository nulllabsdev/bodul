//! Page architecture for MinisForum US (`minisforum.com` US store).
//!
//! A MinisForum Shopify store (currency `USD`, locale `en`). It carries the
//! `xcotton_pp_variants` and `const product = {...}` product scripts but not
//! `tt_product`. The main-product DOM section is absent on US pages, so the
//! `xxxx` segment yields nothing here — product data comes from the JSON blocks.
//! See [`crate::retailers::minisforum::architecture_v1`] for the shared structure.

use crate::parsing::structure::RetailerArchitecture;
use crate::retailers::minisforum::architecture_v1::{Config, Xcotton, offer_detail_architecture_v1 as build};

/// The MinisForum US page architecture.
pub fn offer_detail_architecture_v1() -> RetailerArchitecture {
    build(Config {
        tt_product: false,
        xcotton: Xcotton::Script,
        const_product: true,
        const_product_variants: false,
    })
}
