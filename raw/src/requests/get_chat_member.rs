use types::*;
use requests::*;

/// Use this method to get information about a member of a chat.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetChatMember<'c> {
    chat_id: ChatRef<'c>,
    user_id: UserId,
}

impl<'c> Request for GetChatMember<'c> {
    type Response = IdResponse<ChatMember>;

    fn name(&self) -> &'static str {
        "getChatMember"
    }
}

impl<'c> GetChatMember<'c> {
    pub fn new<C, U>(chat: C, user: U) -> Self where C: ToChatRef<'c>, U: ToUserId {
        GetChatMember {
            chat_id: chat.to_chat_ref(),
            user_id: user.to_user_id(),
        }
    }
}

/// Get information about a member of a chat.
pub trait CanGetChatMemberForChat<'c> {
    fn get_member<O>(&self, other: O) -> GetChatMember<'c> where O: ToUserId;
}

impl<'c, C> CanGetChatMemberForChat<'c> for C where C: ToChatRef<'c> {
    fn get_member<O>(&self, other: O) -> GetChatMember<'c> where O: ToUserId {
        GetChatMember::new(self, other)
    }
}

/// Get information about a member of a chat.
pub trait CanGetChatMemberForUser<'c> {
    fn get_member_from<O>(&self, other: O) -> GetChatMember<'c> where O: ToChatRef<'c>;
}

impl<'c, U> CanGetChatMemberForUser<'c> for U where U: ToUserId {
    fn get_member_from<O>(&self, other: O) -> GetChatMember<'c> where O: ToChatRef<'c> {
        GetChatMember::new(other, self)
    }
}
