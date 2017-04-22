use types::*;
use requests::*;

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

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct SendChatAction<'c> {
    chat_id: ChatId<'c>,
    action: ChatAction,
}

impl<'c> Request for SendChatAction<'c> {
    type Response = ();
    type RawResponse = True;

    fn map(_raw: Self::RawResponse) -> Self::Response {
        ()
    }

    fn name() -> &'static str {
        "sendChatAction"
    }
}

impl<'c> SendChatAction<'c> {
    pub fn new<C>(chat: C, action: ChatAction) -> Self where C: Into<ChatId<'c>> {
        SendChatAction {
            chat_id: chat.into(),
            action: action,
        }
    }
}

pub trait CanSendChatAction<'bc, 'c> {
    fn chat_action(&'bc self, action: ChatAction) -> SendChatAction<'c>;
}

impl<'c, 'bc, C: 'bc> CanSendChatAction<'bc, 'c> for C where &'bc C: Into<ChatId<'c>> {
    fn chat_action(&'bc self, action: ChatAction) -> SendChatAction<'c> {
        SendChatAction::new(self, action)
    }
}
