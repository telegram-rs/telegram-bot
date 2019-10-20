use std::borrow::Cow;

use crate::requests::*;
use crate::types::*;

/// Use this method to send an video
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[must_use = "requests do nothing unless sent"]
pub struct SendVideo<'c> {
    chat_id: ChatRef,
    video: InputFile,
    caption: Option<Cow<'c, str>>,
    parse_mode: Option<ParseMode>,
    duration: Option<Integer>,
    width: Option<Integer>,
    height: Option<Integer>,
    supports_streaming: bool,
    thumb: Option<InputFile>,
    reply_to_message_id: Option<MessageId>,
    disable_notification: bool,
    reply_markup: Option<ReplyMarkup>,
}

impl<'c> ToMultipart for SendVideo<'c> {
    fn to_multipart(&self) -> Result<Multipart, Error> {
        multipart_map! {
            self,
            (chat_id (text));
            (video (raw));
            (caption (text), optional);
            (parse_mode (text), optional);
            (duration (text), optional);
            (width (text), optional);
            (height (text), optional);
            (supports_streaming (text), when_true);
            (thumb (raw), optional);
            (reply_to_message_id (text), optional);
            (disable_notification (text), when_true);
            (reply_markup (json), optional);
        }
    }
}

impl<'c> Request for SendVideo<'c> {
    type Type = MultipartRequestType<Self>;
    type Response = JsonIdResponse<Message>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("sendVideo"), self)
    }
}

impl<'c> SendVideo<'c> {
    pub fn new<C, V>(chat: C, video: V) -> Self
    where
        C: ToChatRef,
        V: Into<InputFile>,
    {
        Self {
            chat_id: chat.to_chat_ref(),
            video: video.into(),
            caption: None,
            parse_mode: None,
            duration: None,
            width: None,
            height: None,
            supports_streaming: false,
            thumb: None,
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

    pub fn duration(&mut self, duration: Integer) -> &mut Self {
        self.duration = Some(duration);
        self
    }

    pub fn width(&mut self, width: Integer) -> &mut Self {
        self.width = Some(width);
        self
    }

    pub fn height(&mut self, height: Integer) -> &mut Self {
        self.height = Some(height);
        self
    }

    pub fn supports_streaming(&mut self) -> &mut Self {
        self.supports_streaming = true;
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

    pub fn disable_notification(&mut self) -> &mut Self {
        self.disable_notification = true;
        self
    }
}

/// Can reply with an video
pub trait CanReplySendVideo {
    fn video_reply<'c, T>(&self, video: T) -> SendVideo<'c>
    where
        T: Into<InputFile>;
}

impl<M> CanReplySendVideo for M
where
    M: ToMessageId + ToSourceChat,
{
    fn video_reply<'c, T>(&self, video: T) -> SendVideo<'c>
    where
        T: Into<InputFile>,
    {
        let mut req = SendVideo::new(self.to_source_chat(), video);
        req.reply_to(self);
        req
    }
}

/// Send an video
pub trait CanSendVideo {
    fn video<'c, T>(&self, video: T) -> SendVideo<'c>
    where
        T: Into<InputFile>;
}

impl<M> CanSendVideo for M
where
    M: ToChatRef,
{
    fn video<'c, T>(&self, video: T) -> SendVideo<'c>
    where
        T: Into<InputFile>,
    {
        SendVideo::new(self.to_chat_ref(), video)
    }
}
