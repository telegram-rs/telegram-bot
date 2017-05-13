use std::ops::Not;

use types::*;
use requests::*;

/// Use this method to forward messages of any kind.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct ForwardMessage<'c, 'f> {
    chat_id: ChatRef<'c>,
    from_chat_id: ChatRef<'f>,
    #[serde(skip_serializing_if = "Not::not")]
    disable_notification: bool,
    message_id: MessageId,
}

impl<'c, 'f> Request for ForwardMessage<'c, 'f> {
    type Response = Message;
    type RawResponse = Message;

    fn map(raw: Self::RawResponse) -> Self::Response {
        raw
    }

    fn name() -> &'static str {
        "forwardMessage"
    }
}

impl<'c, 'f> ForwardMessage<'c, 'f> {
    pub fn new<M, F, T>(message: M, from: F, to: T) -> Self
        where M: ToMessageId, F: ToChatRef<'f>, T: ToChatRef<'c> {

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

pub trait CanForwardMessage {
    fn forward<'c, 'f, T>(&self, to: T) -> ForwardMessage<'c, 'f> where T: ToChatRef<'c>;
}

impl<M> CanForwardMessage for M where M: ToMessageId + ToSourceChat {
    fn forward<'c, 'f, T>(&self, to: T) -> ForwardMessage<'c, 'f> where T: ToChatRef<'c> {
        ForwardMessage::new(self.to_message_id(), self.to_source_chat(), to)
    }
}
