use types::*;
use requests::*;

/// Use this method to unban a previously kicked user in a supergroup.
/// The user will not return to the group automatically, but will be able to
/// join via link, etc. The bot must be an administrator in the group for this to work.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct UnbanChatMember<'c> {
    chat_id: ChatRef<'c>,
    user_id: UserId,
}

impl<'c> Request for UnbanChatMember<'c> {
    type Response = TrueToUnitResponse;

    fn name(&self) -> &'static str {
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

/// Unban a previously kicked user in a supergroup.
pub trait CanUnbanChatMemberForChat<'c> {
    fn unban<O>(&self, other: O) -> UnbanChatMember<'c> where O: ToUserId;
}

impl<'c, C> CanUnbanChatMemberForChat<'c> for C where C: ToChatRef<'c> {
    fn unban<O>(&self, other: O) -> UnbanChatMember<'c> where O: ToUserId {
        UnbanChatMember::new(self, other)
    }
}

/// Unban a previously kicked user in a supergroup.
pub trait CanUnbanChatMemberForUser<'c> {
    fn unban_in<O>(&self, other: O) -> UnbanChatMember<'c> where O: ToChatRef<'c>;
}

impl<'c, U> CanUnbanChatMemberForUser<'c> for U where U: ToUserId {
    fn unban_in<O>(&self, other: O) -> UnbanChatMember<'c> where O: ToChatRef<'c> {
        UnbanChatMember::new(other, self)
    }
}
