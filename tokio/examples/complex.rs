extern crate telegram_bot_tokio;
extern crate tokio_core;
extern crate futures;

use std::env;

use futures::{Future, Stream};
use tokio_core::reactor::{Core, Handle};
use telegram_bot_tokio::{Api, Message, ParseMode, MessageKind, UpdateKind};
use telegram_bot_tokio::prelude::*;

fn message_test(api: &Api, message: &Message, handle: &Handle) {
    let simple = api.send(&message.text_reply("Simple message"));

    let markdown = api.send(&message.text_reply("`Markdown message`")
        .parse_mode(ParseMode::Markdown)
    );

    let html = api.send(&message.text_reply("<b>Bold HTML message</b>")
        .parse_mode(ParseMode::Html)
    );

    handle.spawn({
        let future = simple
            .and_then(|_| markdown)
            .and_then(|_| html);

        future.map_err(|_| ()).map(|_| ())
    })
}

fn preview_test(api: &Api, message: &Message, handle: &Handle) {
    let preview = api.send(&message.text_reply("Message with preview https://telegram.org"));

    let no_preview = api.send(&message.text_reply("Message without preview https://telegram.org")
        .disable_web_page_preview()
    );

    handle.spawn({
        let future = preview.and_then(|_| no_preview);

        future.map_err(|_| ()).map(|_| ())
    })
}

fn reply_test(api: &Api, message: &Message, handle: &Handle) {
    let msg = api.send(&message.text_reply("Reply to message"));
    let chat = api.send(&message.chat.text("Text to message chat"));

    let private = message.from.as_ref().map(|from| {
        api.send(&from.text("Private text"))
    });

    handle.spawn({
        let future = msg.and_then(|_| chat).and_then(|_| private);

        future.map_err(|_| ()).map(|_| ())
    })
}

fn test_forward(api: &Api, message: &Message, _handle: &Handle) {
    api.spawn(&message.forward(&message.chat));

    if let Some(ref from) = message.from {
        api.spawn(&message.forward(from))
    }
}

fn test(api: &Api, message: &Message, handle: &Handle) {
    if let MessageKind::Text {ref data, ..} = message.kind {
        match data.as_str() {
            "/message" => message_test(api, message, handle),
            "/preview" => preview_test(api, message, handle),
            "/reply" => reply_test(api, message, handle),
            "/forward" => test_forward(api, message, handle),
            _ => (),
        }
    }
}

fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let api = Api::from_token(&core.handle(), &token).unwrap();

    let future = api.stream().for_each(|update| {
        if let UpdateKind::Message(message) = update.kind {
            test(&api, &message, &handle)
        }
        Ok(())
    });

    core.run(future).unwrap();
}
