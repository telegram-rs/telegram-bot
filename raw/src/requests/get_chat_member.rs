use types::*;
use requests::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct GetChatMember<'c> {
    chat_id: ChatId<'c>,
    user_id: UserId,
}

impl<'c> Request for GetChatMember<'c> {
    type Response = ChatMember;

    fn name(&self) -> &'static str {
        "getChatMember"
    }
}

impl<'c> GetChatMember<'c> {
    pub fn new<C, U>(chat: C, user: U) -> Self where C: Into<ChatId<'c>>, U: Into<UserId> {
        GetChatMember {
            chat_id: chat.into(),
            user_id: user.into(),
        }
    }
}

pub trait CanGetChatMemberForChat<'sc, 'c> {
    fn get_chat_member<O>(&'sc self, other: O) -> GetChatMember<'c> where O: Into<UserId>;
}

impl<'c, 'sc, C: 'sc> CanGetChatMemberForChat<'sc, 'c> for C where &'sc C: Into<ChatId<'c>> {
    fn get_chat_member<O>(&'sc self, other: O) -> GetChatMember<'c> where O: Into<UserId> {
        GetChatMember::new(self, other)
    }
}

pub trait CanGetChatMemberForUser<'sc, 'c> {
    fn get_chat_member<O>(&'sc self, other: O) -> GetChatMember<'c> where O: Into<ChatId<'c>>;
}

impl<'c, 'sc, U: 'sc> CanGetChatMemberForUser<'sc, 'c> for U where &'sc U: Into<UserId> {
    fn get_chat_member<O>(&'sc self, other: O) -> GetChatMember<'c> where O: Into<ChatId<'c>> {
        GetChatMember::new(other, self)
    }
}
