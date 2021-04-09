use crate::types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct ChatInviteLink {
    pub invite_link: String,
    pub creator: User,
    pub is_primary: bool,
    pub is_revoked: bool,
    pub expire_date: Integer,
    pub member_limit: Integer,
}
