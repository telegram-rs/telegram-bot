use requests::*;
use types::*;

/// Use this method to stop updating a live location message sent by the bot
/// before live_period expires.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct StopMessageLiveLocation {
    chat_id: ChatRef,
    message_id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl Request for StopMessageLiveLocation {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<Message>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("stopMessageLiveLocation"), self)
    }
}

impl StopMessageLiveLocation {
    pub fn new<C, M>(chat: C, message_id: M) -> Self
    where
        C: ToChatRef,
        M: ToMessageId,
    {
        StopMessageLiveLocation {
            chat_id: chat.to_chat_ref(),
            message_id: message_id.to_message_id(),
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

/// Stop updating a live location message sent by the bot.
pub trait CanStopMessageLiveLocation {
    fn stop_live_location(&self) -> StopMessageLiveLocation;
}

impl<M> CanStopMessageLiveLocation for M
where
    M: ToMessageId + ToSourceChat,
{
    fn stop_live_location(&self) -> StopMessageLiveLocation {
        StopMessageLiveLocation::new(self.to_source_chat(), self.to_message_id())
    }
}
