Rust Telegram Bot Library
=========================
[![Build Status](https://img.shields.io/travis/LukasKalbertodt/telegram-bot/master.svg)](https://travis-ci.org/LukasKalbertodt/telegram-bot)
[![License](https://img.shields.io/github/license/LukasKalbertodt/telegram-bot.svg)]()
[![Crates.io](https://img.shields.io/crates/v/telegram-bot.svg)](https://crates.io/crates/telegram-bot)

[**Documentation**](https://lukaskalbertodt.github.io/telegram-bot/telegram_bot/)

A library for writing your own [Telegram](https://telegram.org/) bots. More information [here](https://core.telegram.org/bots). Official API [here](https://core.telegram.org/bots/api). **Note:** It's usable, but not feature complete yet.

## Example
Here is a simple example (see [`example/simple.rs`](https://github.com/LukasKalbertodt/telegram-bot/blob/master/examples/simple.rs)):

``` rust
extern crate telegram_bot;

use telegram_bot::*;

fn main() {
    // Create bot, test simple API call and print bot information
    let api = Api::from_env("TELEGRAM_BOT_TOKEN").unwrap();
    println!("getMe: {:?}", api.get_me());
    let mut listener = api.listener(ListeningMethod::LongPoll(None));

    // Fetch new updates via long poll method
    let res = listener.listen(|u| {
        // If the received update contains a message...
        if let Some(m) = u.message {
            let name = m.from.first_name;

            // Match message type
            match m.msg {
                MessageType::Text(t) => {
                    // Print received text message to stdout
                    println!("<{}> {}", name, t);

                    if t == "/exit" {
                        return Ok(ListeningAction::Stop);
                    }

                    // Answer message with "Hi"
                    try!(api.send_message(
                        m.chat.id(),
                        format!("Hi, {}! You just wrote '{}'", name, t),
                        None, None, None, None));
                },
                _ => {}
            }
        }

        // If none of the "try!" statements returned an error: It's Ok!
        Ok(ListeningAction::Continue)
    });

    if let Err(e) = res {
        println!("An error occured: {}", e);
    }
}
```
You can find a bigger example in the `examples` folder and run them like this:

```bash
TELEGRAM_BOT_TOKEN=XXXXXXXXXXXXXXXXXXXXXXXXXxx cargo run --example features
```

## Usage
This library is available via `crates.io`. In order to use it, just add this to your `Cargo.toml`:

```
telegram-bot = "0.4"
```

## Collaboration
Yes please! Every type of contribution is welcome: Create issues, hack some code or make suggestions. If you don't know where to start, just contact me (my email is on my github profile).

Please submit pull request against the `master` branch, unless all changes are just documentation fixes.

## Todo

- [x] "getMe"
- [x] Methods without files
  - [x] "getMe"
  - [x] "sendMessage"
  - [x] "forwardMessage"
  - [x] "sendLocation"
  - [x] "sendChatAction"
  - [x] "getUserProfilePhotos"
- [x] "getUpdates" and `long_poll`
- [ ] "setWebhook" and `listen`
- [x] sending files ("sendAudio", "sendDocument", ...)
- [x] More good documentation and examples
- [x] Maybe think about multithreading stuff
