use requests::*;
use types::*;

/// A simple method for testing your bot's auth token. Requires no parameters.
/// Returns basic information about the bot in form of a User object.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetMe;

impl Request for GetMe {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<User>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("getMe"), self)
    }
}
