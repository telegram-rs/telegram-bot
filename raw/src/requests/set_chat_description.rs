use std::borrow::Cow;

use crate::requests::*;
use crate::types::*;

/// Use this method to set group descriptions.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetChatDescription<'s> {
    chat_id: ChatRef,
    description: Cow<'s, str>,
}

impl<'c, 's> Request for SetChatDescription<'s> {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<bool>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("setChatDescription"), self)
    }
}

impl<'s> SetChatDescription<'s> {
    pub fn new<C, D>(chat: C, description: D) -> Self
    where
        C: ToChatRef,
        D: Into<Cow<'s, str>>,
    {
        SetChatDescription {
            chat_id: chat.to_chat_ref(),
            description: description.into(),
        }
    }
}
