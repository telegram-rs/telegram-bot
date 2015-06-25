extern crate rustc_serialize;
extern crate telegram_bot;

use telegram_bot::{Bot, Update};
use std::env;
use rustc_serialize::json;

fn main() {
    let token = match env::var("TELEGRAM_BOT_TOKEN") {
        Ok(tok) => tok,
        Err(e) => panic!("Environment variable 'TELEGRAM_BOT_TOKEN' missing!"),
    };

    let mut bot = Bot::new(token);
    println!("{:?}", bot.get_me());
    let u = bot.get_updates();
    println!("{:?}", u);
    // println!("{}", u.unwrap_err());
    // println!("{:?}", );

}
