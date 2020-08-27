use crate::requests::*;
use crate::types::*;

/// Use this method to stop a poll which was sent by the bot.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct StopPoll {
    chat_id: ChatRef,
    message_id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl Request for StopPoll {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<Poll>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("stopPoll"), self)
    }
}

impl StopPoll {
    pub fn new<C, M>(chat: C, message: M) -> Self
    where
        C: ToChatRef,
        M: ToMessageId,
    {
        StopPoll {
            chat_id: chat.to_chat_ref(),
            message_id: message.to_message_id(),
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

/// Stop a poll which was sent by the bot.
pub trait CanStopPoll {
    fn stop_poll(&self) -> StopPoll;
}

impl<M> CanStopPoll for M
where
    M: ToMessageId + ToSourceChat,
{
    fn stop_poll(&self) -> StopPoll {
        StopPoll::new(self.to_source_chat(), self.to_message_id())
    }
}
