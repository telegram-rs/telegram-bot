use types::*;
use requests::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct GetChatMembersCount<'c> {
    chat_id: ChatId<'c>
}

impl<'c> Request for GetChatMembersCount<'c> {
    type Response = Integer;

    fn name(&self) -> &'static str {
        "getChatMembersCount"
    }
}

impl<'c> GetChatMembersCount<'c> {
    pub fn new<C>(chat: C) -> Self where C: Into<ChatId<'c>> {
        GetChatMembersCount {
            chat_id: chat.into()
        }
    }
}

pub trait CanGetChatMembersCount<'bc, 'c> {
    fn get_chat_members_count(&'bc self) -> GetChatMembersCount<'c>;
}

impl<'c, 'bc, C: 'bc> CanGetChatMembersCount<'bc, 'c> for C where &'bc C: Into<ChatId<'c>> {
    fn get_chat_members_count(&'bc self) -> GetChatMembersCount<'c> {
        GetChatMembersCount::new(self)
    }
}
