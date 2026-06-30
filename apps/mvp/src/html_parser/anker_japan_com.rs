//! Page architecture for Anker Japan (`www.ankerjapan.com`).

use super::RetailerArchitecture;
use super::anker::architecture as build;

pub fn architecture() -> RetailerArchitecture {
    build()
}
