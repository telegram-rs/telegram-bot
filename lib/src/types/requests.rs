//! Telegram Bot API methods.

pub use telegram_bot_raw::AnswerCallbackQuery;
pub use telegram_bot_raw::{DeleteMessage, ForwardMessage};
pub use telegram_bot_raw::{GetFile, GetMe, GetUpdates, GetUserProfilePhotos};
pub use telegram_bot_raw::{SendAudio, SendContact, SendLocation, SendMessage, SendVenue};
pub use telegram_bot_raw::{GetChat, LeaveChat};
pub use telegram_bot_raw::SendChatAction;
pub use telegram_bot_raw::{GetChatAdministrators, GetChatMember, GetChatMembersCount};
pub use telegram_bot_raw::{KickChatMember, UnbanChatMember, RestrictChatMember};
pub use telegram_bot_raw::{EditMessageCaption, EditMessageReplyMarkup, EditMessageText};
pub use telegram_bot_raw::{PinChatMessage, UnpinChatMessage};
pub use telegram_bot_raw::{EditMessageLiveLocation, StopMessageLiveLocation};
