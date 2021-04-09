use crate::types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct ChatMemberUpdate {
    pub chat: Chat,
    pub from: User,
    pub date: Integer,
    pub old_chat_member: ChatMember,
    pub new_chat_member: ChatMember,
    pub invite_link: Option<ChatInviteLink>,
}
