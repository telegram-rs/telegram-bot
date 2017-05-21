use types::*;
use requests::*;

/// Use this method for your bot to leave a group, supergroup or channel.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct LeaveChat {
    chat_id: ChatRef
}

impl Request for LeaveChat {
    type Response = TrueToUnitResponse;

    fn name(&self) -> &'static str {
        "leaveChat"
    }
}

impl LeaveChat {
    pub fn new<C>(chat: C) -> Self where C: ToChatRef {
        LeaveChat {
            chat_id: chat.to_chat_ref()
        }
    }
}

/// Leave a group, supergroup or channel.
pub trait CanLeaveChat {
    fn leave(&self) -> LeaveChat;
}

impl<C> CanLeaveChat for C where C: ToChatRef {
    fn leave(&self) -> LeaveChat {
        LeaveChat::new(self)
    }
}
