use requests::*;
use types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct GetMe;

impl Request for GetMe {
    type Response = User;
    type RawResponse = User;

    fn map(raw: Self::RawResponse) -> Self::Response {
        raw
    }

    fn name() -> &'static str {
        "getMe"
    }
}
