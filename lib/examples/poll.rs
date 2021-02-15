use std::env;
use std::time::Duration;

use futures::StreamExt;
use tokio::time::sleep;

use telegram_bot::prelude::*;
use telegram_bot::{
    Api, Error, Message, MessageKind, Poll, PollAnswer, SendPoll, UpdateKind, User,
};

fn make_test_poll<'p>(message: Message) -> SendPoll<'p, 'p, 'p> {
    let question = "Simple poll";
    let options = vec!["Option 1", "Option 2"];

    message.poll_reply(question, options)
}

async fn send_and_stop_poll<'p>(api: Api, poll: SendPoll<'p, 'p, 'p>) -> Result<(), Error> {
    let poll_message = api.send(poll).await?;

    sleep(Duration::from_secs(10)).await;

    api.send(poll_message.stop_poll()).await?;
    Ok(())
}

async fn test_anonymous_poll(api: Api, message: Message) -> Result<(), Error> {
    let poll = make_test_poll(message);
    send_and_stop_poll(api, poll).await
}

async fn test_public_poll(api: Api, message: Message) -> Result<(), Error> {
    let poll = make_test_poll(message.clone()).not_anonymous().to_owned();

    send_and_stop_poll(api, poll).await
}

async fn test_quiz_poll(api: Api, message: Message) -> Result<(), Error> {
    let poll = make_test_poll(message.clone())
        .quiz()
        .correct_option_id(0)
        .explanation("Some explanation")
        .to_owned();

    send_and_stop_poll(api, poll).await
}

async fn test_multiple_answers(api: Api, message: Message) -> Result<(), Error> {
    let poll = make_test_poll(message.clone())
        .allows_multiple_answers()
        .to_owned();

    send_and_stop_poll(api, poll).await
}

async fn test_closed_poll(api: Api, message: Message) -> Result<(), Error> {
    let poll = make_test_poll(message.clone()).closed().to_owned();

    api.send(poll).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");

    let api = Api::new(token);
    let mut stream = api.stream();

    while let Some(update) = stream.next().await {
        let update = update?;

        match update.kind {
            UpdateKind::Message(message) => match message.kind {
                MessageKind::Text { ref data, .. } => match data.as_str() {
                    "/poll" => test_anonymous_poll(api.clone(), message).await?,
                    "/quiz" => test_quiz_poll(api.clone(), message).await?,
                    "/public" => test_public_poll(api.clone(), message).await?,
                    "/multiple" => test_multiple_answers(api.clone(), message).await?,
                    "/closed" => test_closed_poll(api.clone(), message).await?,
                    _ => (),
                },
                _ => (),
            },
            UpdateKind::Poll(Poll {
                total_voter_count,
                id,
                ..
            }) => println!(
                "Poll update - {} with total voters {}",
                id, total_voter_count
            ),
            UpdateKind::PollAnswer(PollAnswer {
                poll_id,
                user: User { first_name, .. },
                option_ids,
            }) => println!(
                "In poll {} {} voted for {:?}",
                poll_id, first_name, option_ids
            ),
            _ => (),
        }
    }

    Ok(())
}
