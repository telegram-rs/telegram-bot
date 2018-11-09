use requests::*;
use types::*;

/// Use this method to edit only the reply markup of messages sent by the bot.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct EditMessageReplyMarkup {
    chat_id: ChatRef,
    message_id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl Request for EditMessageReplyMarkup {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<Message>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("editMessageReplyMarkup"), self)
    }
}

impl EditMessageReplyMarkup {
    pub fn new<C, M, R>(chat: C, message_id: M, reply_markup: Option<R>) -> Self
    where
        C: ToChatRef,
        M: ToMessageId,
        R: Into<ReplyMarkup>,
    {
        EditMessageReplyMarkup {
            chat_id: chat.to_chat_ref(),
            message_id: message_id.to_message_id(),
            reply_markup: reply_markup.map(|r| r.into()),
        }
    }
}

/// Edit reply markup of messages sent by the bot.
pub trait CanEditMessageReplyMarkup {
    fn edit_reply_markup<R>(&self, reply_markup: Option<R>) -> EditMessageReplyMarkup
    where
        R: Into<ReplyMarkup>;
}

impl<M> CanEditMessageReplyMarkup for M
where
    M: ToMessageId + ToSourceChat,
{
    fn edit_reply_markup<R>(&self, reply_markup: Option<R>) -> EditMessageReplyMarkup
    where
        R: Into<ReplyMarkup>,
    {
        EditMessageReplyMarkup::new(self.to_source_chat(), self.to_message_id(), reply_markup)
    }
}
