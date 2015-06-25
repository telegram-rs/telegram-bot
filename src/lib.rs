extern crate hyper;
extern crate rustc_serialize;

mod types;

pub use types::*;

use rustc_serialize::{json, Decodable};
use std::io::Read;
use std::fmt;
use hyper::Client;
use hyper::header::Connection;

pub struct Bot {
    token: String,
}

impl Bot {
    pub fn new(token: String) -> Bot {
        Bot {
            token: token,
        }
    }

    fn request_url(&self, method: &str) -> String {
        format!("https://api.telegram.org/bot{}/{}", self.token, method)
    }

    fn send_request<T: Decodable>(&self, method: &str) -> Result<T> {
        let url = self.request_url(method);

        let mut client = Client::new();
        let req = client.get(&url).header(Connection::close());

        let mut resp = match req.send() {
            Ok(resp) => resp,
            Err(e) => return Err(Error::Http(e)),
        };

        let mut body = String::new();
        if let Err(e) = resp.read_to_string(&mut body) {
            return Err(Error::Io(e));
        }

        match json::decode(&*body) {
            Err(e) => Err(Error::Json(e)),
            Ok(Response { ok: false, description: Some(desc), ..}) => {
                Err(Error::Api(desc))
            },
            Ok(Response { ok: true, result: Some(res), ..}) => {
                Ok(res)
            },
            _ => Err(Error::InvalidState("Invalid server response".into())),
        }
    }

    pub fn get_me(&self) -> Result<User> {
        self.send_request("getMe")
    }

    pub fn get_updates(&mut self) -> Result<Vec<Update>> {
        self.send_request("getUpdates")
    }
}



pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Http(hyper::error::Error),
    Io(std::io::Error),
    Json(json::DecoderError),
    Api(String),
    InvalidState(String),
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Http(ref e) => e.description(),
            Error::Io(ref e) => e.description(),
            Error::Json(ref e) => e.description(),
            Error::Api(ref s) => &*s,
            Error::InvalidState(ref s) => &*s,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Http(ref e) => e.fmt(f),
            Error::Io(ref e) => e.fmt(f),
            Error::Json(ref e) => e.fmt(f),
            Error::Api(ref s) => s.fmt(f),
            Error::InvalidState(ref s) => s.fmt(f),
        }
    }
}


#[test]
fn it_works() {
}
