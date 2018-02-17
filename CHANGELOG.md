# Change Log
All notable changes to this project will be documented in this file.

## 0.6.1 - 2018-02-17

### Fixes
- Re-export forgotten types ([#85](https://github.com/telegram-rs/telegram-bot/issues/85))

### Features
- pinChatMessage and unpinChatMessage methods
- Bots can now send and receive Live Locations ([#83](https://github.com/telegram-rs/telegram-bot/issues/83))

## 0.6.0 - 2018-01-09

Rewritten from scratch.

## 0.5.0 - 2016-10-21

### Fixed
- Update dependencies.
- Handle unknown messages.

## 0.4.1 - 2016-02-25

### Fixed
- Fix a bug with broken forward messages.

## 0.4.0 - 2016-02-18

### Added
- Supergroups support.
- `ParseMode` structure.

### Changed
- `Integer` type to be an alias to i64 instead of i32 because of supergroups.
- New `parse_mode` parameter in `API::send_message` method.
- `Chat` enum to support supergroups and channels.
- Specified dependencies versions in Cargo.toml.

### Fixed
- Update type of `user_id` field in `Contact` struct
- Handling of replies to a message.

## 0.3.0 - 2015-08-29

## 0.2.0 - 2015-08-10

## 0.1.2 - 2015-07-30

### Changed
- `Api::long_poll` method to take `FnMut` instead of `Fn`.

## 0.1.1 - 2015-07-26

## 0.1.0 - 2015-06-30

- Initial release
