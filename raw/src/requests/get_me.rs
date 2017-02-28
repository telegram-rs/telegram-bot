use requests::*;
use types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct GetMe;

impl Request for GetMe {
    type Response = User;

    fn name(&self) -> &'static str {
        "getMe"
    }
}
