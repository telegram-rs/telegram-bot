use crate::types::*;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct InlineQuery {
    pub id: InlineQueryId,
    pub from: User,
    pub location: Option<Location>,
    pub query: String,
    pub offset: String,
}

impl Into<InlineQueryId> for InlineQuery {
    fn into(self) -> InlineQueryId {
        self.id
    }
}
