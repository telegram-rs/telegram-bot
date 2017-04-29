use serde::de::{Deserialize, Deserializer};

use types::*;

/// This object represents an incoming update.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Update {
    /// The update‘s unique identifier. Update identifiers start from a certain
    /// positive number and increase sequentially.
    pub id: Integer,
    /// Kind of the incoming update.
    pub kind: UpdateKind,
}

/// Kind of the incoming update.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum UpdateKind {
    /// New incoming message of any kind — text, photo, sticker, etc.
    Message(Message),
    /// New version of a message that is known to the bot and was edited
    EditedMessage(Message),
    /// New incoming channel post of any kind — text, photo, sticker, etc.
    ChannelPost(Message),
    /// New version of a channel post that is known to the bot and was edited
    EditedChannelPost(Message),
    // InlineQuery(InlineQuery),
    // ChosenInlineResult(ChosenInlineResult),
    // CallbackQuery(OptionCallbackQuery),
    #[doc(hidden)]
    Unknown(RawUpdate),
}

impl<'de> Deserialize<'de> for Update {
    fn deserialize<D>(deserializer: D) -> Result<Update, D::Error>
        where D: Deserializer<'de>
    {
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
        // maybe_field!(inline_query, InlineQuery);
        // maybe_field!(chosen_inline_result, ChosenInlineResult);
        // maybe_field!(callback_query, CallbackQuery);

        Ok(Update {
            id: raw.update_id,
            kind: UpdateKind::Unknown(raw),
        })
    }
}

/// This object represents an incoming update. Directly mapped.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct RawUpdate {
    /// The update‘s unique identifier. Update identifiers start from a certain
    /// positive number and increase sequentially.
    pub update_id: Integer,
    /// New incoming message of any kind — text, photo, sticker, etc.
    pub message: Option<Message>,
    /// New version of a message that is known to the bot and was edited
    pub edited_message: Option<Message>,
    /// New incoming channel post of any kind — text, photo, sticker, etc.
    pub channel_post: Option<Message>,
    /// New version of a channel post that is known to the bot and was edited
    pub edited_channel_post: Option<Message>, 
    // pub inline_query: Option<InlineQuery>,
    // pub chosen_inline_result: Option<ChosenInlineResult>,
    // pub callback_query: Option<CallbackQuery>,
}
