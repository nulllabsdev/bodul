//! Page architecture for MinisForum UK (`uk.minisforum.com`).
//!
//! A MinisForum Shopify store (currency `GBP`, locale `en`). It carries the
//! `tt_product` and `xcotton_pp_variants` product scripts but not the
//! `const product = {...}` JS block. See [`super::minisforum`] for the shared
//! structure.

use super::minisforum::{Config, Xcotton, architecture as build};
use super::structure::RetailerArchitecture;

/// The MinisForum UK page architecture.
pub fn architecture() -> RetailerArchitecture {
    build(Config {
        tt_product: true,
        xcotton: Xcotton::JsVar,
        const_product: false,
        const_product_variants: false,
    })
}
