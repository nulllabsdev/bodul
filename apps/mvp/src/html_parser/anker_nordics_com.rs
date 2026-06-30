//! Page architecture for Anker Nordics (`www.ankernordics.com`).

use super::RetailerArchitecture;
use super::anker::architecture as build;

pub fn architecture() -> RetailerArchitecture {
    build()
}
