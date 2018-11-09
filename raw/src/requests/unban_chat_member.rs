use requests::*;
use types::*;

/// Use this method to unban a previously kicked user in a supergroup or channel.
/// The user will not return to the group or channel automatically, but will be able to
/// join via link, etc. The bot must be an administrator in the group for this to work.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct UnbanChatMember {
    chat_id: ChatRef,
    user_id: UserId,
}

impl Request for UnbanChatMember {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("unbanChatMember"), self)
    }
}

impl UnbanChatMember {
    pub fn new<C, U>(chat: C, user: U) -> Self
    where
        C: ToChatRef,
        U: ToUserId,
    {
        UnbanChatMember {
            chat_id: chat.to_chat_ref(),
            user_id: user.to_user_id(),
        }
    }
}

/// Unban a previously kicked user in a supergroup or channel.
pub trait CanUnbanChatMemberForChat {
    fn unban<O>(&self, other: O) -> UnbanChatMember
    where
        O: ToUserId;
}

impl<C> CanUnbanChatMemberForChat for C
where
    C: ToChatRef,
{
    fn unban<O>(&self, other: O) -> UnbanChatMember
    where
        O: ToUserId,
    {
        UnbanChatMember::new(self, other)
    }
}

/// Unban a previously kicked user in a supergroup or channel.
pub trait CanUnbanChatMemberForUser {
    fn unban_in<O>(&self, other: O) -> UnbanChatMember
    where
        O: ToChatRef;
}

impl<U> CanUnbanChatMemberForUser for U
where
    U: ToUserId,
{
    fn unban_in<O>(&self, other: O) -> UnbanChatMember
    where
        O: ToChatRef,
    {
        UnbanChatMember::new(other, self)
    }
}
