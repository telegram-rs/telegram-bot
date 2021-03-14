use actix_web::{error, web, App, Error, HttpResponse, HttpServer};
use futures::StreamExt;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use telegram_bot::{
    types::{requests::SendMessage, MessageKind, Update, UpdateKind},
    Api,
};

async fn handler(mut payload: web::Payload, api: web::Data<Api>) -> Result<HttpResponse, Error> {
    // parse json now
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk.map_err(|e| error::ErrorBadRequest(e.to_string()))?;

        if (body.len() + chunk.len()) > 262_144_usize {
            return Err(error::ErrorBadRequest(""));
        }

        body.extend_from_slice(&chunk);
    }

    let update = serde_json::from_slice::<Update>(&body)?;

    // start your business here
    if let UpdateKind::Message(message) = update.kind {
        if let MessageKind::Text { ref data, .. } = message.kind {
            // Print received text message to stdout.
            println!("<{}>: {}", &message.from.first_name, data);

            // Answer message with "Hi".
            api.send(SendMessage::new(
                message.chat,
                format!(
                    "Hi, {}! You just wrote '{}'",
                    &message.from.first_name, data
                ),
            ))
            .await
            .map_err(|_| Error::from(HttpResponse::BadRequest()))?;
        }
    }

    Ok(HttpResponse::Ok().body(""))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // SSL builder
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // read private key
    builder
        .set_private_key_file("./vault/key.pem", SslFiletype::PEM)
        .unwrap();
    // read certificate
    builder
        .set_certificate_chain_file("./vault/certs.pem")
        .unwrap();

    // declare endpoint
    let endpoint = include_str!("../vault/endpoint");

    // read token
    let token = include_str!("../vault/telebottoken")
        .lines()
        .next()
        .unwrap();

    // tracing
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_env_filter("telegram_bot=trace")
            .finish(),
    )
    .unwrap();

    // start http server
    HttpServer::new(move || {
        App::new()
            .data(Api::new(token))
            .route(endpoint, web::post().to(handler))
    })
    .bind_openssl("0.0.0.0:8443", builder)?
    .run()
    .await
}
