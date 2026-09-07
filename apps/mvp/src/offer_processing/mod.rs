//! Stage E — offer page processing.
//!
//! Consumes the destructured product JSON (see the `destructure` binary) and
//! turns it into typed records. Each retailer has its own module
//! (e.g. [`minisforum_au`]) holding a `DestructuredProduct` mirror of its JSON.
