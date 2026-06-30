//! Page architecture for Anker Poland (`www.anker.com/eu-pl`).

use super::RetailerArchitecture;
use super::anker::architecture as build;

pub fn architecture() -> RetailerArchitecture {
    build()
}
