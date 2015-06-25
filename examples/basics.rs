extern crate telegram_bot;

use telegram_bot::{Bot};
use std::env;

fn main() {
    let token = match env::var("TELEGRAM_BOT_TOKEN") {
        Ok(tok) => tok,
        Err(e) => panic!("Environment variable 'TELEGRAM_BOT_TOKEN' missing!"),
    };

    let bot = Bot::new(token);
    println!("{:?}", bot.get_me());
    // println!("{:?}", );
}
