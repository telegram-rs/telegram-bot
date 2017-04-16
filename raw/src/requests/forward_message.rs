use std::ops::Not;

use types::*;
use requests::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct ForwardMessage<'c, 'f> {
    chat_id: ChatId<'c>,
    from_chat_id: ChatId<'f>,
    #[serde(skip_serializing_if = "Not::not")]
    disable_notification: bool,
    message_id: Integer,
}

impl<'c, 'f> Request for ForwardMessage<'c, 'f> {
    type Response = Message;

    fn name(&self) -> &'static str {
        "forwardMessage"
    }
}

impl<'c, 'f> ForwardMessage<'c, 'f> {
    pub fn new<M, F, T>(message: M, from: F, to: T) -> Self
        where M: Into<MessageId>, F: Into<ChatId<'f>>, T: Into<ChatId<'c>> {

        ForwardMessage {
            chat_id: to.into(),
            from_chat_id: from.into(),
            disable_notification: false,
            message_id: message.into().0,
        }
    }

    pub fn disable_notification(mut self) -> Self {
        self.disable_notification = true;
        self
    }
}

pub trait CanForwardMessage {
    fn forward<'c, 'f, T>(&self, to: T) -> ForwardMessage<'c, 'f> where T: Into<ChatId<'c>>;
}

impl CanForwardMessage for Message {
    fn forward<'c, 'f, T>(&self, to: T) -> ForwardMessage<'c, 'f> where T: Into<ChatId<'c>> {
        ForwardMessage::new(self, &self.chat, to)
    }
}
