extern crate telegram_bot;
extern crate tokio_core;

use std::env;

use telegram_bot::{Api, GetMe};
use tokio_core::reactor::Core;

fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");

    let mut core = Core::new().unwrap();

    let api = Api::configure(token).build(core.handle()).unwrap();
    let future = api.send(GetMe);

    println!("{:?}", core.run(future))
}
