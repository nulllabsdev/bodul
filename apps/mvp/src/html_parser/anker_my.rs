//! Page architecture for Anker Malaysia (`www.anker.com/my`).

use super::RetailerArchitecture;
use super::anker::architecture as build;

pub fn architecture() -> RetailerArchitecture {
    build()
}
