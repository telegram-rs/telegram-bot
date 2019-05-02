extern crate serde;
extern crate serde_json;

extern crate telegram_bot_raw;

use std::fs::File;
use std::io::prelude::*;

use telegram_bot_raw::types::message::MessageKind;
use telegram_bot_raw::types::update::{Update, UpdateKind};

macro_rules! make_test {
    ($asset: ident, $test: expr) => {
        #[test]
        fn $asset() {
            let data = {
                let filename = format!("tests/update_assets/{}.json", stringify!($asset));
                let mut data = Vec::new();
                let mut file = File::open(filename).unwrap();
                file.read_to_end(&mut data).unwrap();
                data
            };
            let update = serde_json::from_slice::<Update>(&data).unwrap();
            $test(update)
        }
    };
}

make_test!(migrate_from_chat_id, |update: Update| {
    if let UpdateKind::Message(message) = update.kind {
        if let MessageKind::MigrateFromChatId { .. } = message.kind {
            return ();
        }
    }
    assert!(false)
});

make_test!(migrate_to_chat_id, |update: Update| {
    if let UpdateKind::Message(message) = update.kind {
        if let MessageKind::MigrateToChatId { .. } = message.kind {
            return ();
        }
    }
    assert!(false)
});
