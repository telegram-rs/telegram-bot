use types::*;
use requests::*;

/// Strongly typed ChatAction. Instead of passing a String to the
/// `chat_action` method, this is used.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub enum ChatAction {
    #[serde(rename = "typing")]
    Typing,
    #[serde(rename = "upload_photo")]
    UploadPhoto,
    #[serde(rename = "record_video")]
    RecordVideo,
    #[serde(rename = "upload_video")]
    UploadVideo,
    #[serde(rename = "record_audio")]
    RecordAudio,
    #[serde(rename = "upload_audio")]
    UploadAudio,
    #[serde(rename = "upload_document")]
    UploadDocument,
    #[serde(rename = "find_location")]
    FindLocation,
}

/// Use this method when you need to tell the user that something is happening on the bot's side.
/// The status is set for 5 seconds or less (when a message arrives from your bot,
/// Telegram clients clear its typing status).
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct SendChatAction<'c> {
    chat_id: ChatRef<'c>,
    action: ChatAction,
}

impl<'c> Request for SendChatAction<'c> {
    type Response = TrueToUnitResponse;

    fn name() -> &'static str {
        "sendChatAction"
    }
}

impl<'c> SendChatAction<'c> {
    pub fn new<C>(chat: C, action: ChatAction) -> Self where C: ToChatRef<'c> {
        SendChatAction {
            chat_id: chat.to_chat_ref(),
            action: action,
        }
    }
}

pub trait CanSendChatAction<'c> {
    fn chat_action(&self, action: ChatAction) -> SendChatAction<'c>;
}

impl<'c, C> CanSendChatAction<'c> for C where C: ToChatRef<'c> {
    fn chat_action(&self, action: ChatAction) -> SendChatAction<'c> {
        SendChatAction::new(self, action)
    }
}
