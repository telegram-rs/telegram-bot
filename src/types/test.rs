#![allow(unused_imports)]
use rustc_serialize::json;

#[test]
fn reply_keyboard_markup() {
    use ReplyKeyboardMarkup as RKM;

    // Test default values
    assert_eq!(RKM::default(), RKM {
        keyboard: Vec::new(),
        resize_keyboard: None,
        one_time_keyboard: None,
        selective: None,
    });

    // Test encoding
    let x = RKM::default();
    assert_eq!(json::encode(&x).unwrap(), "{\"keyboard\":[]}".to_string());

    let x = RKM { resize_keyboard: Some(true), ..Default::default() };
    assert_eq!(json::encode(&x).unwrap(),
        "{\"keyboard\":[],\"resize_keyboard\":true}".to_string());

    let x = RKM {
        keyboard: vec![vec!["ABC".into()], vec!["X".into(), "Y".into()]],
        resize_keyboard: Some(false),
        ..Default::default()
    };
    assert_eq!(json::encode(&x).unwrap(),
        "{\"keyboard\":[[\"ABC\"],[\"X\",\"Y\"]],\"resize_keyboard\":false}".to_string());
}

#[test]
fn keyboard_markup() {
    use ReplyKeyboardMarkup as RKM;
    use ReplyMarkup as RM;

    // Test encoding
    let x = RM::Keyboard(RKM::default());
    assert_eq!(json::encode(&x).unwrap(), "{\"keyboard\":[]}".to_string());

    let x = RM::KeyboardHide(false);
    assert_eq!(json::encode(&x).unwrap(),
        "{\"hide_keyboard\":true,\"selective\":false}".to_string());

    let x = RM::ForceReply(true);
    assert_eq!(json::encode(&x).unwrap(),
        "{\"force_reply\":true,\"selective\":true}".to_string());
}

#[test]
fn decode_group_chat() {
    use Chat;
    use GroupChat;

    let blob = r#"{"title":"This is a group chat","id":-12345678}"#;
    let groupchat: GroupChat = json::decode(&blob).unwrap();
    let chat: Chat = json::decode(&blob).unwrap();

    assert!(chat.is_group());
    assert_eq!(Chat::Group(groupchat), chat);
}

#[test]
fn decode_user_chat() {
    use Chat;
    use User;

    let blob = r#"{"first_name":"test","id":123456789,"username":"test"}"#;
    let chat: Chat = json::decode(&blob).unwrap();
    let user: User = json::decode(&blob).unwrap();

    assert!(chat.is_user());
    assert_eq!(Chat::User(user), chat);
}
