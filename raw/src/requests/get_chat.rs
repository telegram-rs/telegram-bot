use types::*;
use requests::*;

/// Use this method to get up to date information about the chat.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetChat {
    chat_id: ChatRef
}

impl Request for GetChat {
    type Response = IdResponse<Chat>;

    fn name(&self) -> &'static str {
        "getChat"
    }
}

impl GetChat {
    pub fn new<C>(chat: C) -> Self where C: ToChatRef {
        GetChat {
            chat_id: chat.to_chat_ref()
        }
    }
}

/// Get up to date information about the chat.
pub trait CanGetChat {
    fn get_chat(&self) -> GetChat;
}

impl<C> CanGetChat for C where C: ToChatRef {
    fn get_chat(&self) -> GetChat {
        GetChat::new(self)
    }
}
