use crate::requests::*;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[must_use = "requests do nothing unless sent"]
pub struct SendSticker {
    chat_id: ChatRef,
    sticker: InputFile,
    disable_notification: bool,
    reply_to_message_id: Option<MessageId>,
    reply_markup: Option<ReplyMarkup>,
}

impl ToMultipart for SendSticker {
    fn to_multipart(&self) -> Result<Multipart, Error> {
        multipart_map! {
            self,
            (chat_id (text));
            (sticker (raw));
            (disable_notification (text), when_true);
            (reply_to_message_id (text), optional);
            (reply_markup (json), optional);
        }
    }
}

impl Request for SendSticker {
    type Type = MultipartRequestType<Self>;
    type Response = JsonIdResponse<Message>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("sendSticker"), self)
    }
}

impl SendSticker {
    pub fn new<C, V>(chat: C, photo: V) -> Self
        where
            C: ToChatRef,
            V: Into<InputFile>,
    {
        Self {
            chat_id: chat.to_chat_ref(),
            sticker: photo.into(),
            reply_to_message_id: None,
            reply_markup: None,
            disable_notification: false,
        }
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
pub trait CanReplySendSticker {
    fn sticker_reply<T>(&self, photo: T) -> SendSticker
        where
            T: Into<InputFile>;
}

impl<M> CanReplySendSticker for M
    where
        M: ToMessageId + ToSourceChat,
{
    fn sticker_reply<T>(&self, photo: T) -> SendSticker
        where
            T: Into<InputFile>,
    {
        let mut req = SendSticker::new(self.to_source_chat(), photo);
        req.reply_to(self);
        req
    }
}

/// Send an photo
pub trait CanSendSticker {
    fn sticker<T>(&self, photo: T) -> SendSticker
        where
            T: Into<InputFile>;
}

impl<M> CanSendSticker for M
    where
        M: ToChatRef,
{
    fn sticker<T>(&self, photo: T) -> SendSticker
        where
            T: Into<InputFile>,
    {
        SendSticker::new(self.to_chat_ref(), photo)
    }
}
