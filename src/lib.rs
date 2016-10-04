//! This crate helps writing bots for the messenger Telegram. Here is a
//! minimalistic example:
//!
//! ```no_run
//! use telegram_bot::*;
//!
//! // Create the Api from a bot token saved in a environment variable and
//! // test an Api-call
//! let api = Api::from_env("TELEGRAM_BOT_TOKEN").unwrap();
//! println!("getMe: {:?}", api.get_me());
//! // We want to listen for new updates via LongPoll
//! let mut listener = api.listener(ListeningMethod::LongPoll(None));
//!
//! // Fetch new updates
//! listener.listen(|u| {
//!     // If the received update contains a message...
//!     if let Some(m) = u.message {
//!         // if the message was a text message:
//!         if let MessageType::Text(_) = m.msg {
//!             // Answer message with "Hi"
//!             try!(api.send_message(
//!                 m.chat.id(),
//!                 format!("Hi, {}!", m.from.first_name),
//!                 None, None, None, None)
//!             );
//!         }
//!     }
//!
//!     // If none of the "try!" statements returned an error: It's Ok!
//!     Ok(ListeningAction::Continue)
//! });
//! ```
//!
//! How to use it
//! -------------
//!
//! *Note*: You should be familiar with the
//! [official HTTP Api](https://core.telegram.org/bots/api) to use this library
//! effectivly.
//!
//! The first step is always to create an `Api` object. You need one `Api` for
//! every bot (token) you want to control. You can either create it directly
//! from a token with `from_token` or, since you shouldn't hardcode your token,
//! a bit easier: From an environment variable with `from_env`.
//!
//! The `Api` object has all methods of the Telegram HTTP API, like
//! `send_message`. For more information see the `Api` struct documentation.
//!
//! Next you want to listen for new updates. This is best done via the `listen`
//! method on the `Listener` type. To obtain a listener, call `listener` on the
//! `Api` object.
//!
//! Examples
//! --------
//!
//! There are two examples in the `examples/` directory in the project's
//! repository.
extern crate hyper;
extern crate rustc_serialize;
extern crate url;
extern crate multipart;

mod error;
mod util;
pub mod types;

pub use types::*;
pub use error::*;
use util::Params;

use rustc_serialize::{json, Decodable};
use std::env;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;
use hyper::{Client, Url};
use hyper::client::IntoUrl;
use hyper::client::request::Request;
use hyper::method::Method;
use hyper::header::{Connection, ContentType, ContentLength};
use multipart::client::Multipart;

/// API-URL prefix
pub const API_URL : &'static str = "https://api.telegram.org/bot";

// RequestType let you choose between a post request or a multipart request
enum RequestType {
    Post,
    Multipart(SendPath),
}

// SendPath is used in multipart request to send a local file or an id string
enum SendPath {
    File(String, PathBuf),
    Id(String, String),
}

/// Main type for sending requests to the Telegram bot API.
///
/// You can create an `API` object via `from_token` or `from_env`. A `Listener`
/// object is obtained via `listener`. All remaining methods correspond
/// directly to a telegram API call and are named like the API method, but in
/// `camel_case`.
pub struct Api {
    url: Url,
    client: Client,
}

impl Clone for Api {
    fn clone(&self) -> Api {
        Api {
            url: self.url.clone(),
            client: Client::new(),
        }
    }
}

impl Api {
    // =======================================================================
    // Constructors
    // =======================================================================
    /// Creates a new bot with the given token. If the token is completely
    /// invalid (resulting in an invalid API-URL), the function will return
    /// an `Err` value. However, the function will not check if the given token
    /// is a valid Telegram token. You can call `get_me` to execute a test
    /// request.
    pub fn from_token(token: &str) -> Result<Api> {
        let url = match Url::parse(&format!("{}{}/dummy", API_URL, token)) {
            Ok(url) => url,
            Err(e) => return Err(Error::InvalidTokenFormat(e)),
        };
        Ok(Api {
            url: url,
            client: Client::new(),
        })
    }

    /// Will receive the bot token from the environment variable `var` and call
    /// `from_token` with it. Will return an `Err` value, if the environment
    /// var could not be read or the token has an invalid format.
    pub fn from_env(var: &str) -> Result<Api> {
        let token = match env::var(var) {
            Ok(tok) => tok,
            Err(e) => return Err(Error::InvalidEnvironmentVar(e)),
        };

        Self::from_token(&token)
    }


    // =======================================================================
    // Methods corresponding directly to a API method
    // =======================================================================
    /// Corresponds to the "getMe" method of the API.
    pub fn get_me(&self) -> Result<User> {
        // Execute request with empty parameter list
        self.send_request("getMe", Params::new(), RequestType::Post)
    }

