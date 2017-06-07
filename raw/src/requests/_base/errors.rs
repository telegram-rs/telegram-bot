use types::*;

error_chain! {
    foreign_links {
        Json(::serde_json::Error);
    }

    errors {
        EmptyBody
        TelegramError {
            description: String,
            parameters: Option<ResponseParameters>
        }
        DetachedError(err: String)
    }
}
