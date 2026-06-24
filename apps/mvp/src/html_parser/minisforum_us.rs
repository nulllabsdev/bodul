//! Page architecture for MinisForum US (`minisforum.com` US store).
//!
//! A MinisForum Shopify store (currency `USD`, locale `en`). It carries the
//! `xcotton_pp_variants` and `const product = {...}` product scripts but not
//! `tt_product`. The main-product DOM section is absent on US pages, so the
//! `xxxx` segment yields nothing here — product data comes from the JSON blocks.
//! See [`super::minisforum`] for the shared structure.

use super::minisforum::{Config, Xcotton, architecture as build};
use super::structure::RetailerArchitecture;

/// The MinisForum US page architecture.
pub fn architecture() -> RetailerArchitecture {
    build(Config {
        tt_product: false,
        xcotton: Xcotton::Script,
        const_product: true,
        const_product_variants: false,
    })
}
