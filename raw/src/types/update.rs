use crate::types::*;

/// This object represents an incoming update.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Update {
    /// The update‘s unique identifier. Update identifiers start from a certain
    /// positive number and increase sequentially.
    #[serde(rename = "update_id")]
    pub id: Integer,
    /// Kind of the incoming update.
    #[serde(flatten)]
    pub kind: UpdateKind,
}

/// Kind of the incoming update.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub enum UpdateKind {
    /// New incoming message of any kind — text, photo, sticker, etc.
    #[serde(rename = "message")]
    Message(Message),
    /// New version of a message that is known to the bot and was edited
    #[serde(rename = "edited_message")]
    EditedMessage(Message),
    /// New incoming channel post of any kind — text, photo, sticker, etc.
    #[serde(rename = "channel_post")]
    ChannelPost(ChannelPost),
    /// New version of a channel post that is known to the bot and was edited
    #[serde(rename = "edited_channel_post")]
    EditedChannelPost(ChannelPost),
    #[serde(rename = "inline_query")]
    InlineQuery(InlineQuery),
    /// The result of an inline query that was chosen by a user and sent to their chat partner. Please see our documentation on the feedback collecting for details on how to enable these updates for your bot.
    #[serde(rename = "chosen_inline_result")]
    ChosenInlineResult(ChosenInlineResult),
    #[serde(rename = "callback_query")]
    CallbackQuery(CallbackQuery),
    /// New incoming shipping query. Only for invoices with flexible price
    #[serde(rename = "shipping_query")]
    ShippingQuery(ShippingQuery),
    /// New incoming pre-checkout query. Contains full information about checkout
    #[serde(rename = "pre_checkout_query")]
    PreCheckoutQuery(PreCheckoutQuery),
    /// New poll state. Bots receive only updates about stopped polls and polls, which are sent by the bot
    #[serde(rename = "poll")]
    Poll(Poll),
    /// A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself
    #[serde(rename = "poll_answer")]
    PollAnswer(PollAnswer),
    /// The bot's chat member status was updated in a chat. For private chats, this update is received only when the bot is blocked or unblocked by the user.
    #[serde(rename = "my_chat_member")]
    MyChatMember(ChatMemberUpdate),
    /// A chat member's status was updated in a chat. The bot must be an administrator in the chat and must explicitly specify “chat_member” in the list of allowed_updates to receive these updates.
    #[serde(rename = "chat_member")]
    ChatMember(ChatMemberUpdate),
    #[doc(hidden)]
    Error(String),
    #[doc(hidden)]
    Unknown,
}
