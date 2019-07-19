extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;

use std::env;

use futures::Stream;
use telegram_bot::*;
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new().unwrap();

    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::configure(token).build(core.handle()).unwrap();

    // Convert stream to the stream with errors in result
    let stream = api.stream().then(|mb_update| {
        let res: Result<Result<Update, Error>, ()> = Ok(mb_update);
        res
    });

    // Print update or error for each update.
    let future = stream.for_each(|mb_update| {
        println!("{:?}", mb_update);
        Ok(())
    });

    core.run(future).unwrap();
}
