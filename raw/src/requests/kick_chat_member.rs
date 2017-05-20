use types::*;
use requests::*;

/// Use this method to kick a user from a group or a supergroup.
/// In the case of supergroups, the user will not be able to return to the group on
/// their own using invite links, etc., unless unbanned first.
/// The bot must be an administrator in the group for this to work.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct KickChatMember<'c> {
    chat_id: ChatRef<'c>,
    user_id: UserId,
}

impl<'c> Request for KickChatMember<'c> {
    type Response = TrueToUnitResponse;

    fn name(&self) -> &'static str {
        "kickChatMember"
    }
}

impl<'c> KickChatMember<'c> {
    pub fn new<C, U>(chat: C, user: U) -> Self where C: ToChatRef<'c>, U: ToUserId {
        KickChatMember {
            chat_id: chat.to_chat_ref(),
            user_id: user.to_user_id(),
        }
    }
}

/// Kick a user from a group or a supergroup.
pub trait CanKickChatMemberForChat<'c> {
    fn kick<O>(&self, other: O) -> KickChatMember<'c> where O: ToUserId;
}

impl<'c, C> CanKickChatMemberForChat<'c> for C where C: ToChatRef<'c> {
    fn kick<O>(&self, other: O) -> KickChatMember<'c> where O: ToUserId {
        KickChatMember::new(self, other)
    }
}

/// Kick a user from a group or a supergroup.
pub trait CanKickChatMemberForUser<'c> {
    fn kick_from<O>(&self, other: O) -> KickChatMember<'c> where O: ToChatRef<'c>;
}

impl<'c, U> CanKickChatMemberForUser<'c> for U where U: ToUserId {
    fn kick_from<O>(&self, other: O) -> KickChatMember<'c> where O: ToChatRef<'c> {
        KickChatMember::new(other, self)
    }
}
