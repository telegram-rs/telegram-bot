use requests::_base::*;
use types::chat::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GetMe;

impl Request for GetMe {
    type Response = User;
}
