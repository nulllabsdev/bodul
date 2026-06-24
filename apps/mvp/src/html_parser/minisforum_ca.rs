//! Page architecture for MinisForum CA (`ca.minisforum.com`).
//!
//! A MinisForum Shopify store (currency `CAD`, locale `en`). It carries all three
//! optional product scripts (`tt_product`, `xcotton_pp_variants`, and the
//! `const product = {...}` JS block). See [`super::minisforum`] for the shared
//! structure.

use super::minisforum::{Config, Xcotton, architecture as build};
use super::structure::RetailerArchitecture;

/// The MinisForum CA page architecture.
pub fn architecture() -> RetailerArchitecture {
    build(Config {
        tt_product: true,
        xcotton: Xcotton::Script,
        const_product: true,
        const_product_variants: false,
    })
}
