//! Page architecture for MinisForum EU (`minisforum.com` EU store).
//!
//! A MinisForum Shopify store (currency `EUR`, multilingual — locale `de`/`en`).
//! It carries the `xcotton_pp_variants` product script but not `tt_product` or the
//! `const product = {...}` JS block. See [`crate::retailers::minisforum::architecture_v1`] for the shared
//! structure.

use crate::parsing::structure::RetailerArchitecture;
use crate::retailers::minisforum::architecture_v1::{Config, Xcotton, offer_detail_architecture_v1 as build};

/// The MinisForum EU page architecture.
pub fn offer_detail_architecture_v1() -> RetailerArchitecture {
    build(Config {
        tt_product: false,
        xcotton: Xcotton::Script,
        const_product: false,
        const_product_variants: false,
    })
}
