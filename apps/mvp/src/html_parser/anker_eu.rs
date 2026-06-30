//! Page architecture for Anker EU (`www.anker.com/eu-en`).

use super::RetailerArchitecture;
use super::anker::architecture as build;

pub fn architecture() -> RetailerArchitecture {
    build()
}
