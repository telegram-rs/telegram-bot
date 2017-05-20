use std::ops::Not;

use types::*;
use requests::*;

/// Use this method to send point on the map.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SendLocation<'c> {
    chat_id: ChatRef<'c>,
    latitude: Float,
    longitude: Float,
    #[serde(skip_serializing_if = "Not::not")]
    disable_notification: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl<'c> Request for SendLocation<'c> {
    type Response = IdResponse<Message>;

    fn name(&self) -> &'static str {
        "sendLocation"
    }
}

impl<'c> SendLocation<'c> {
    pub fn new<C>(chat: C, latitude: Float, longitude: Float) -> Self where C: ToChatRef<'c> {
        SendLocation {
            chat_id: chat.to_chat_ref(),
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

    pub fn reply_to<R>(&mut self, to: R) -> &mut Self where R: ToMessageId {
        self.reply_to_message_id = Some(to.to_message_id());
        self
    }

    pub fn reply_markup<R>(&mut self, reply_markup: R) -> &mut Self where R: Into<ReplyMarkup> { // TODO(knsd): nice builder?
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

/// Send point on the map.
pub trait CanSendLocation<'c> {
    fn location(&self, latitude: Float, longitude: Float) -> SendLocation<'c>;
}

impl<'c, C> CanSendLocation<'c> for C where C: ToChatRef<'c> {
    fn location(&self, latitude: Float, longitude: Float) -> SendLocation<'c> {
        SendLocation::new(self, latitude, longitude)
    }
}

/// Reply with point on the map.
pub trait CanReplySendLocation {
    fn location_reply<'c>(&self, latitude: Float, longitude: Float) -> SendLocation<'c>;
}

impl<M> CanReplySendLocation for M where M: ToMessageId + ToSourceChat {
    fn location_reply<'c>(&self, latitude: Float, longitude: Float) -> SendLocation<'c> {
        let mut rq = self.to_source_chat().location(latitude, longitude);
        rq.reply_to(self.to_message_id());
        rq
    }
}

impl<'b, 'c> ToRequest<'b, 'c> for Location {
    type Request = SendLocation<'c>;

    fn to_request<C>(&'b self, chat: C) -> Self::Request where C: ToChatRef<'c> {
        chat.location(self.latitude, self.longitude)
    }
}

impl<'b, 'c> ToReplyRequest<'b, 'c> for Location {
    type Request = SendLocation<'c>;

    fn to_reply_request<M>(&'b self, message: M) -> Self::Request
        where M: ToMessageId + ToSourceChat {

        message.location_reply(self.latitude, self.longitude)
    }
}
