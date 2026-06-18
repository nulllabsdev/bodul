use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RetailerCode {
    Minisforum,
}

#[derive(Debug, Clone)]
pub struct Retailer {
    pub id: Uuid,
    pub name: String,
    pub code: RetailerCode,
}
