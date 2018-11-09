use requests::*;
use types::*;

/// Use this method to get the number of members in a chat.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetChatMembersCount {
    chat_id: ChatRef,
}

impl Request for GetChatMembersCount {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<Integer>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("getChatMembersCount"), self)
    }
}

impl GetChatMembersCount {
    pub fn new<C>(chat: C) -> Self
    where
        C: ToChatRef,
    {
        GetChatMembersCount {
            chat_id: chat.to_chat_ref(),
        }
    }
}

/// Get the number of members in a chat.
pub trait CanGetChatMembersCount {
    fn get_members_count(&self) -> GetChatMembersCount;
}

impl<C> CanGetChatMembersCount for C
where
    C: ToChatRef,
{
    fn get_members_count(&self) -> GetChatMembersCount {
        GetChatMembersCount::new(self)
    }
}
