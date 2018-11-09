extern crate futures;
extern crate telegram_bot;
extern crate tokio;

use std::env;

use futures::{future::lazy, Future};

use telegram_bot::*;

fn main() {
    let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();
    runtime
        .block_on(lazy(|| {
            let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
            let api = Api::configure(token).build().unwrap();

            tokio::executor::current_thread::spawn(api.send(GetMe).then(|r| {
                println!("{:?}", r);

                Ok(())
            }));

            Ok::<_, Error>(())
        }))
        .unwrap();

    runtime.run().unwrap();
}
