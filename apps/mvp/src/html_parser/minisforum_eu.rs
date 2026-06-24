//! Page architecture for MinisForum EU (`minisforum.com` EU store).
//!
//! A MinisForum Shopify store (currency `EUR`, multilingual — locale `de`/`en`).
//! It carries the `xcotton_pp_variants` product script but not `tt_product` or the
//! `const product = {...}` JS block. See [`super::minisforum`] for the shared
//! structure.

use super::minisforum::{Config, Xcotton, architecture as build};
use super::structure::RetailerArchitecture;

/// The MinisForum EU page architecture.
pub fn architecture() -> RetailerArchitecture {
    build(Config {
        tt_product: false,
        xcotton: Xcotton::Script,
        const_product: false,
        const_product_variants: false,
    })
}
