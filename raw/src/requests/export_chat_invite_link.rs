use crate::requests::*;
use crate::types::*;

/// Use this method to export chat invite links.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct ExportChatInviteLink {
    chat_id: ChatRef,
}

impl Request for ExportChatInviteLink {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<String>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("exportChatInviteLink"), self)
    }
}

impl ExportChatInviteLink {
    pub fn new<C>(chat: C) -> Self
    where
        C: ToChatRef,
    {
        ExportChatInviteLink {
            chat_id: chat.to_chat_ref(),
        }
    }
}

/// Export chat invite link.
pub trait CanExportChatInviteLink {
    fn export_invite_link(&self) -> ExportChatInviteLink;
}

impl<C> CanExportChatInviteLink for C
where
    C: ToChatRef,
{
    fn export_invite_link(&self) -> ExportChatInviteLink {
        ExportChatInviteLink::new(self)
    }
}
