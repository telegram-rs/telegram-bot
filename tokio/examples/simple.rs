extern crate telegram_bot_raw;
extern crate telegram_bot_tokio;
extern crate tokio_core;

use std::env;

use tokio_core::reactor::Core;
use telegram_bot_raw::GetMe;
use telegram_bot_tokio::Api;

fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();

    let mut core = Core::new().unwrap();

    let api = Api::from_token(&core.handle(), &token).unwrap();
    let future = api.send(GetMe);

    println!("{:?}", core.run(future))
}
