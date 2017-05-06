use types::*;
use requests::*;

/// Use this method to unban a previously kicked user in a supergroup.
/// The user will not return to the group automatically, but will be able to
/// join via link, etc. The bot must be an administrator in the group for this to work.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct UnbanChatMember<'c> {
    chat_id: ChatRef<'c>,
    user_id: UserId,
}

impl<'c> Request for UnbanChatMember<'c> {
    type Response = ();
    type RawResponse = True;

    fn map(_raw: Self::RawResponse) -> Self::Response {
        ()
    }

    fn name() -> &'static str {
        "unbanChatMember"
    }
}

impl<'c> UnbanChatMember<'c> {
    pub fn new<C, U>(chat: C, user: U) -> Self where C: ToChatRef<'c>, U: ToUserId {
        UnbanChatMember {
            chat_id: chat.to_chat_ref(),
            user_id: user.to_user_id(),
        }
    }
}

pub trait CanUnbanChatMemberForChat<'c> {
    fn unban_chat_member<O>(&self, other: O) -> UnbanChatMember<'c> where O: ToUserId;
}

impl<'c, C> CanUnbanChatMemberForChat<'c> for C where C: ToChatRef<'c> {
    fn unban_chat_member<O>(&self, other: O) -> UnbanChatMember<'c> where O: ToUserId {
        UnbanChatMember::new(self, other)
    }
}

pub trait CanUnbanChatMemberForUser<'c> {
    fn unban_chat_member<O>(&self, other: O) -> UnbanChatMember<'c> where O: ToChatRef<'c>;
}

impl<'c, U> CanUnbanChatMemberForUser<'c> for U where U: ToUserId {
    fn unban_chat_member<O>(&self, other: O) -> UnbanChatMember<'c> where O: ToChatRef<'c> {
        UnbanChatMember::new(other, self)
    }
}
