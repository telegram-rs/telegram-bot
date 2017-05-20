//! Telegram Bot API methods.

pub use telegram_bot_raw::{DeleteMessage, ForwardMessage};
pub use telegram_bot_raw::{GetFile, GetMe, GetUpdates, GetUserProfilePhotos};
pub use telegram_bot_raw::{SendLocation, SendMessage, SendContact, SendVenue};
pub use telegram_bot_raw::{GetChat, LeaveChat};
pub use telegram_bot_raw::SendChatAction;
pub use telegram_bot_raw::{GetChatAdministrators, GetChatMember, GetChatMembersCount};
pub use telegram_bot_raw::{KickChatMember, UnbanChatMember};
pub use telegram_bot_raw::{EditMessageText, EditMessageCaption, EditMessageReplyMarkup};
