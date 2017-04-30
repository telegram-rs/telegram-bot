extern crate telegram_bot;
extern crate tokio_core;

use std::env;

use tokio_core::reactor::Core;
use telegram_bot::{Api, GetMe};

fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();

    let mut core = Core::new().unwrap();

    let api = Api::from_token(&core.handle(), &token).unwrap();
    let future = api.send(GetMe);

    println!("{:?}", core.run(future))
}
