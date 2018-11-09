use std::borrow::Cow;

use requests::*;
use types::*;

/// Use this method to edit captions of messages sent by the bot.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct EditMessageCaption<'s> {
    chat_id: ChatRef,
    message_id: MessageId,
    caption: Cow<'s, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl<'s> Request for EditMessageCaption<'s> {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<Message>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("editMessageCaption"), self)
    }
}

impl<'s> EditMessageCaption<'s> {
    pub fn new<C, M, T>(chat: C, message_id: M, caption: T) -> Self
    where
        C: ToChatRef,
        M: ToMessageId,
        T: Into<Cow<'s, str>>,
    {
        EditMessageCaption {
            chat_id: chat.to_chat_ref(),
            message_id: message_id.to_message_id(),
            caption: caption.into(),
            reply_markup: None,
        }
    }

    pub fn reply_markup<R>(&mut self, reply_markup: R) -> &mut Self
    where
        R: Into<ReplyMarkup>,
    {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

/// Edit captions of messages sent by the bot.
pub trait CanEditMessageCaption {
    fn edit_caption<'s, T>(&self, caption: T) -> EditMessageCaption<'s>
    where
        T: Into<Cow<'s, str>>;
}

impl<M> CanEditMessageCaption for M
where
    M: ToMessageId + ToSourceChat,
{
    fn edit_caption<'s, T>(&self, caption: T) -> EditMessageCaption<'s>
    where
        T: Into<Cow<'s, str>>,
    {
        EditMessageCaption::new(self.to_source_chat(), self.to_message_id(), caption)
    }
}
