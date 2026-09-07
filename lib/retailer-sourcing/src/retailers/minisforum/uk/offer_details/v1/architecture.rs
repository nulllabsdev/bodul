//! Page architecture for MinisForum UK (`uk.minisforum.com`).
//!
//! A MinisForum Shopify store (currency `GBP`, locale `en`). It carries the
//! `tt_product` and `xcotton_pp_variants` product scripts but not the
//! `const product = {...}` JS block. See [`crate::retailers::minisforum::architecture_v1`] for the shared
//! structure.

use crate::parsing::structure::RetailerArchitecture;
use crate::retailers::minisforum::architecture_v1::{Config, Xcotton, offer_detail_architecture_v1 as build};

/// The MinisForum UK page architecture.
pub fn offer_detail_architecture_v1() -> RetailerArchitecture {
    build(Config {
        tt_product: true,
        xcotton: Xcotton::JsVar,
        const_product: false,
        const_product_variants: false,
    })
}
