//! Page architecture for MinisForum JP (`jp.minisforum.com`).
//!
//! A MinisForum Shopify store (currency `JPY`, locale `ja`). It carries none of
//! the optional product scripts, but the main-product DOM section is present, so
//! the `xxxx` segment (gallery/variants/price) applies in addition to the JSON
//! blocks. See [`super::minisforum`] for the shared structure.

use super::minisforum::{Config, Xcotton, architecture as build};
use super::structure::RetailerArchitecture;

/// The MinisForum JP page architecture.
pub fn architecture() -> RetailerArchitecture {
    build(Config {
        tt_product: false,
        xcotton: Xcotton::None,
        const_product: false,
        const_product_variants: true,
    })
}