    /// Corresponds to the "sendMessage" method of the API.
    pub fn send_message(&self, chat_id: Integer, text: String,
                        parse_mode: Option<ParseMode>,
                        disable_web_page_preview: Option<bool>,
                        reply_to_message_id: Option<Integer>,
                        reply_markup: Option<ReplyMarkup>)
                        -> Result<Message> {
        // Prepare parameters
        let mut params = Params::new();
        params.add_get("chat_id", chat_id);
        params.add_get("text", text);
        params.add_get_opt("parse_mode", parse_mode);
        params.add_get_opt("disable_web_page_preview", disable_web_page_preview);
        params.add_get_opt("reply_to_message_id", reply_to_message_id);
        try!(params.add_get_json_opt("reply_markup", reply_markup));

        // Execute request
        self.send_request("sendMessage", params, RequestType::Post)
    }

    /// Corresponds to the "forwardMessage" method of the API.
    pub fn forward_message(&self, chat_id: Integer, from_chat_id: Integer,
                           message_id: Integer) -> Result<Message> {
        // Prepare parameters
        let mut params = Params::new();
        params.add_get("chat_id", chat_id);
        params.add_get("from_chat_id", from_chat_id);
        params.add_get("message_id", message_id);

        // Execute request
        self.send_request("forwardMessage", params, RequestType::Post)
    }

    /// Corresponds to the "sendLocation" method of the API.
    pub fn send_location(&self, chat_id: Integer, latitude: Float,
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
        self.send_request("sendLocation", params, RequestType::Post)
    }

    /// Corresponds to the "sendChatAction" method of the API.
    pub fn send_chat_action(&self, chat_id: Integer, action: ChatAction)
                            -> Result<bool> {
        let mut params = Params::new();
        params.add_get("chat_id", chat_id);
        params.add_get("action", action);

        // Execute request
        self.send_request("sendChatAction", params, RequestType::Post)
    }

    /// Corresponds to the "getUserProfilePhotos" method of the API.
    pub fn get_user_profile_photos(&self, user_id: Integer,
                                   offset: Option<Integer>,
                                   limit: Option<Integer>)
                                   -> Result<UserProfilePhotos> {
        let mut params = Params::new();
        params.add_get("user_id", user_id);
        params.add_get_opt("offset", offset);
        params.add_get_opt("limit", limit);

        // Execute request
        self.send_request("getUserProfilePhotos", params, RequestType::Post)
    }

    /// Corresponds to the "getUpdates" method of the API.
    ///
    /// **Note:**
    /// The method will not set the offset parameter on its own. To receive
    /// updates in a more high level way, see `listener`.
    pub fn get_updates(&self, offset: Option<Integer>,
                       limit: Option<Integer>, timeout: Option<Integer>)
                       -> Result<Vec<Update>> {
        // Prepare parameters
        let mut params = Params::new();
        params.add_get_opt("offset", offset);
        params.add_get_opt("limit", limit);
        params.add_get_opt("timeout", timeout);

        // Execute request
        self.send_request("getUpdates", params, RequestType::Post)
    }

    /// Corresponds to the `sendPhoto` method of the API.
    pub fn send_photo(&self, chat_id: Integer, path: String,
                      caption: Option<String>,
                      reply_to_message_id: Option<Integer>,
                      reply_markup: Option<ReplyMarkup>)
                      -> Result<Message> {
        // Prepare parameters
        let mut params = Params::new();
        params.add_get("chat_id", chat_id);
        params.add_get_opt("caption", caption);
        params.add_get_opt("reply_to_message_id", reply_to_message_id);
        try!(params.add_get_json_opt("reply_markup", reply_markup));

        // Convert path string to SendPath
        let path_file = Api::detect_file_or_id("photo", path);
        // Execute request
        self.send_request("sendPhoto", params, RequestType::Multipart(path_file))
    }

    /// Corresponds to the `sendAudio` method of the API.
    pub fn send_audio(&self, chat_id: Integer, path: String,
                      duration: Option<Integer>,
                      performer: Option<String>,
                      title: Option<String>,
                      reply_to_message_id: Option<Integer>,
                      reply_markup: Option<ReplyMarkup>)
                      -> Result<Message> {
        // Prepare parameters
        let mut params = Params::new();
        params.add_get("chat_id", chat_id);
        params.add_get_opt("duration", duration);
        params.add_get_opt("performer", performer);
        params.add_get_opt("title", title);
        params.add_get_opt("reply_to_message_id", reply_to_message_id);
        try!(params.add_get_json_opt("reply_markup", reply_markup));

        // Convert path string to SendPath
        let path_file = Api::detect_file_or_id("audio", path);
        // Execute request
        self.send_request("sendAudio", params, RequestType::Multipart(path_file))
    }

