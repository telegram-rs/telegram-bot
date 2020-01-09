use std::borrow::Cow;
use std::ops::Not;

use crate::requests::*;
use crate::types::*;

/// Use this method to send text messages.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SendMessage<'s> {
    chat_id: ChatRef,
    text: Cow<'s, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Not::not")]
    disable_web_page_preview: bool,
    #[serde(skip_serializing_if = "Not::not")]
    disable_notification: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl<'c, 's> Request for SendMessage<'s> {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<MessageOrChannelPost>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("sendMessage"), self)
    }
}

impl<'s> SendMessage<'s> {
    pub fn new<C, T>(chat: C, text: T) -> Self
    where
        C: ToChatRef,
        T: Into<Cow<'s, str>>,
    {
        SendMessage {
            chat_id: chat.to_chat_ref(),
            text: text.into(),
            parse_mode: None,
            disable_web_page_preview: false,
            disable_notification: false,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn parse_mode(&mut self, parse_mode: ParseMode) -> &mut Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    pub fn disable_preview(&mut self) -> &mut Self {
        self.disable_web_page_preview = true;
        self
    }

    pub fn disable_notification(&mut self) -> &mut Self {
        self.disable_notification = true;
        self
    }

    pub fn reply_to<R>(&mut self, to: R) -> &mut Self
    where
        R: ToMessageId,
    {
        self.reply_to_message_id = Some(to.to_message_id());
        self
    }

    pub fn reply_markup<R>(&mut self, reply_markup: R) -> &mut Self
    where
        R: Into<ReplyMarkup>,
    {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

/// Send text message.
pub trait CanSendMessage {
    fn text<'s, T>(&self, text: T) -> SendMessage<'s>
    where
        T: Into<Cow<'s, str>>;
}

impl<C> CanSendMessage for C
where
    C: ToChatRef,
{
    fn text<'s, T>(&self, text: T) -> SendMessage<'s>
    where
        T: Into<Cow<'s, str>>,
    {
        SendMessage::new(self, text)
    }
}

/// Reply with text message.
pub trait CanReplySendMessage {
    fn text_reply<'c, 's, T>(&self, text: T) -> SendMessage<'s>
    where
        T: Into<Cow<'s, str>>;
}

impl<M> CanReplySendMessage for M
where
    M: ToMessageId + ToSourceChat,
{
    fn text_reply<'c, 's, T>(&self, text: T) -> SendMessage<'s>
    where
        T: Into<Cow<'s, str>>,
    {
        let mut rq = self.to_source_chat().text(text);
        rq.reply_to(self.to_message_id());
        rq
    }
}
