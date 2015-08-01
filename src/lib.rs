extern crate hyper;
extern crate rustc_serialize;

mod error;
mod util;
pub mod types;

pub use types::*;
pub use error::*;
use util::Params;

use rustc_serialize::{json, Decodable};
use std::io::Read;
use hyper::{Client, Url};
use hyper::client::IntoUrl;
use hyper::header::Connection;

/// API-URL prefix
pub const API_URL : &'static str = "https://api.telegram.org/bot";

/// Main type for sending requests to the Telegram bot API.
pub struct Api {
    offset: Integer,
    url: Url,
    client: Client,
}

impl Api {
    // =======================================================================
    // Constructors
    // =======================================================================
    /// Creates a new bot with the given token. If the token is completely
    /// invalid (resulting in an invalid API-URL), the function will panic.
    /// However, the function will not check if the given token is a valid
    /// Telegram token. You can call `get_me` to execute a test request.
    pub fn new(token: String) -> Api {
        let url = match Url::parse(&*format!("{}{}/dummy", API_URL, token)) {
            Ok(url) => url,
            Err(e) => panic!("Invalid token! ({})", e),
        };
        Api {
            offset: 0,
            url: url,
            client: Client::new(),
        }
    }


    // =======================================================================
    // Methods corresponding directly to a API method
    // =======================================================================
    /// Corresponds to the "getMe" method of the API.
    pub fn get_me(&mut self) -> Result<User> {
        // Execute request with empty parameter list
        self.send_request("getMe", Params::new())
    }

    /// Corresponds to the "sendMessage" method of the API.
    pub fn send_message(&mut self, chat_id: Integer, text: String,
                        disable_web_page_preview: Option<bool>,
                        reply_to_message_id: Option<Integer>,
                        reply_markup: Option<ReplyMarkup>)
                        -> Result<Message> {
        // Prepare parameters
        let mut params = Params::new();
        params.add_get("chat_id", chat_id);
        params.add_get("text", text);
        params.add_get_opt("disable_web_page_preview", disable_web_page_preview);
        params.add_get_opt("reply_to_message_id", reply_to_message_id);
        try!(params.add_get_json_opt("reply_markup", reply_markup));

        // Execute request
        self.send_request("sendMessage", params)
    }

    /// Corresponds to the "forwardMessage" method of the API.
    pub fn forward_message(&mut self, chat_id: Integer, from_chat_id: Integer,
                           message_id: Integer) -> Result<Message> {
        // Prepare parameters
        let mut params = Params::new();
        params.add_get("chat_id", chat_id);
        params.add_get("from_chat_id", from_chat_id);
        params.add_get("message_id", message_id);

        // Execute request
        self.send_request("forwardMessage", params)
    }

    /// Corresponds to the "sendLocation" method of the API.
    pub fn send_location(&mut self, chat_id: Integer, latitude: Float,
                         longitude: Float, reply_to_message_id: Option<Integer>,
                         reply_markup: Option<ReplyMarkup>)
                         -> Result<Message> {
        // Prepare parameters
        let mut params = Params::new();
        params.add_get("chat_id", chat_id);
        params.add_get("latitude", latitude);
        params.add_get("longitude", longitude);
        params.add_get_opt("reply_to_message_id", reply_to_message_id);
        try!(params.add_get_json_opt("reply_markup", reply_markup));

        // Execute request
        self.send_request("sendLocation", params)
    }

    /// Corresponds to the "sendChatAction" method of the API.
    pub fn send_chat_action(&mut self, chat_id: Integer, action: ChatAction)
                            -> Result<bool> {
        let mut params = Params::new();
        params.add_get("chat_id", chat_id);
        params.add_get("action", action);

        // Execute request
        self.send_request("sendChatAction", params)
    }

    /// Corresponds to the "getUserProfilePhotos" method of the API.
    pub fn get_user_profile_photos(&mut self, user_id: Integer,
                                   offset: Option<Integer>,
                                   limit: Option<Integer>)
                                   -> Result<UserProfilePhotos> {
        let mut params = Params::new();
        params.add_get("user_id", user_id);
        params.add_get_opt("offset", offset);
        params.add_get_opt("limit", limit);

        // Execute request
        self.send_request("getUserProfilePhotos", params)
    }

