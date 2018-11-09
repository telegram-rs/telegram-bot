extern crate futures;
extern crate telegram_bot;
extern crate tokio;
extern crate tokio_timer;

use std::env;
use std::ops::Add;
use std::time::{Duration, Instant};

use futures::{future::lazy, Future, Stream};

use tokio_timer::Delay;

use telegram_bot::*;

fn test_message(api: Api, message: Message) {
    let simple = api.send(message.text_reply("Simple message"));

    let markdown = api.send(
        message
            .text_reply("`Markdown message`")
            .parse_mode(ParseMode::Markdown),
    );

    let html = api.send(
        message
            .text_reply("<b>Bold HTML message</b>")
            .parse_mode(ParseMode::Html),
    );

    tokio::executor::current_thread::spawn({
        let future = simple.and_then(|_| markdown).and_then(|_| html);

        future.map_err(|_| ()).map(|_| ())
    })
}

fn test_preview(api: Api, message: Message) {
    let preview = api.send(message.text_reply("Message with preview https://telegram.org"));

    let no_preview = api.send(
        message
            .text_reply("Message without preview https://telegram.org")
            .disable_preview(),
    );

    tokio::executor::current_thread::spawn({
        let future = preview.and_then(|_| no_preview);

        future.map_err(|_| ()).map(|_| ())
    })
}

fn test_reply(api: Api, message: Message) {
    let msg = api.send(message.text_reply("Reply to message"));
    let chat = api.send(message.chat.text("Text to message chat"));

    let private = api.send(message.from.text("Private text"));

    tokio::executor::current_thread::spawn({
        let future = msg.and_then(|_| chat).and_then(|_| private);

        future.map_err(|_| ()).map(|_| ())
    })
}

fn test_forward(api: Api, message: Message) {
    api.spawn(message.forward(&message.chat));

    api.spawn(message.forward(&message.from))
}

fn test_edit_message(api: Api, message: Message) {
    let round_1 = api.send(message.text_reply("Round 1"));

    let duration_1 = Duration::from_secs(2);

    let sleep_1 = Delay::new(Instant::now().add(duration_1)).map_err(From::from);

    let round_2_api = api.clone();
    let round_2 = round_1
        .join(sleep_1)
        .and_then(move |(message, _)| round_2_api.send(message.edit_text("Round 2")));

    let duration_2 = Duration::from_secs(4);
    let sleep_2 = Delay::new(Instant::now().add(duration_2)).map_err(From::from);

    let round_3 = round_2
        .join(sleep_2)
        .map_err(|_| ())
        .and_then(move |(message, _)| {
            api.spawn(message.edit_text("Round 3"));
            Ok(())
        });

    tokio::executor::current_thread::spawn(round_3)
}

fn test_get_chat(api: Api, message: Message) {
    let chat = api.send(message.chat.get_chat());
    let future = chat.and_then(move |chat| api.send(chat.text(format!("Chat id {}", chat.id()))));

    tokio::executor::current_thread::spawn({ future.map_err(|_| ()).map(|_| ()) })
}

fn test_get_chat_administrators(api: Api, message: Message) {
    let administrators = api.send(message.chat.get_administrators());
    let future = administrators.and_then(move |administrators| {
        let mut response = Vec::new();
        for member in administrators {
            response.push(member.user.first_name.clone())
        }
        api.send(message.text_reply(format!("Administrators: {}", response.join(", "))))
    });

    tokio::executor::current_thread::spawn({ future.map_err(|_| ()).map(|_| ()) })
}

fn test_get_chat_members_count(api: Api, message: Message) {
    let count = api.send(message.chat.get_members_count());
    let future = count
        .and_then(move |count| api.send(message.text_reply(format!("Members count: {}", count))));

    tokio::executor::current_thread::spawn({ future.map_err(|_| ()).map(|_| ()) })
}

fn test_get_chat_member(api: Api, message: Message) {
    let member = api.send(message.chat.get_member(&message.from));
    let future = member.and_then(move |member| {
        let first_name = member.user.first_name.clone();
        let status = member.status;
        api.send(message.text_reply(format!("Member {}, status {:?}", first_name, status)))
    });

    tokio::executor::current_thread::spawn({ future.map_err(|_| ()).map(|_| ()) })
}

fn test_get_user_profile_photos(api: Api, message: Message) {
    let photos = api.send(message.from.get_user_profile_photos());

    let future = photos.and_then(move |photos| {
        api.send(message.text_reply(format!("Found photos: {}", photos.total_count)))
    });

    tokio::executor::current_thread::spawn({ future.map_err(|_| ()).map(|_| ()) })
}

fn test_leave(api: Api, message: Message) {
    api.spawn(message.chat.leave())
}

fn test(api: Api, message: Message) {
    let function: fn(Api, Message) = match message.kind {
        MessageKind::Text { ref data, .. } => match data.as_str() {
            "/message" => test_message,
            "/preview" => test_preview,
            "/reply" => test_reply,
            "/forward" => test_forward,
            "/edit-message" => test_edit_message,
            "/get_chat" => test_get_chat,
            "/get_chat_administrators" => test_get_chat_administrators,
            "/get_chat_members_count" => test_get_chat_members_count,
            "/get_chat_member" => test_get_chat_member,
            "/get_user_profile_photos" => test_get_user_profile_photos,
            "/leave" => test_leave,
            _ => return,
        },
        _ => return,
    };

    function(api, message)
}

fn main() {
    let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();
    runtime
        .block_on(lazy(|| {
            let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
            let api = Api::configure(token).build().unwrap();

            let stream = api.stream().then(|mb_update| {
                let res: Result<Result<Update, Error>, ()> = Ok(mb_update);
                res
            });

            tokio::executor::current_thread::spawn(stream.for_each(move |update| {
                match update {
                    Ok(update) => {
                        if let UpdateKind::Message(message) = update.kind {
                            test(api.clone(), message)
                        }
                    }
                    Err(_) => {}
                }

                Ok(())
            }));

            Ok::<_, ()>(())
        }))
        .unwrap();

    runtime.run().unwrap();
}
