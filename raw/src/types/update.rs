use serde::de::{Deserialize, Deserializer};

use types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Update {
    pub id: Integer,
    pub kind: UpdateKind,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum UpdateKind {
    Message(Message),
    EditedMessage(Message),
    ChannelPost(Message),
    EditedChannelPost(Message),
//    InlineQuery(InlineQuery),
//    ChosenInlineResult(ChosenInlineResult),
//    CallbackQuery(OptionCallbackQuery),
    Unknown(RawUpdate)
}

impl Deserialize for Update {
    fn deserialize<D>(deserializer: D) -> Result<Update, D::Error> where D: Deserializer {
        let raw: RawUpdate = Deserialize::deserialize(deserializer)?;
        macro_rules! maybe_field {
            ($name:ident, $variant:ident) => {{
                if let Some(val) = raw.$name {
                    return Ok(Update {
                        id: raw.update_id,
                        kind: UpdateKind::$variant(val),
                    })
                }
            }}
        }

        maybe_field!(message, Message);
        maybe_field!(edited_message, EditedMessage);
        maybe_field!(channel_post, ChannelPost);
        maybe_field!(edited_channel_post, EditedChannelPost);
//        maybe_field!(inline_query, InlineQuery);
//        maybe_field!(chosen_inline_result, ChosenInlineResult);
//        maybe_field!(callback_query, CallbackQuery);

        Ok(Update {
            id: raw.update_id,
            kind: UpdateKind::Unknown(raw),
        })
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct RawUpdate {
    pub update_id: Integer,
    pub message: Option<Message>,
    pub edited_message: Option<Message>,
    pub channel_post: Option<Message>,
    pub edited_channel_post: Option<Message>,
//    pub inline_query: Option<InlineQuery>,
//    pub chosen_inline_result: Option<ChosenInlineResult>,
//    pub callback_query: Option<CallbackQuery>,
}
