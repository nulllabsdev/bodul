//! Page architecture for Anker UK (`www.anker.com/uk`).

use super::RetailerArchitecture;
use super::anker::architecture as build;

pub fn architecture() -> RetailerArchitecture {
    build()
}
