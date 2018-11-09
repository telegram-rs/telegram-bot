use requests::*;
use types::*;

/// Use this method to get information about a member of a chat.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetChatMember {
    chat_id: ChatRef,
    user_id: UserId,
}

impl Request for GetChatMember {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<ChatMember>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("getChatMember"), self)
    }
}

impl GetChatMember {
    pub fn new<C, U>(chat: C, user: U) -> Self
    where
        C: ToChatRef,
        U: ToUserId,
    {
        GetChatMember {
            chat_id: chat.to_chat_ref(),
            user_id: user.to_user_id(),
        }
    }
}

/// Get information about a member of a chat.
pub trait CanGetChatMemberForChat {
    fn get_member<O>(&self, other: O) -> GetChatMember
    where
        O: ToUserId;
}

impl<C> CanGetChatMemberForChat for C
where
    C: ToChatRef,
{
    fn get_member<O>(&self, other: O) -> GetChatMember
    where
        O: ToUserId,
    {
        GetChatMember::new(self, other)
    }
}

/// Get information about a member of a chat.
pub trait CanGetChatMemberForUser {
    fn get_member_from<O>(&self, other: O) -> GetChatMember
    where
        O: ToChatRef;
}

impl<U> CanGetChatMemberForUser for U
where
    U: ToUserId,
{
    fn get_member_from<O>(&self, other: O) -> GetChatMember
    where
        O: ToChatRef,
    {
        GetChatMember::new(other, self)
    }
}
