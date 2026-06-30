//! Page architecture for Anker New Zealand (`www.anker.com/nz`).

use super::RetailerArchitecture;
use super::anker::architecture as build;

pub fn architecture() -> RetailerArchitecture {
    build()
}
