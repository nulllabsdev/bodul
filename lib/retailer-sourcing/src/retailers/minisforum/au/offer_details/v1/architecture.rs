//! Page architecture for MinisForum AU (`au.minisforum.com`).
//!
//! A MinisForum Shopify store (currency `AUD`, locale `en`). It carries the
//! `tt_product` and `const product = {...}` scripts but no `xcotton_pp_variants`
//! (in either form). See [`crate::retailers::minisforum::architecture_v1`] for the shared structure.

use crate::parsing::structure::RetailerArchitecture;
use crate::retailers::minisforum::architecture_v1::{Config, Xcotton, offer_detail_architecture_v1 as build};

/// The MinisForum AU page architecture.
pub fn offer_detail_architecture_v1() -> RetailerArchitecture {
    build(Config {
        tt_product: true,
        xcotton: Xcotton::None,
        const_product: true,
        const_product_variants: true,
    })
}
