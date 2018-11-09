use std::ops::Not;

use requests::*;
use types::*;

/// Use this method to forward messages of any kind.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct ForwardMessage {
    chat_id: ChatRef,
    from_chat_id: ChatRef,
    #[serde(skip_serializing_if = "Not::not")]
    disable_notification: bool,
    message_id: MessageId,
}

impl Request for ForwardMessage {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<Message>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("forwardMessage"), self)
    }
}

impl ForwardMessage {
    pub fn new<M, F, T>(message: M, from: F, to: T) -> Self
    where
        M: ToMessageId,
        F: ToChatRef,
        T: ToChatRef,
    {
        ForwardMessage {
            chat_id: to.to_chat_ref(),
            from_chat_id: from.to_chat_ref(),
            disable_notification: false,
            message_id: message.to_message_id(),
        }
    }

    pub fn disable_notification(&mut self) -> &mut Self {
        self.disable_notification = true;
        self
    }
}

/// Forward message.
pub trait CanForwardMessage {
    fn forward<T>(&self, to: T) -> ForwardMessage
    where
        T: ToChatRef;
}

impl<M> CanForwardMessage for M
where
    M: ToMessageId + ToSourceChat,
{
    fn forward<T>(&self, to: T) -> ForwardMessage
    where
        T: ToChatRef,
    {
        ForwardMessage::new(self.to_message_id(), self.to_source_chat(), to)
    }
}