    /// Corresponds to the `sendVoice` method of the API.
    pub fn send_voice(&self, chat_id: Integer, path: String,
                      duration: Option<Integer>,
                      reply_to_message_id: Option<Integer>,
                      reply_markup: Option<ReplyMarkup>)
                      -> Result<Message> {
        // Prepare parameters
        let mut params = Params::new();
        params.add_get("chat_id", chat_id);
        params.add_get_opt("duration", duration);
        params.add_get_opt("reply_to_message_id", reply_to_message_id);
        try!(params.add_get_json_opt("reply_markup", reply_markup));

        // Convert path string to SendPath
        let path_file = Api::detect_file_or_id("voice", path);
        // Execute request
        self.send_request("sendVoice", params, RequestType::Multipart(path_file))
    }


    /// Corresponds to the `sendDocument` method of the API.
    pub fn send_document(&self, chat_id: Integer, path: String,
                         reply_to_message_id: Option<Integer>,
                         reply_markup: Option<ReplyMarkup>)
                         -> Result<Message> {
        // Prepare parameters
        let mut params = Params::new();
        params.add_get("chat_id", chat_id);
        params.add_get_opt("reply_to_message_id", reply_to_message_id);
        try!(params.add_get_json_opt("reply_markup", reply_markup));

        // Convert path string to SendPath
        let path_file = Api::detect_file_or_id("document", path);
        // Execute request
        self.send_request("sendDocument", params, RequestType::Multipart(path_file))
    }

    /// Corresponds to the `sendSticker` method of the API.
    pub fn send_sticker(&self, chat_id: Integer, path: String,
                        reply_to_message_id: Option<Integer>,
                        reply_markup: Option<ReplyMarkup>)
                        -> Result<Message> {
        // Prepare parameters
        let mut params = Params::new();
        params.add_get("chat_id", chat_id);
        params.add_get_opt("reply_to_message_id", reply_to_message_id);
        try!(params.add_get_json_opt("reply_markup", reply_markup));

        // Convert path string to SendPath
        let path_file = Api::detect_file_or_id("sticker", path);
        // Execute request
        self.send_request("sendSticker", params, RequestType::Multipart(path_file))
    }

    /// Corresponds to the `sendVideo` method of the API.
    pub fn send_video(&self, chat_id: Integer, path: String,
                      caption: Option<String>,
                      duration: Option<Integer>,
                      reply_to_message_id: Option<Integer>,
                      reply_markup: Option<ReplyMarkup>)
                      -> Result<Message> {
        // Prepare parameters
        let mut params = Params::new();
        params.add_get("chat_id", chat_id);
        params.add_get_opt("caption", caption);
        params.add_get_opt("duration", duration);
        params.add_get_opt("reply_to_message_id", reply_to_message_id);
        try!(params.add_get_json_opt("reply_markup", reply_markup));

        // Convert path string to SendPath
        let path_file = Api::detect_file_or_id("video", path);
        // Execute request
        self.send_request("sendVideo", params, RequestType::Multipart(path_file))
    }

