use types::*;
use requests::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct UnbanChatMember<'c> {
    pub chat_id: ChatId<'c>,
    pub user_id: UserId,
}

impl<'c> Request for UnbanChatMember<'c> {
    type Response = True;

    fn name(&self) -> &'static str {
        "unbanChatMember"
    }
}

impl<'c> UnbanChatMember<'c> {
    pub fn new<C, U>(chat: C, user: U) -> Self where C: Into<ChatId<'c>>, U: Into<UserId> {
        UnbanChatMember {
            chat_id: chat.into(),
            user_id: user.into(),
        }
    }
}

pub trait CanUnbanChatMemberForChat<'sc, 'c> {
    fn unban_chat_member<O>(&'sc self, other: O) -> UnbanChatMember<'c> where O: Into<UserId>;
}

impl<'c, 'sc, C: 'sc> CanUnbanChatMemberForChat<'sc, 'c> for C where &'sc C: Into<ChatId<'c>> {
    fn unban_chat_member<O>(&'sc self, other: O) -> UnbanChatMember<'c> where O: Into<UserId> {
        UnbanChatMember::new(self, other)
    }
}

pub trait CanUnbanChatMemberForUser<'sc, 'c> {
    fn unban_chat_member<O>(&'sc self, other: O) -> UnbanChatMember<'c> where O: Into<ChatId<'c>>;
}

impl<'c, 'sc, U: 'sc> CanUnbanChatMemberForUser<'sc, 'c> for U where &'sc U: Into<UserId> {
    fn unban_chat_member<O>(&'sc self, other: O) -> UnbanChatMember<'c> where O: Into<ChatId<'c>> {
        UnbanChatMember::new(other, self)
    }
}
