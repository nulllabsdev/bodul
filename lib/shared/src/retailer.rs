use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RetailerCode {
    MinisForumEu,
    MinisForumUs,
    MinisForumUk,
    MinisForumFr,
    MinisForumCa,
    MinisForumAu,
    MinisForumKr,
    MinisForumJp,
    MinisForumRu,
    MinisForumHk,
}

impl RetailerCode {
    /// Every known retailer code.
    pub const ALL: [RetailerCode; 10] = [
        RetailerCode::MinisForumEu,
        RetailerCode::MinisForumUs,
        RetailerCode::MinisForumUk,
        RetailerCode::MinisForumFr,
        RetailerCode::MinisForumCa,
        RetailerCode::MinisForumAu,
        RetailerCode::MinisForumKr,
        RetailerCode::MinisForumJp,
        RetailerCode::MinisForumRu,
        RetailerCode::MinisForumHk,
    ];
}

#[derive(Debug, Clone)]
pub struct Retailer {
    pub id: Uuid,
    pub name: String,
    pub code: RetailerCode,
}
