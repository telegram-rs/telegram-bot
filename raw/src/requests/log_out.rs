use crate::requests::*;

/// Use this method to log out from the cloud Bot API server before launching the bot locally.
/// You must log out the bot before running it locally,
/// therwise there is no guarantee that the bot will receive updates.
/// After a successful call, you can immediately log in on a local server,
/// but will not be able to log in back to the cloud Bot API server for 10 minutes.
/// Returns True on success. Requires no parameters.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct LogOut;

impl Request for LogOut {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<bool>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("logOut"), self)
    }
}
