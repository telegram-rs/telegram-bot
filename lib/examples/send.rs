use std::env;

use futures::StreamExt;

use telegram_bot::prelude::*;
use telegram_bot::{Api, Error, InputFileRef, InputFileUpload, Message, MessageKind, UpdateKind};

async fn run_test(api: Api, message: Message) -> Result<(), Error> {
    let chat = message.chat.clone();

    // Send a document from memory
    let document = "foo\nbar\nbaz".as_bytes();
    let file = InputFileUpload::with_data(document, "doc.txt");

    api.send(message.document_reply(&file).caption("Reply to message"))
        .await?;
    api.send(chat.document(&file).caption("Direct to chat"))
        .await?;
    api.send(message.from.document(&file).caption("Send to user"))
        .await?;

    // With custom thumbnail
    api.send(
        chat.document(&file)
            .thumb(InputFileUpload::with_path("data/thumb.jpg")),
    )
    .await?;

    // Send a document from disk
    let file = InputFileUpload::with_path("data/image.jpg");
    api.send(chat.document(&file)).await?;

    // With custom name
    api.send(chat.document(file.file_name("picture.png")))
        .await?;

    // Send an image from disk
    api.send(chat.photo(&file)).await?;

    // Send an audio file from disk
    let file = InputFileUpload::with_path("data/sound.mp3");
    let resp = api.send(chat.audio(file)).await?;

    // Resend an audio file by file_id
    if let MessageKind::Audio { data } = resp.kind {
        api.send(chat.audio(InputFileRef::new(data.file_id)))
            .await?;
    }

    // Send an image by url
    api.send(chat.photo(InputFileRef::new("https://telegram.org/img/t_logo.png")))
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");

    let api = Api::new(token);
    let mut stream = api.stream();

    while let Some(update) = stream.next().await {
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            match message.kind {
                MessageKind::Text { ref data, .. } if data.as_str() == "/test" => {
                    let api = api.clone();
                    tokio::spawn(async move {
                        if let Err(err) = run_test(api, message).await {
                            eprintln!("Error: {:?}", err);
                        }
                    });
                }
                _ => (),
            };
        }
    }

    Ok(())
}
