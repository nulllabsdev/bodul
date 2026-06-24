//! Page architecture for MinisForum FR (`fr.minisforum.com`).
//!
//! A MinisForum Shopify store (currency `EUR`, locale `en`). It carries only the
//! `xcotton_pp_variants` product script (no `tt_product`, no `const product`).
//! See [`super::minisforum`] for the shared structure.

use super::minisforum::{Config, Xcotton, architecture as build};
use super::structure::RetailerArchitecture;

/// The MinisForum FR page architecture.
pub fn architecture() -> RetailerArchitecture {
    build(Config {
        tt_product: false,
        xcotton: Xcotton::JsVar,
        const_product: false,
        const_product_variants: false,
    })
}
