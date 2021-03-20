use crate::types::*;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,
    pub location: Option<Location>,
    pub inline_message_id: Option<String>,
    pub query: String,
}