    /// Corresponds to the "getUpdates" method of the API.
    ///
    /// **Note:**
    /// The method will not set the offset parameter on its own. To receive
    /// updates in a more high level way, see `long_poll`.
    pub fn get_updates(&mut self, offset: Option<Integer>,
                       limit: Option<Integer>, timeout: Option<Integer>)
                       -> Result<Vec<Update>> {
        // Prepare parameters
        let mut params = Params::new();
        params.add_get_opt("offset", offset);
        params.add_get_opt("limit", limit);
        params.add_get_opt("timeout", timeout);

        // Execute request
        self.send_request("getUpdates", params)
    }

    pub fn set_webhook<U: IntoUrl>(&mut self, url: Option<U>) -> Result<bool> {
        let u = url.map_or("".into(), |u| u.into_url().unwrap().to_string());

        // Prepare parameters
        let mut params = Params::new();
        params.add_get("url", u);

        // Execute request
        self.send_request("setWebhook", params)
    }


    // =======================================================================
    // Methods for receiving updates
    // =======================================================================
    /// Receive and handle updates via "getUpdates".
    ///
    /// This method will repeatedly call `get_updates` to receive new updates.
    /// It will then call the given handler for every update and increase the
    /// update offset accordingly, so the handler will never be called with
    /// the same update twice.
    /// The `timeout` parameter influences how long (in seconds) each poll may
    /// last. Defaults to 30.
    /// The handler gets a mutable reference to the bot since borrowing it
    /// from the outer scope won't work. When the handler returns an `Err`
    /// value the bot will stop listening for updates and `long_poll` will
    /// return the Error. If you want to stop listening you can just return
    /// `Error::UserInterrupt`.
    ///
    /// **Note:**
    /// If the bot is restarted, but the last received updates are not yet
    /// confirmed (the last poll was not empty), there will be some duplicate
    /// updates.
    pub fn long_poll<H>(&mut self, timeout: Option<Integer>, mut handler: H)
                        -> Result<()>
                        where H: FnMut(&mut Api, Update) -> Result<()> {
        // Calculate final timeout: Given or default (30s)
        let timeout = timeout.or(Some(30));

        loop {
            // Receive updates with correct offset
            let offset = Some(self.offset);
            let updates = try!(self.get_updates(offset, None, timeout));

            // For every update: Increase the offset and call the handler.
            for u in updates {
                if u.update_id >= self.offset {
                    self.offset = u.update_id + 1;
                }

                try!(handler(self, u));
            }
        }
    }

    // =======================================================================
    // Private methods
    // =======================================================================
    fn send_request<T: Decodable>(&mut self, method: &str, p: Params)
                   -> Result<T> {
        // Prepare URL for request: Clone and change the last path fragment
        // to the method name and append GET parameters.
        let mut url = self.url.clone();
        url.path_mut().map(|path| {         // if theres a path: Change it
            path.last_mut().map(|last| {    // if its not empty: Change last...
                *last = method.into()       // ... into method name
            })
        });

        // For all (str, String) pairs: Map to (str, str) and append it to URL
        let it = p.get_params().iter().map(|&(k, ref v)| (k, &**v));
        url.set_query_from_pairs(it);

        // Prepare HTTP Request
        let req = self.client.get(url).header(Connection::close());

        // Send request and check if it failed
        let mut resp = try!(req.send());

        // Read response into String and return error if it failed
        let mut body = String::new();
        try!(resp.read_to_string(&mut body));

        // Try to decode response as JSON representing a Response
        match try!(json::decode(&*body)) {
            // If the response says that there was an error: Return API-Error
            // with the given description.
            Response { ok: false, description: Some(desc), ..} => {
                Err(Error::Api(desc))
            },
            // If response is "ok": Return the result.
            Response { ok: true, result: Some(res), ..} => {
                Ok(res)
            },
            // This should never occur: If "ok"==false, "description" should
            // always be Some. If "ok"==true, then "result" should always be
            // Some. We could also panic in this case.
            _ => Err(Error::InvalidState("Invalid server response".into())),
        }
    }
}
