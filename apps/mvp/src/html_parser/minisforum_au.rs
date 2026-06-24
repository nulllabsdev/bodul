//! Page architecture for MinisForum AU (`au.minisforum.com`).
//!
//! A MinisForum Shopify store (currency `AUD`, locale `en`). It carries the
//! `tt_product` and `const product = {...}` scripts but no `xcotton_pp_variants`
//! (in either form). See [`super::minisforum`] for the shared structure.

use super::minisforum::{Config, Xcotton, architecture as build};
use super::structure::RetailerArchitecture;

/// The MinisForum AU page architecture.
pub fn architecture() -> RetailerArchitecture {
    build(Config {
        tt_product: true,
        xcotton: Xcotton::None,
        const_product: true,
        const_product_variants: false,
    })
}
