use std::fmt;

use serde::de::{Deserialize, Deserializer, Error, MapAccess, Visitor};
use serde_value::Value;

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
    ChannelPost(ChannelPost),
    /// New version of a channel post that is known to the bot and was edited
    EditedChannelPost(ChannelPost),
    // InlineQuery(InlineQuery),
    // ChosenInlineResult(ChosenInlineResult),
    CallbackQuery(CallbackQuery),
    #[doc(hidden)]
    Error(String),
    #[doc(hidden)]
    Unknown,
}

impl<'de> Deserialize<'de> for Update {
    fn deserialize<D>(deserializer: D) -> Result<Update, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            UpdateId,
            Message,
            EditedMessage,
            ChannelPost,
            EditedChannelPost,
            CallbackQuery,
        }

        struct UpdateVisitor;

        impl<'de> Visitor<'de> for UpdateVisitor {
            type Value = Update;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Update")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut update_id = None;
                let mut error: Option<V::Error> = None;

                macro_rules! match_fields {
                    ($(($variant:ident, $name:ident));*; ) => {{
                        $(
                            let mut $name = None;
                        )*

                        while let Some(key) = map.next_key()? {
                            match key {
                                Field::UpdateId => {
                                    if update_id.is_some() {
                                        return Err(Error::duplicate_field("update_id"));
                                    }
                                    update_id = Some(map.next_value()?)
                                },
                                $(
                                    Field::$variant => {
                                        if $name.is_some() {
                                            return Err(Error::duplicate_field(stringify!($name)));
                                        }
                                        let value = map.next_value::<Value>()?;
                                        match value.deserialize_into() {
                                            Ok(value) => $name = Some(value),
                                            Err(err) => error = Some(Error::custom(err)),
                                        }
                                    }
                                )*
                            }
                        }

                        let update_id = update_id.ok_or_else(|| Error::missing_field("update_id"))?;

                        if let Some(err) = error {
                            return Ok(Update {
                                id: update_id,
                                kind: UpdateKind::Error(format!("{}", err))
                            })
                        }

                        $(
                            if let Some(val) = $name {
                                return Ok(Update {
                                    id: update_id,
                                    kind: UpdateKind::$variant(val),
                                })
                            }
                        )*

                        return Ok(Update {
                            id: update_id,
                            kind: UpdateKind::Unknown,
                        })
                    }};
                }

                match_fields!(
                    (Message, message);
                    (EditedMessage, edited_message);
                    (ChannelPost, channel_post);
                    (EditedChannelPost, edited_channel_post);
                    (CallbackQuery, callback_query);
                )
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "update_id",
            "message",
            "edited_message",
            "channel_post",
            "edited_channel_post",
            "callback_query",
        ];

        deserializer.deserialize_struct("Duration", FIELDS, UpdateVisitor)
    }
}
