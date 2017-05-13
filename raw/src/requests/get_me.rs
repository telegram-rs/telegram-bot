use requests::*;
use types::*;

/// A simple method for testing your bot's auth token. Requires no parameters.
/// Returns basic information about the bot in form of a User object.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct GetMe;

impl Request for GetMe {
    type Response = IdResponse<User>;

    fn name() -> &'static str {
        "getMe"
    }
}
