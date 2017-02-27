use serde::de::{Deserialize, Deserializer, Error};

use types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Response<T> {
    Success {
        result: T,
    },
    Error {
        description: String,
        parameters: Option<ResponseParameters>,
    },
}

impl<T: Deserialize> Deserialize for Response<T> {
    fn deserialize<D>(deserializer: D) -> Result<Response<T>, D::Error> where D: Deserializer {
        let raw: RawResponse<T> = Deserialize::deserialize(deserializer)?;
        match (raw.ok, raw.description, raw.result) {
            (false, Some(description), None) => {
                Ok(Response::Error {
                    description: description,
                    parameters: raw.parameters,
                })
            },
            (true, None, Some(result)) => {
                Ok(Response::Success {
                    result: result,
                })
            }
            _ => Err(D::Error::custom("ambiguous response")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct RawResponse<T> {
    ok: bool,
    description: Option<String>,
    result: Option<T>,
    parameters: Option<ResponseParameters>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct ResponseParameters {
    pub migrate_to_chat_id: Option<Integer>,
    pub retry_after: Option<Integer>,
}
