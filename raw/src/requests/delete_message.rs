use requests::*;
use types::*;

// Use this method to delete a message.
// A message can only be deleted if it was sent less than 48 hours ago.
// Any such recently sent outgoing message may be deleted.
// Additionally, if the bot is an administrator in a group chat, it can delete any message.
// If the bot is an administrator in a supergroup, it can delete messages from any
// other user and service messages about people joining or leaving the
// group (other types of service messages may only be removed by the group creator).
// In channels, bots can only remove their own messages.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct DeleteMessage {
    chat_id: ChatRef,
    message_id: MessageId,
}

impl Request for DeleteMessage {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("deleteMessage"), self)
    }
}

impl DeleteMessage {
    pub fn new<C, M>(chat: C, message_id: M) -> Self
    where
        C: ToChatRef,
        M: ToMessageId,
    {
        DeleteMessage {
            chat_id: chat.to_chat_ref(),
            message_id: message_id.to_message_id(),
        }
    }
}

/// Delete messages..
pub trait CanDeleteMessage {
    fn delete(&self) -> DeleteMessage;
}

impl<M> CanDeleteMessage for M
where
    M: ToMessageId + ToSourceChat,
{
    fn delete(&self) -> DeleteMessage {
        DeleteMessage::new(self.to_source_chat(), self.to_message_id())
    }
}