    /// Corresponds to the `setWebhook` method of the API.
    ///
    /// **Note:**
    /// This library does not yet offer the feature to listen via webhook. This
    /// is just the raw telegram API request and will do nothing more. Use only
    /// if you know what you're doing.
    pub fn set_webhook<U: IntoUrl>(&self, url: Option<U>) -> Result<bool> {
        let u = url.map_or("".into(), |u| u.into_url().unwrap().to_string());

        // Prepare parameters
        let mut params = Params::new();
        params.add_get("url", u);

        // Execute request
        self.send_request("setWebhook", params, RequestType::Post)
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
    // pub fn long_poll<H>(&mut self, timeout: Option<Integer>, mut handler: H)
    //                     -> Result<()>
    //                     where H: FnMut(&mut Api, Update) -> Result<()> {
    //     // Calculate final timeout: Given or default (30s)
    //     let timeout = timeout.or(Some(30));

    //     loop {
    //         // Receive updates with correct offset
    //         let offset = Some(self.offset);
    //         let updates = try!(self.get_updates(offset, None, timeout));

    //         // For every update: Increase the offset and call the handler.
    //         for u in updates {
    //             if u.update_id >= self.offset {
    //                 self.offset = u.update_id + 1;
    //             }

    //             try!(handler(self, u));
    //         }
    //     }
    // }

    pub fn listener(&self, method: ListeningMethod) -> Listener {
        Listener {
            method: method,
            confirmed: 0,
            url: self.url.clone(),
            client: Client::new(),
        }
    }

    // =======================================================================
    // Private methods
    // =======================================================================

    fn detect_file_or_id(name: &str, path: String) -> SendPath {
        // When PathExt becomes stable, use Path::new(&path).exists() instead of this!
        let check = fs::metadata(&path);
        if path.contains(".") && check.is_ok() && check.unwrap().is_file() {
            SendPath::File(name.to_owned(), Path::new(&path).to_path_buf())
        } else {
            SendPath::Id(name.to_owned(), path)
        }
    }

    fn send_request<T: Decodable>(&self, method: &str,
                                  p: Params, typ: RequestType) -> Result<T> {
        Self::request(&self.client, &self.url, method, p, typ)
    }

    fn request<T: Decodable>(client: &Client, url: &Url,
                             method: &str, p: Params, typ: RequestType) -> Result<T> {
        match typ {
            RequestType::Post => Self::post_request(client, url, method, p),
            RequestType::Multipart(sendpath) => Self::multipart_request(url, method, p, sendpath),
        }
    }

    fn multipart_request<T: Decodable>(url: &Url, method: &str,
                                       p: Params, file: SendPath) -> Result<T> {
        // Prepare URL for request: Clone and change the last path fragment
        // to the method name and append GET parameters.
        let mut url = url.clone();
        // if theres a path: Change it
        if let Ok(mut segments_mut) = url.path_segments_mut() {
            segments_mut.pop().push(method.into()); // Change last into method name
        }

        let r = try!(Request::new(Method::Post, url));
        let mut req = try!(Multipart::from_request(r));

        for &(k, ref v) in p.get_params().into_iter() {
            try!(req.write_text(k, v));
        }

        try!(match file {
            SendPath::File(name, path) => {
                match path.to_str() {
                    Some(p) => req.write_file(&name, p),
                    None => return Err(Error::InvalidPath("Invalid path given.".into())),
                }
            },
            SendPath::Id(name, id) => req.write_text(&name, id),
        });

        // Send request and check if it failed
        let mut resp = try!(req.send());

        // Read response into String and return error if it failed
        let mut body = String::new();
        try!(resp.read_to_string(&mut body));

        // Try to decode response as JSON representing a Response
        match try!(json::decode(&body)) {
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

    fn post_request<T: Decodable>(client: &Client, url: &Url,
                                  method: &str, p: Params) -> Result<T> {
        // Prepare URL for request: Clone and change the last path fragment
        // to the method name and append GET parameters.
        let mut url = url.clone();
        // if theres a path: Change it
        if let Ok(mut segments_mut) = url.path_segments_mut() {
            segments_mut.pop().push(method.into()); // Change last into method name
        }

        // Change the parameters to a well formed url-encoded string.
        // Change connect("&") to join("&") when rust 1.3 becomes stable
        let bodyparams = p.get_params().into_iter().map(|&(k, ref  v)| {
            format!("{}={}", k, &**v)
        }).collect::<Vec<_>>().join("&");

        // Create the request with the body and headers
        let req = client
            .post(url)
            .body(&*bodyparams)
            .header(Connection::close())
            .header(ContentType::form_url_encoded())
            .header(ContentLength(bodyparams.len() as u64));

        // Send request and check if it failed
        let mut resp = try!(req.send());

        // Read response into String and return error if it failed
        let mut body = String::new();
        try!(resp.read_to_string(&mut body));

        // Try to decode response as JSON representing a Response
        match try!(json::decode(&body)) {
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

/// Different method how to listen for new updates. Currently `LongPoll` is
/// the only method supported by this library. The Telegram API offers a
/// webhook method which is not yet implemented here.
pub enum ListeningMethod {
    LongPoll(Option<Integer>),
}

/// A listening handler returns this type to signal the listening-method either
/// to stop or to continue. If a handler returns `Stop`, the update it was
/// passed counts as "handled" and won't be handled again.
pub enum ListeningAction {
    Continue,
    Stop
}

/// Offers methods to easily receive new updates via the specified method. This
/// should be used instead of calling methods like `get_updates` yourself.
///
/// To create a listener, you first have to create an `Api` object and call
/// `listener` on it. In order to make listening easier in a concurrent
/// environment, the `Listener` object and the `Api` object don't share any
/// internal state. This makes creating a `Listener` a bit more expensive, but
/// it's usually sufficient for any purpose to create a `Listener` only once.
pub struct Listener {
    method: ListeningMethod,
    confirmed: Integer,
    url: Url,
    client: Client,
}


impl Listener {

    fn send_get_updates(&self, offset: Integer, timeout: Option<Integer>, limit: Option<Integer>)
                        -> Result<Vec<Update>> {
        let mut params = Params::new();
        params.add_get("offset", offset);
        params.add_get_opt("timeout", timeout);
        params.add_get_opt("limit", limit);
        Api::request(&self.client, &self.url, "getUpdates", params, RequestType::Post)
    }

    /// Receive and handle updates with the given closure.
    ///
    /// This method will use the specified listening method to receive new
    /// updates and will then call the given handler for every update. Normally
    /// the handler won't ever be called for the same update twice (see Note
    /// below).
    /// When the handler returns an `Err` value, this function will stop
    /// listening and return the same `Err`. If you want to stop listening you
    /// can return `Ok(ListeningAction::Stop)` instead of an `Err` value.
    ///
    /// When returning an `Ok` value, the update that was passed to the handler
    /// is considered handled and won't be passed to a handler again. On the
    /// other hand if an `Err` is returned, the update is not considered handled
    /// so it will be passed to a handler the next time again.
    ///
    /// **Note:**
    /// If you are listening via `LongPoll` method and your handler panics or
    /// the program is aborted in an abnormal way (e.g. `SIGKILL`), the handler
    /// might receive some already handled updates a second time.
    pub fn listen<H>(&mut self, mut handler: H) -> Result<()>
        where H: FnMut(Update) -> Result<ListeningAction>
    {
        match self.method {
            ListeningMethod::LongPoll(timeout) => {
                // `handled_until` will hold the id of the last handled update
                let mut handled_until = self.confirmed;

                // Calculate final timeout: Given or default (30s)
                let timeout = timeout.or(Some(30));

                loop {
                    // Receive updates with correct offset. We don't specify a
                    // limit (Telegram limits to 100 automatically).
                    let updates = try!(self.send_get_updates(handled_until, timeout, None));

                    self.confirmed = handled_until;

                    // For every update: Increase the offset & call the handler.
                    for u in updates {
                        let update_id = u.update_id;

                        // Execute the handler and save it's result.
                        let res = handler(u);

                        // If an error was returned: Confirm the update before
                        // (if necessary) and return the given error.
                        if let Err(e) = res {
                            // Send a last request to confirm already handled
                            // updates.
                            // We don't specify a timeout (Telegram timeout 0 seconds by default)
                            let _ = try!(self.send_get_updates(handled_until, None, Some(0)));

                            self.confirmed = handled_until;

                            return Err(e);
                        }

                        // The update is now considered "handled". The
                        // if-condition should always be true.
                        if update_id >= handled_until {
                            handled_until = update_id + 1;
                        }

                        // If an Ok(Stop) was returned, stop listening now with
                        // confirmed update.
                        if let Ok(ListeningAction::Stop) = res {
                            // Send a last request to confirm already handled
                            // updates.
                            // We don't specify a timeout (Telegram timeout 0 seconds by default)
                            let _ = try!(self.send_get_updates(handled_until, None, Some(0)));

                            self.confirmed = handled_until;

                            return Ok(());
                        }
                    }
                }
            }
        }
    }

    /// Consumes `self` and returns a sender-receiver pair. You can receive
    /// new updates through the Receiver. Each update needs to be confirmed
    /// with a `Result<ListeningAction>` before the next update can be handled.
    ///
    /// This means that handling updates isn't done in parallel. The only
    /// advantage of this function over the `listen` function is that you can
    /// ask the receiver, if a new update has arrived. This is useful if you
    /// want to handle different events in one thread. E.g. a remainder bot
    /// gets active on every received message AND on timed events.
    ///
    /// **Note:** Remember to send a result through the `Sender` after each
    /// update!
    pub fn channel(mut self)
        -> (mpsc::Sender<Result<ListeningAction>>, mpsc::Receiver<Update>)
    {
        // Create channels for sending updates and handle result
        let (update_tx, update_rx) = mpsc::channel();
        let (res_tx, res_rx) = mpsc::channel();

        // Listen for new updates in a new thread. Sadly we cannot easily
        // return the result of `listen`, so we just discard it.
        thread::spawn(move || {
            let _ = self.listen(|u| {
                // Send received update and return if the receiver hung up.
                if let Err(_) = update_tx.send(u) {
                    return Ok(ListeningAction::Stop);
                }

                // Receive handle result. If the channel hung up: Stop.
                res_rx.recv().unwrap_or(Ok(ListeningAction::Stop))
            });
        });

        (res_tx, update_rx)
    }
}
