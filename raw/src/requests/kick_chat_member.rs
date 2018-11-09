use requests::*;
use types::*;

/// Use this method to kick a user from a group or a supergroup.
/// In the case of supergroups, the user will not be able to return to the group on
/// their own using invite links, etc., unless unbanned first.
/// The bot must be an administrator in the group for this to work.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct KickChatMember {
    chat_id: ChatRef,
    user_id: UserId,
}

impl Request for KickChatMember {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("kickChatMember"), self)
    }
}

impl KickChatMember {
    pub fn new<C, U>(chat: C, user: U) -> Self
    where
        C: ToChatRef,
        U: ToUserId,
    {
        KickChatMember {
            chat_id: chat.to_chat_ref(),
            user_id: user.to_user_id(),
        }
    }
}

/// Kick a user from a group or a supergroup.
pub trait CanKickChatMemberForChat {
    fn kick<O>(&self, other: O) -> KickChatMember
    where
        O: ToUserId;
}

impl<C> CanKickChatMemberForChat for C
where
    C: ToChatRef,
{
    fn kick<O>(&self, other: O) -> KickChatMember
    where
        O: ToUserId,
    {
        KickChatMember::new(self, other)
    }
}

/// Kick a user from a group or a supergroup.
pub trait CanKickChatMemberForUser {
    fn kick_from<O>(&self, other: O) -> KickChatMember
    where
        O: ToChatRef;
}

impl<U> CanKickChatMemberForUser for U
where
    U: ToUserId,
{
    fn kick_from<O>(&self, other: O) -> KickChatMember
    where
        O: ToChatRef,
    {
        KickChatMember::new(other, self)
    }
}
