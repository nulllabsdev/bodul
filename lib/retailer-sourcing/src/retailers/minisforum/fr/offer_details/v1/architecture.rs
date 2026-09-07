//! Page architecture for MinisForum FR (`fr.minisforum.com`).
//!
//! A MinisForum Shopify store (currency `EUR`, locale `en`). It carries only the
//! `xcotton_pp_variants` product script (no `tt_product`, no `const product`).
//! See [`crate::retailers::minisforum::architecture_v1`] for the shared structure.

use crate::parsing::structure::RetailerArchitecture;
use crate::retailers::minisforum::architecture_v1::{Config, Xcotton, offer_detail_architecture_v1 as build};

/// The MinisForum FR page architecture.
pub fn offer_detail_architecture_v1() -> RetailerArchitecture {
    build(Config {
        tt_product: false,
        xcotton: Xcotton::JsVar,
        const_product: false,
        const_product_variants: true,
    })
}
