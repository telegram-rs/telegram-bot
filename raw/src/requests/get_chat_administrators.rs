use requests::*;
use types::*;

/// Use this method to get a list of administrators in a chat.
/// If the chat is a group or a supergroup and no administrators were appointed,
/// only the creator will be returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetChatAdministrators {
    chat_id: ChatRef,
}

impl Request for GetChatAdministrators {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<Vec<ChatMember>>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("getChatAdministrators"), self)
    }
}

impl GetChatAdministrators {
    pub fn new<C>(chat: C) -> Self
    where
        C: ToChatRef,
    {
        GetChatAdministrators {
            chat_id: chat.to_chat_ref(),
        }
    }
}

/// Get a list of administrators in a chat.
pub trait CanGetChatAdministrators {
    fn get_administrators(&self) -> GetChatAdministrators;
}

impl<C> CanGetChatAdministrators for C
where
    C: ToChatRef,
{
    fn get_administrators(&self) -> GetChatAdministrators {
        GetChatAdministrators::new(self)
    }
}
