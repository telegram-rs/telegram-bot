extern crate futures;
extern crate telegram_bot;
extern crate tokio;
extern crate tokio_timer;

use std::env;
use std::ops::Add;
use std::time::{Duration, Instant};

use futures::{future::lazy, Future, Stream};

use tokio_timer::Delay;

use telegram_bot::*;

fn test(api: Api, message: Message) {
    let timeout = |n| Delay::new(Instant::now().add(Duration::from_secs(n))).map_err(From::from);
    let api_future = || Ok(api.clone());

    let future = api
        .send(message.location_reply(0.0, 0.0).live_period(60))
        .join(api_future())
        .join(timeout(2))
        .and_then(|((message, api), _)| api.send(message.edit_live_location(10.0, 10.0)))
        .join(api_future())
        .join(timeout(4))
        .and_then(|((message, api), _)| api.send(message.edit_live_location(20.0, 20.0)))
        .join(api_future())
        .join(timeout(6))
        .and_then(|((message, api), _)| api.send(message.edit_live_location(30.0, 30.0)));

    tokio::executor::current_thread::spawn(future.then(|_| Ok(())));
}

fn main() {
    let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();
    runtime
        .block_on(lazy(|| {
            let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
            let api = Api::configure(token).build().unwrap();

            let stream = api.stream().then(|mb_update| {
                let res: Result<Result<Update, Error>, ()> = Ok(mb_update);
                res
            });

            tokio::executor::current_thread::spawn(stream.for_each(move |update| {
                match update {
                    Ok(update) => {
                        if let UpdateKind::Message(message) = update.kind {
                            if let MessageKind::Text { ref data, .. } = message.kind {
                                match data.as_str() {
                                    "/livelocation" => test(api.clone(), message.clone()),
                                    _ => (),
                                }
                            }
                        }
                    }
                    Err(_) => {}
                }

                Ok(())
            }));

            Ok::<_, ()>(())
        }))
        .unwrap();

    runtime.run().unwrap();
}
