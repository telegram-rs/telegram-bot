extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;

use std::env;

use futures::Stream;
use telegram_bot::*;
use tokio_core::reactor::Core;

fn process(api: Api, message: Message) {
    if let MessageKind::Text { ref data, .. } = message.kind {
        match data.as_str() {
            "/pin" => message
                .reply_to_message
                .map(|message| api.spawn(message.pin()))
                .unwrap_or(()),
            "/unpin" => api.spawn(message.chat.unpin_message()),
            _ => (),
        }
    }
}

fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");

    let mut core = Core::new().unwrap();

    let api = Api::configure(token).build(core.handle()).unwrap();

    let future = api.stream().for_each(|update| {
        if let UpdateKind::Message(message) = update.kind {
            process(api.clone(), message)
        }
        Ok(())
    });

    core.run(future).unwrap();
}
