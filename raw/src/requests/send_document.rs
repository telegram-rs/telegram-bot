use std::borrow::Cow;

use crate::requests::*;
use crate::types::*;

/// Use this method to send general files. On success, the sent Message is returned.
/// Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[must_use = "requests do nothing unless sent"]
pub struct SendDocument<'c> {
    chat_id: ChatRef,
    document: InputFile,
    thumb: Option<InputFile>,
    caption: Option<Cow<'c, str>>,
    parse_mode: Option<ParseMode>,
    reply_to_message_id: Option<MessageId>,
    disable_notification: bool,
    reply_markup: Option<ReplyMarkup>,
}

impl<'c> ToMultipart for SendDocument<'c> {
    fn to_multipart(&self) -> Result<Multipart, Error> {
        multipart_map! {
            self,
            (chat_id (text));
            (document (raw));
            (thumb (raw), optional);
            (caption (text), optional);
            (parse_mode (text), optional);
            (reply_to_message_id (text), optional);
            (disable_notification (text), when_true);
            (reply_markup (json), optional);
        }
    }
}

impl<'c> Request for SendDocument<'c> {
    type Type = MultipartRequestType<Self>;
    type Response = JsonIdResponse<Message>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("sendDocument"), self)
    }
}

impl<'c> SendDocument<'c> {
    pub fn new<C, V>(chat: C, document: V) -> Self
    where
        C: ToChatRef,
        V: Into<InputFile>,
    {
        Self {
            chat_id: chat.to_chat_ref(),
            document: document.into(),
            thumb: None,
            caption: None,
            parse_mode: None,
            reply_to_message_id: None,
            reply_markup: None,
            disable_notification: false,
        }
    }

    pub fn thumb<V>(&mut self, thumb: V) -> &mut Self
    where
        V: Into<InputFileUpload>,
    {
        self.thumb = Some(thumb.into().into());
        self
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

/// Can reply with a document
pub trait CanReplySendDocument {
    fn document_reply<'c, T>(&self, document: T) -> SendDocument<'c>
    where
        T: Into<InputFile>;
}

impl<M> CanReplySendDocument for M
where
    M: ToMessageId + ToSourceChat,
{
    fn document_reply<'c, T>(&self, document: T) -> SendDocument<'c>
    where
        T: Into<InputFile>,
    {
        let mut req = SendDocument::new(self.to_source_chat(), document);
        req.reply_to(self);
        req
    }
}

/// Send a document
pub trait CanSendDocument {
    fn document<'c, T>(&self, document: T) -> SendDocument<'c>
    where
        T: Into<InputFile>;
}

impl<M> CanSendDocument for M
where
    M: ToChatRef,
{
    fn document<'c, T>(&self, document: T) -> SendDocument<'c>
    where
        T: Into<InputFile>,
    {
        SendDocument::new(self.to_chat_ref(), document)
    }
}
