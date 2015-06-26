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
    offset: Integer,
}

impl Bot {
    pub fn new(token: String) -> Bot {
        Bot {
            token: token,
            offset: 0,
        }
    }

    fn request_url(&self, suffix: &str) -> String {
        format!("https://api.telegram.org/bot{}/{}", self.token, suffix)
    }

    fn send_request<T: Decodable>(&self, suffix: String) -> Result<T> {
        let url = self.request_url(&*suffix);
        // println!("URL: {}", url);

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
        self.send_request("getMe".into())
    }

    pub fn get_updates(&self, offset: Option<Integer>,
                       limit: Option<Integer>, timeout: Option<Integer>)
                       -> Result<Vec<Update>> {
        // Set method name and append possible parameters
        let mut suffix : String = "getUpdates".into();

        let mut params : String = "?".into();
        if let Some(i) = offset { params.push_str(&*format!("offset={}&", i)); }
        if let Some(i) = limit { params.push_str(&*format!("limit={}&", i)); }
        if let Some(i) = timeout { params.push_str(&*format!("timeout={}&", i)); }
        if params.len() > 0 {
            params.pop();   // remove last '&'
            suffix.push_str(&*params);
        }

        self.send_request(suffix)
    }

    pub fn long_poll<F>(&mut self, timeout: Option<Integer>, handler: F)
                        -> Result<()>
                        where F: Fn(&mut Bot, Update) {
        let timeout = Some(if let Some(t) = timeout { t } else { 10 });
        loop {
            let updates = try!(self.get_updates(
                Some(self.offset), None, timeout));
            // println!("Getting Updates...");

            for u in updates {
                if u.update_id >= self.offset {
                    self.offset = u.update_id + 1;
                }

                handler(self, u);
            }
        }
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
