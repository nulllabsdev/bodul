//! Stage E — offer page processing.
//!
//! Consumes the destructured product JSON (see the `destructure` binary) and
//! turns it into typed records. Each retailer has its own module
//! (e.g. [`minisforum_au`]) holding a `DestructuredProduct` mirror of its JSON.

pub mod minisforum_au;
pub mod minisforum_ca;
pub mod minisforum_eu;
pub mod minisforum_fr;
pub mod minisforum_hk;
pub mod minisforum_jp;
pub mod minisforum_kr;
pub mod minisforum_ru;
pub mod minisforum_uk;
pub mod minisforum_us;
