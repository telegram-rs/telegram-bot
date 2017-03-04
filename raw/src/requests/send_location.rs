use std::ops::Not;

use types::*;
use requests::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct SendLocation<'c> {
    pub chat_id: ChatId<'c>,
    pub latitude: Float,
    pub longitude: Float,
    #[serde(skip_serializing_if = "Not::not")]
    pub disable_notification: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl<'c> Request for SendLocation<'c> {
    type Response = Message;

    fn name(&self) -> &'static str {
        "sendLocation"
    }
}

impl<'c> SendLocation<'c> {
    pub fn new<C>(chat: C, latitude: Float, longitude: Float) -> Self where C: Into<ChatId<'c>> {
        SendLocation {
            chat_id: chat.into(),
            latitude: latitude,
            longitude: longitude,
            disable_notification: false,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn disable_notification(&mut self) -> &mut Self {
        self.disable_notification = true;
        self
    }

    pub fn reply_to<R>(&mut self, to: R) -> &mut Self where R: Into<MessageId> {
        self.reply_to_message_id = Some(to.into().0);
        self
    }

    pub fn reply_markup<R>(&mut self, reply_markup: R) -> &mut Self where R: Into<ReplyMarkup> { // TODO(knsd): nice builder?
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

pub trait CanSendLocation<'bc, 'c> {
    fn location(&'bc self, latitude: Float, longitude: Float) -> SendLocation<'c>;
}

impl<'c, 'bc, C: 'bc> CanSendLocation<'bc, 'c> for C where &'bc C: Into<ChatId<'c>> {
    fn location(&'bc self, latitude: Float, longitude: Float) -> SendLocation<'c> {
        SendLocation::new(self, latitude, longitude)
    }
}

pub trait CanReplySendLocation {
    fn location_reply<'c>(&self, latitude: Float, longitude: Float) -> SendLocation<'c>;
}

impl CanReplySendLocation for Message {
    fn location_reply<'c>(&self, latitude: Float, longitude: Float) -> SendLocation<'c> {
        let mut msg = self.chat.location(latitude, longitude);
        msg.reply_to(self);
        msg
    }
}
