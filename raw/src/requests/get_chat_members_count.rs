use types::*;
use requests::*;

/// Use this method to get the number of members in a chat.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetChatMembersCount<'c> {
    chat_id: ChatRef<'c>
}

impl<'c> Request for GetChatMembersCount<'c> {
    type Response = IdResponse<Integer>;

    fn name(&self) -> &'static str {
        "getChatMembersCount"
    }
}

impl<'c> GetChatMembersCount<'c> {
    pub fn new<C>(chat: C) -> Self where C: ToChatRef<'c> {
        GetChatMembersCount {
            chat_id: chat.to_chat_ref()
        }
    }
}

pub trait CanGetChatMembersCount<'c> {
    fn get_members_count(&self) -> GetChatMembersCount<'c>;
}

impl<'c, C> CanGetChatMembersCount<'c> for C where C: ToChatRef<'c> {
    fn get_members_count(&self) -> GetChatMembersCount<'c> {
        GetChatMembersCount::new(self)
    }
}
