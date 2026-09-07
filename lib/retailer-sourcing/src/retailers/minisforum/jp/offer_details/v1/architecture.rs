//! Page architecture for MinisForum JP (`jp.minisforum.com`).
//!
//! A MinisForum Shopify store (currency `JPY`, locale `ja`). It carries none of
//! the optional product scripts, but the main-product DOM section is present, so
//! the `xxxx` segment (gallery/variants/price) applies in addition to the JSON
//! blocks. See [`crate::retailers::minisforum::architecture_v1`] for the shared structure.

use crate::parsing::structure::RetailerArchitecture;
use crate::retailers::minisforum::architecture_v1::{Config, Xcotton, offer_detail_architecture_v1 as build};

/// The MinisForum JP page architecture.
pub fn offer_detail_architecture_v1() -> RetailerArchitecture {
    build(Config {
        tt_product: false,
        xcotton: Xcotton::None,
        const_product: false,
        const_product_variants: true,
    })
}
