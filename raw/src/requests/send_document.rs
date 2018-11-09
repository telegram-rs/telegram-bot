use std::ops::Not;
use std::borrow::Cow;

use types::*;
use requests::*;

/// Use this method to send general files. On success, the sent Message is returned.
/// Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SendDocument<'s, 'c, 'p, 't> {
    chat_id: ChatRef,
    document: Cow<'s, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<Cow<'c, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Not::not")]
    disable_notification: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl<'s, 'c, 'p, 't> Request for SendDocument<'s, 'c, 'p, 't> {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("SendDocument"), self)
    }
}

impl<'s, 'c, 'p, 't> SendDocument<'s, 'c, 'p, 't> {
    pub fn with_url<C, T>(chat: C, url: T) -> Self
    where
        C: ToChatRef,
        T: Into<Cow<'s, str>>,
    {
        Self {
            chat_id: chat.to_chat_ref(),
            document: url.into(),
            caption: None,
            parse_mode: None,
            reply_to_message_id: None,
            reply_markup: None,
            disable_notification: false,
        }
    }

    pub fn caption<T>(&mut self, caption: T) -> &mut Self
    where
        T: Into<Cow<'c, str>>,
    {
        self.caption = Some(caption.into());
        self
    }

    pub fn parse_mode(&mut self, parse_mode: ParseMode) -> &mut Self {
        self.parse_mode = Some(parse_mode);
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

/// Can reply with a document
pub trait CanReplySendDocument {
    fn document_url_reply<'s, 'c, 'p, 't, T>(&self, url: T) -> SendDocument<'s, 'c, 'p, 't>
    where
        T: Into<Cow<'s, str>>;
}

impl<M> CanReplySendDocument for M
where
    M: ToMessageId + ToSourceChat,
{
    fn document_url_reply<'s, 'c, 'p, 't, T>(&self, url: T) -> SendDocument<'s, 'c, 'p, 't>
    where
        T: Into<Cow<'s, str>>,
    {
        let mut req = SendDocument::with_url(self.to_source_chat(), url);
        req.reply_to(self);
        req
    }
}

/// Send an audio
pub trait CanSendDocument {
    fn document_url<'s, 'c, 'p, 't, T>(&self, url: T) -> SendDocument<'s, 'c, 'p, 't>
    where
        T: Into<Cow<'s, str>>;
}

impl<M> CanSendDocument for M
where
    M: ToChatRef,
{
    fn document_url<'s, 'c, 'p, 't, T>(&self, url: T) -> SendDocument<'s, 'c, 'p, 't>
    where
        T: Into<Cow<'s, str>>,
    {
        SendDocument::with_url(self.to_chat_ref(), url)
    }
}
