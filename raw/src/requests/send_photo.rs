use std::borrow::Cow;

use crate::requests::*;
use crate::types::*;

/// Use this method to send photos
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[must_use = "requests do nothing unless sent"]
pub struct SendPhoto<'c> {
    chat_id: ChatRef,
    photo: InputFile,
    caption: Option<Cow<'c, str>>,
    parse_mode: Option<ParseMode>,
    reply_to_message_id: Option<MessageId>,
    disable_notification: bool,
    reply_markup: Option<ReplyMarkup>,
}

impl<'c> ToMultipart for SendPhoto<'c> {
    fn to_multipart(&self) -> Result<Multipart, Error> {
        multipart_map! {
            self,
            (chat_id (text));
            (photo (raw));
            (caption (text), optional);
            (parse_mode (text), optional);
            (reply_to_message_id (text), optional);
            (disable_notification (text), when_true);
            (reply_markup (json), optional);
        }
    }
}

impl<'c> Request for SendPhoto<'c> {
    type Type = MultipartRequestType<Self>;
    type Response = JsonIdResponse<Message>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("sendPhoto"), self)
    }
}

impl<'c> SendPhoto<'c> {
    pub fn new<C, V>(chat: C, photo: V) -> Self
    where
        C: ToChatRef,
        V: Into<InputFile>,
    {
        Self {
            chat_id: chat.to_chat_ref(),
            photo: photo.into(),
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

    pub fn disable_notification(&mut self) -> &mut Self {
        self.disable_notification = true;
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

/// Can reply with an photo
pub trait CanReplySendPhoto {
    fn photo_reply<'c, T>(&self, photo: T) -> SendPhoto<'c>
    where
        T: Into<InputFile>;
}

impl<M> CanReplySendPhoto for M
where
    M: ToMessageId + ToSourceChat,
{
    fn photo_reply<'c, T>(&self, photo: T) -> SendPhoto<'c>
    where
        T: Into<InputFile>,
    {
        let mut req = SendPhoto::new(self.to_source_chat(), photo);
        req.reply_to(self);
        req
    }
}

/// Send an photo
pub trait CanSendPhoto {
    fn photo<'c, T>(&self, photo: T) -> SendPhoto<'c>
    where
        T: Into<InputFile>;
}

impl<M> CanSendPhoto for M
where
    M: ToChatRef,
{
    fn photo<'c, T>(&self, photo: T) -> SendPhoto<'c>
    where
        T: Into<InputFile>,
    {
        SendPhoto::new(self.to_chat_ref(), photo)
    }
}
