use types::primitive::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct ResponseParameters {
    pub migrate_to_chat_id: Option<Integer>,
    pub retry_after: Option<Integer>,
}
