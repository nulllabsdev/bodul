//! Page architecture for MinisForum CA (`ca.minisforum.com`).
//!
//! A MinisForum Shopify store (currency `CAD`, locale `en`). It carries all three
//! optional product scripts (`tt_product`, `xcotton_pp_variants`, and the
//! `const product = {...}` JS block). See [`crate::retailers::minisforum::architecture_v1`] for the shared
//! structure.

use crate::parsing::structure::RetailerArchitecture;
use crate::retailers::minisforum::architecture_v1::{Config, Xcotton, offer_detail_architecture_v1 as build};

/// The MinisForum CA page architecture.
pub fn offer_detail_architecture_v1() -> RetailerArchitecture {
    build(Config {
        tt_product: true,
        xcotton: Xcotton::Script,
        const_product: true,
        const_product_variants: true,
    })
}
