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
    pub fn from_slug(slug: &str) -> Option<RetailerCode> {
        RetailerCode::ALL
            .into_iter()
            .find(|code| format!("{code:?}").to_lowercase() == slug)
    }
}

#[derive(Debug, Clone)]
pub struct Retailer {
    pub id: Uuid,
    pub name: String,
    pub code: RetailerCode,
}

#[cfg(test)]
mod tests {
    use super::RetailerCode;

    #[test]
    fn resolves_code_from_slug() {
        assert_eq!(
            RetailerCode::from_slug("minisforumeu"),
            Some(RetailerCode::MinisForumEu)
        );
        assert_eq!(RetailerCode::from_slug("nope"), None);
    }
}
