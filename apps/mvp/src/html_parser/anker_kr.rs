//! Page architecture for Anker Korea (`ankerkorea.co.kr`).

use super::RetailerArchitecture;
use super::anker::architecture as build;

pub fn architecture() -> RetailerArchitecture {
    build()
}
