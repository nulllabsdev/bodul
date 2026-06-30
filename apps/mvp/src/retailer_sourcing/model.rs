use shared::retailer::RetailerCode;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Retailer {
    pub id: Uuid,
    pub name: String,
    pub code: RetailerCode,
}

pub struct InMemoryRetailerRepository {
    retailers: Vec<Retailer>,
}

impl InMemoryRetailerRepository {
    pub fn new() -> Self {
        let retailers = vec![
            Retailer {
                id: Uuid::parse_str("718c82c3-1b1e-4796-8f14-a32f3227cc00").unwrap(),
                name: "MinisForumEu".to_string(),
                code: RetailerCode::MinisForumEu,
            },
            Retailer {
                id: Uuid::parse_str("5cfcd46b-7966-4193-94a0-7e9954f14e5c").unwrap(),
                name: "MinisForumUs".to_string(),
                code: RetailerCode::MinisForumUs,
            },
            Retailer {
                id: Uuid::parse_str("b7d63252-3121-4370-af65-bc0b69345680").unwrap(),
                name: "MinisForumUk".to_string(),
                code: RetailerCode::MinisForumUk,
            },
            Retailer {
                id: Uuid::parse_str("2eec5987-ac7d-4fd9-ba84-0aa431cf903d").unwrap(),
                name: "MinisForumFr".to_string(),
                code: RetailerCode::MinisForumFr,
            },
            Retailer {
                id: Uuid::parse_str("4181c457-74c7-40dc-b225-7a4461fdb097").unwrap(),
                name: "MinisForumCa".to_string(),
                code: RetailerCode::MinisForumCa,
            },
            Retailer {
                id: Uuid::parse_str("acb3e558-6c04-43f6-ae0d-4573c784c1a1").unwrap(),
                name: "MinisForumAu".to_string(),
                code: RetailerCode::MinisForumAu,
            },
            Retailer {
                id: Uuid::parse_str("c41f0f3f-d95c-48c3-8896-50348351e549").unwrap(),
                name: "MinisForumKr".to_string(),
                code: RetailerCode::MinisForumKr,
            },
            Retailer {
                id: Uuid::parse_str("c00cc124-7cac-497f-94aa-942e9373cefd").unwrap(),
                name: "MinisForumJp".to_string(),
                code: RetailerCode::MinisForumJp,
            },
            Retailer {
                id: Uuid::parse_str("3c56cb03-15de-4523-99c5-fa96177e7993").unwrap(),
                name: "MinisForumRu".to_string(),
                code: RetailerCode::MinisForumRu,
            },
            Retailer {
                id: Uuid::parse_str("49cf7ebf-6a69-4380-8104-eed6d1916d2b").unwrap(),
                name: "MinisForumHk".to_string(),
                code: RetailerCode::MinisForumHk,
            },
        ];

        Self { retailers }
    }
}

impl InMemoryRetailerRepository {
    pub fn all(&self) -> Vec<Retailer> {
        self.retailers.clone()
    }
}

impl Default for InMemoryRetailerRepository {
    fn default() -> Self {
        Self::new()
    }
}
