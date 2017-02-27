use requests::_base::*;
use types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GetMe;

impl Request for GetMe {
    type Response = User;
}
