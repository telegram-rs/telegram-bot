use std::borrow::Cow;
use std::clone::Clone;
use std::ops::Not;

use crate::requests::*;
use crate::types::*;

/// Use this method to send polls.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SendPoll<'q, 'o, 'e> {
    chat_id: ChatRef,
    question: Cow<'q, str>,
    options: Vec<Cow<'o, str>>,
    #[serde(skip_serializing_if = "Clone::clone")]
    // This defaults to true, so don't skip serializing if false.
    is_anonymous: bool,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PollType>,
    #[serde(skip_serializing_if = "Not::not")]
    allows_multiple_answers: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    // TODO: required for quiz polls
    correct_option_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation: Option<Cow<'e, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation_parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_period: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    close_date: Option<Integer>,
    #[serde(skip_serializing_if = "Not::not")]
    is_closed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl<'q, 'o, 'e> Request for SendPoll<'q, 'o, 'e> {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<MessageOrChannelPost>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("sendPoll"), self)
    }
}

impl<'q, 'o, 'e> SendPoll<'q, 'o, 'e> {
    // TODO: allow creating requests only with at least 2 options
    pub fn new<C, Q, O>(chat: C, question: Q, options: Vec<O>) -> Self
    where
        C: ToChatRef,
        Q: Into<Cow<'q, str>>,
        O: Into<Cow<'o, str>>,
    {
        let mut req_options: Vec<Cow<'o, str>> = Vec::new();

        for option in options {
            req_options.push(option.into());
        }

        SendPoll {
            chat_id: chat.to_chat_ref(),
            question: question.into(),
            options: req_options,
            is_anonymous: true,
            type_: None,
            allows_multiple_answers: false,
            correct_option_id: None,
            explanation: None,
            explanation_parse_mode: None,
            open_period: None,
            close_date: None,
            is_closed: false,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn add_option<O>(&mut self, option: O) -> &mut Self
    where
        O: Into<Cow<'o, str>>,
    {
        self.options.push(option.into());
        self
    }

    pub fn not_anonymous(&mut self) -> &mut Self {
        self.is_anonymous = false;
        self
    }

    pub fn quiz(&mut self) -> &mut Self {
        self.type_ = Some(PollType::Quiz);
        self
    }

    pub fn regular(&mut self) -> &mut Self {
        self.type_ = Some(PollType::Regular);
        self
    }

    pub fn allows_multiple_answers(&mut self) -> &mut Self {
        self.allows_multiple_answers = true;
        self
    }

    pub fn correct_option_id(&mut self, id: Integer) -> &mut Self {
        self.correct_option_id = Some(id);
        self
    }

    pub fn explanation<E>(&mut self, text: E) -> &mut Self
    where
        E: Into<Cow<'e, str>>,
    {
        self.explanation = Some(text.into());
        self
    }

    pub fn explanation_parse_mode(&mut self, parse_mode: ParseMode) -> &mut Self {
        self.explanation_parse_mode = Some(parse_mode);
        self
    }

    pub fn open_period(&mut self, period: Integer) -> &mut Self {
        self.open_period = Some(period);
        self
    }

    // TODO: some real date format?
    pub fn close_date(&mut self, date: Integer) -> &mut Self {
        self.close_date = Some(date);
        self
    }

    pub fn closed(&mut self) -> &mut Self {
        self.is_closed = true;
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

/// Send message with a poll.
pub trait CanSendPoll {
    fn poll<'q, 'o, 'e, Q, O>(&self, question: Q, options: Vec<O>) -> SendPoll<'q, 'o, 'e>
    where
        Q: Into<Cow<'q, str>>,
        O: Into<Cow<'o, str>>;
}

impl<C> CanSendPoll for C
where
    C: ToChatRef,
{
    fn poll<'q, 'o, 'e, Q, O>(&self, question: Q, options: Vec<O>) -> SendPoll<'q, 'o, 'e>
    where
        Q: Into<Cow<'q, str>>,
        O: Into<Cow<'o, str>>,
    {
        SendPoll::new(self, question, options)
    }
}

pub trait CanReplySendPoll {
    fn poll_reply<'q, 'o, 'e, Q, O>(&self, question: Q, options: Vec<O>) -> SendPoll<'q, 'o, 'e>
    where
        Q: Into<Cow<'q, str>>,
        O: Into<Cow<'o, str>>;
}

impl<M> CanReplySendPoll for M
where
    M: ToMessageId + ToSourceChat,
{
    fn poll_reply<'q, 'o, 'e, Q, O>(&self, question: Q, options: Vec<O>) -> SendPoll<'q, 'o, 'e>
    where
        Q: Into<Cow<'q, str>>,
        O: Into<Cow<'o, str>>,
    {
        let mut rq = self.to_source_chat().poll(question, options);
        rq.reply_to(self.to_message_id());
        rq
    }
}
