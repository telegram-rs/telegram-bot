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

    // let keyboard = ReplyKeyboardMarkup {
    //     keyboard: vec![vec!["Hi".into()],
    //                    vec!["A".into(), "B".into()]],
    //     .. Default::default()
    // };

    // println!("{}", json::encode(&keyboard).unwrap());

    let res = bot.long_poll(None, |bot, u| {
        if let Some(m) = u.message {
            let name = m.from.first_name + &*m.from.last_name
                .map_or("".to_string(), |mut n| { n.insert(0, ' '); n });
            let chat_id = m.chat.id();

            match m.msg {
                MessageType::Text(t) => {
                    println!("<{}> {}", name, t);
                    let keyboard = ReplyKeyboardMarkup {
                        keyboard: vec![vec![t],
                                       vec!["A".into(), "B".into()]],
                       one_time_keyboard: Some(true),
                        .. Default::default()
                    };

                    try!(bot.send_chat_action(chat_id, ChatAction::Typing));
                    // try!(bot.send_message(
                    //     chat_id,
                    //     format!("Hi, {}", name),
                    //     None,
                    //     None,
                    //     Some(keyboard)));
                },
                MessageType::Location(loc) => {
                    // Print event
                    println!("<{}> is here: {}", name,
                        json::encode(&loc).unwrap());

                    // Calculate and send the location on the other side of the
                    // earth.
                    let lat = -loc.latitude;
                    let lng = if loc.longitude > 0.0 {
                        loc.longitude - 180.0
                    } else {
                        loc.longitude + 180.0
                    };

                    try!(bot.send_location(chat_id, lat, lng, None, None));
                },
                MessageType::Contact(c) => {
                    // Print event
                    println!("<{}> send a contact: {}", name,
                        json::encode(&c).unwrap());

                    // Just forward the contact back to the sender...
                    try!(bot.forward_message(chat_id, chat_id, m.message_id));
                }
                _ => {}
            }

        }
        Ok(())
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
