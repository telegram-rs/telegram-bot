use types::*;
use requests::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct KickChatMember<'c> {
    chat_id: ChatId<'c>,
    user_id: UserId,
}

impl<'c> Request for KickChatMember<'c> {
    type Response = True;
    type RawResponse = True;

    fn map(raw: Self::RawResponse) -> Self::Response {
        raw
    }

    fn name(&self) -> &'static str {
        "kickChatMember"
    }
}

impl<'c> KickChatMember<'c> {
    pub fn new<C, U>(chat: C, user: U) -> Self where C: Into<ChatId<'c>>, U: Into<UserId> {
        KickChatMember {
            chat_id: chat.into(),
            user_id: user.into(),
        }
    }
}

pub trait CanKickChatMemberForChat<'sc, 'c> {
    fn kick_chat_member<O>(&'sc self, other: O) -> KickChatMember<'c> where O: Into<UserId>;
}

impl<'c, 'sc, C: 'sc> CanKickChatMemberForChat<'sc, 'c> for C where &'sc C: Into<ChatId<'c>> {
    fn kick_chat_member<O>(&'sc self, other: O) -> KickChatMember<'c> where O: Into<UserId> {
        KickChatMember::new(self, other)
    }
}

pub trait CanKickChatMemberForUser<'sc, 'c> {
    fn kick_chat_member<O>(&'sc self, other: O) -> KickChatMember<'c> where O: Into<ChatId<'c>>;
}

impl<'c, 'sc, U: 'sc> CanKickChatMemberForUser<'sc, 'c> for U where &'sc U: Into<UserId> {
    fn kick_chat_member<O>(&'sc self, other: O) -> KickChatMember<'c> where O: Into<ChatId<'c>> {
        KickChatMember::new(other, self)
    }
}
