use requests::*;
use types::*;

///Use this method to unpin a message in a supergroup or a channel.
/// The bot must be an administrator in the chat for this to work
/// and must have the ‘can_pin_messages’ admin right in the
/// supergroup or ‘can_edit_messages’ admin right in the channel.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct UnpinChatMessage {
    chat_id: ChatRef,
}

impl Request for UnpinChatMessage {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("unpinChatMessage"), self)
    }
}

impl UnpinChatMessage {
    fn new<C>(chat: C) -> Self
    where
        C: ToChatRef,
    {
        Self {
            chat_id: chat.to_chat_ref(),
        }
    }
}

pub trait CanUnpinMessage {
    fn unpin_message(&self) -> UnpinChatMessage;
}

impl<C> CanUnpinMessage for C
where
    C: ToChatRef,
{
    fn unpin_message(&self) -> UnpinChatMessage {
        UnpinChatMessage::new(self)
    }
}
