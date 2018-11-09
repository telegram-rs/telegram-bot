extern crate futures;
extern crate telegram_bot;
extern crate tokio;

use std::env;

use futures::{future::lazy, Stream};

use telegram_bot::*;

fn main() {
    let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();
    runtime
        .block_on(lazy(|| {
            let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
            let api = Api::configure(token).build().unwrap();

            // Convert stream to the stream with errors in result
            let stream = api.stream().then(|mb_update| {
                let res: Result<Result<Update, Error>, ()> = Ok(mb_update);
                res
            });

            // Print update or error for each update.
            tokio::executor::current_thread::spawn(stream.for_each(|mb_update| {
                println!("{:?}", mb_update);

                Ok(())
            }));

            Ok::<_, ()>(())
        }))
        .unwrap();

    runtime.run().unwrap();
}
