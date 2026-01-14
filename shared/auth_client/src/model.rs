//! Domain models for authentication and authorization.

/// Service tokens.
pub mod tokens {
    pub const VENDOR_SOURCING: &str = "6db46e37-2e76-420d-ae41-8091c43bb790";
    pub const RETRIEVER: &str = "fc2b6164-aefc-4d66-89dc-5e463f946ad3";
    pub const CRAWLER: &str = "e3b0c442-98fc-1c14-9afc-4c8996fb9242";
}

/// Service client identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientId {
    VendorSourcing,
    Retriever,
    Crawler,
}

/// Permission types for service authorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    FetchUrl,
    TestPermission,
}

/// Role types for service authorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    TestRole,
}
