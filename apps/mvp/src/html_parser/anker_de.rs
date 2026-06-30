//! Page architecture for Anker Germany (`www.anker.com/eu-de`).

use super::RetailerArchitecture;
use super::anker::architecture as build;

pub fn architecture() -> RetailerArchitecture {
    build()
}
