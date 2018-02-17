extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;

use std::env;
use std::time::Duration;

use futures::{Future, Stream};
use tokio_core::reactor::{Handle, Core, Timeout};
use telegram_bot::*;

fn test(api: Api, message: Message, handle: Handle) {
    let timeout = |n| Timeout::new(Duration::from_secs(n), &handle).unwrap().map_err(From::from);
    let api_future = || Ok(api.clone());

    let future = api.send(message.location_reply(0.0, 0.0).live_period(60))
        .join(api_future()).join(timeout(2))
        .and_then(|((message, api), _)| api.send(message.edit_live_location(10.0, 10.0)))
        .join(api_future()).join(timeout(4))
        .and_then(|((message, api), _)| api.send(message.edit_live_location(20.0, 20.0)))
        .join(api_future()).join(timeout(6))
        .and_then(|((message, api), _)| api.send(message.edit_live_location(30.0, 30.0)));

    handle.spawn(future.then(|_| Ok(())))
}

fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let api = Api::configure(token).build(core.handle()).unwrap();

    let future = api.stream().for_each(|update| {
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text {ref data, ..} = message.kind {
                match data.as_str() {
                    "/livelocation" => test(api.clone(), message.clone(), handle.clone()),
                    _ => (),
                }
            }
        }
        Ok(())
    });

    core.run(future).unwrap();
}
