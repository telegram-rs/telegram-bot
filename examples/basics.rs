#![allow(unused_imports)]
extern crate rustc_serialize;
extern crate telegram_bot;

use telegram_bot::*;
use std::env;
use rustc_serialize::json;

fn main() {
    let token = match env::var("TELEGRAM_BOT_TOKEN") {
        Ok(tok) => tok,
        Err(e) =>
            panic!("Environment variable 'TELEGRAM_BOT_TOKEN' missing! {}", e),
    };

    let mut bot = Bot::new(token);
    println!("{:?}", bot.get_me());
    // let u = bot.get_updates(None, Some(3), None);
    // println!("{:?}", u);

    let res = bot.long_poll(None, |bot, u| {
        print!("Received Update[{}]: ", u.update_id);
        if let Some(m) = u.message {
            print!("<{}> ", m.from.first_name);
            if let MessageType::Text(t) = m.msg {
                print!("{}", t);
            }
        }
        println!("");
    });

    if let Err(e) = res {
        println!("An error occured: {}", e);
    }

    // println!("{}", u.unwrap_err());
    // println!("{:?}", );

    // let u : Result<Update, _> = json::decode(r#"
    //     {
    //   "update_id": 102520066,
    //   "message": {
    //     "message_id": 6,
    //     "from": {
    //       "id": 10742080,
    //       "first_name": "Lukas",
    //       "last_name": "Kalbertodt",
    //       "username": "LukasKalbertodt"
    //     },
    //     "chat": {
    //       "id": 10742080,
    //       "first_name": "Lukas",
    //       "last_name": "Kalbertodt",
    //       "username": "LukasKalbertodt"
    //     },
    //     "date": 1435244203,
    //     "forward_from": {
    //       "id": 10742080,
    //       "first_name": "Lukas",
    //       "last_name": "Kalbertodt",
    //       "username": "LukasKalbertodt"
    //     },
    //     "forward_date": 1435243061
    //   }
    // }
    // "#);
    // println!("{:?}", u);
}
