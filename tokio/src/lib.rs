#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;
extern crate tokio_core;
extern crate url;
extern crate telegram_bot_raw;

mod api;
mod errors;
pub mod prelude;
mod stream;

pub use telegram_bot_raw::{Integer, Float, True};
pub use telegram_bot_raw::{Update, UpdateKind, RawUpdate};
pub use telegram_bot_raw::{User, Group, Supergroup, Channel, Chat, RawChat};
pub use telegram_bot_raw::{Message, MessageKind, RawMessage, Forward, ForwardFrom};
pub use telegram_bot_raw::{MessageEntity, MessageEntityKind, RawMessageEntity};
pub use telegram_bot_raw::{Audio, Document, PhotoSize, Sticker, Video, Voice};
pub use telegram_bot_raw::{Contact, Location, Venue};

pub use telegram_bot_raw::{ParseMode, ChatId, ReplyTo};
pub use telegram_bot_raw::{ReplyMarkup, InlineKeyboardMarkup, ReplyKeyboardMarkup};
pub use telegram_bot_raw::{ReplyKeyboardRemove, ForceReply};
pub use telegram_bot_raw::{SendMessage, GetMe};

pub use self::api::{Api, TelegramFuture};
pub use self::errors::{Error, Result, ErrorKind};
pub use stream::UpdatesStream;
