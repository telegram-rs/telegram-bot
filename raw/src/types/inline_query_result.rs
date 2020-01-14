use crate::types::*;
use std::ops::Not;

/// This object represents an incoming inline query.
/// When the user sends an empty query, your bot could return some default or trending results.
#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum InlineQueryResult {
    /// Represents a link to an mp3 audio file stored on the Telegram servers.
    /// By default, this audio file will be sent by the user.
    /// Alternatively, you can use input_message_content to send a message with the specified
    /// content instead of the audio.
    #[serde(rename = "audio")]
    InlineQueryResultCachedAudio(InlineQueryResultCachedAudio),
    /// Represents a link to a file stored on the Telegram servers.
    /// By default, this file will be sent by the user with an optional caption.
    /// Alternatively, you can use input_message_content to send a message with the specified
    /// content instead of the file.
    #[serde(rename = "document")]
    InlineQueryResultCachedDocument(InlineQueryResultCachedDocument),
    /// Represents a link to an animated GIF file stored on the Telegram servers.
    /// By default, this animated GIF file will be sent by the user with an optional caption.
    /// Alternatively, you can use input_message_content to send a message with specified content
    /// instead of the animation.
    #[serde(rename = "gif")]
    InlineQueryResultCachedGif(InlineQueryResultCachedGif),
    /// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound) stored on the
    /// Telegram servers.
    /// By default, this animated MPEG-4 file will be sent by the user with an optional caption.
    /// Alternatively, you can use input_message_content to send a message with the specified
    /// content instead of the animation.
    #[serde(rename = "mpeg4gif")]
    InlineQueryResultCachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
    /// Represents a link to a photo stored on the Telegram servers.
    /// By default, this photo will be sent by the user with an optional caption.
    /// Alternatively, you can use input_message_content to send a message with the specified
    /// content instead of the photo.
    #[serde(rename = "photo")]
    InlineQueryResultCachedPhoto(InlineQueryResultCachedPhoto),
    /// Represents a link to a sticker stored on the Telegram servers.
    /// By default, this sticker will be sent by the user.
    /// Alternatively, you can use input_message_content to send a message with the specified
    /// content instead of the sticker.
    #[serde(rename = "sticker")]
    InlineQueryResultCachedSticker(InlineQueryResultCachedSticker),
    /// Represents a link to a video file stored on the Telegram servers.
    /// By default, this video file will be sent by the user with an optional caption.
    /// Alternatively, you can use input_message_content to send a message with the specified
    /// content instead of the video.
    #[serde(rename = "video")]
    InlineQueryResultCachedVideo(InlineQueryResultCachedVideo),
    /// Represents a link to a voice message stored on the Telegram servers.
    /// By default, this voice message will be sent by the user.
    /// Alternatively, you can use input_message_content to send a message with the specified
    /// content instead of the voice message.
    #[serde(rename = "voice")]
    InlineQueryResultCachedVoice(InlineQueryResultCachedVoice),
    /// Represents a link to an article or web page.
    #[serde(rename = "article")]
    InlineQueryResultArticle(InlineQueryResultArticle),
    /// Represents a link to an mp3 audio file.
    /// By default, this audio file will be sent by the user.
    /// Alternatively, you can use input_message_content to send a message with the specified
    /// content instead of the audio.
    #[serde(rename = "audio")]
    InlineQueryResultAudio(InlineQueryResultAudio),
    /// Represents a contact with a phone number.
    /// By default, this contact will be sent by the user.
    /// Alternatively, you can use input_message_content to send a message with the specified
    /// content instead of the contact.
    #[serde(rename = "contact")]
    InlineQueryResultContact(InlineQueryResultContact),
    /// Represents a Game.
    #[serde(rename = "game")]
    InlineQueryResultGame(InlineQueryResultGame),
    /// Represents a link to a file.
    /// By default, this file will be sent by the user with an optional caption.
    /// Alternatively, you can use input_message_content to send a message with the specified
    /// content instead of the file. Currently, only .PDF and .ZIP files can be sent using this
    /// method.
    #[serde(rename = "document")]
    InlineQueryResultDocument(InlineQueryResultDocument),
    /// Represents a link to an animated GIF file.
    /// By default, this animated GIF file will be sent by the user with optional caption.
    /// Alternatively, you can use input_message_content to send a message with the specified
    /// content instead of the animation.
    #[serde(rename = "gif")]
    InlineQueryResultGif(InlineQueryResultGif),
    /// Represents a location on a map.
    /// By default, the location will be sent by the user.
    /// Alternatively, you can use input_message_content to send a message with the specified
    /// content instead of the location.
    #[serde(rename = "location")]
    InlineQueryResultLocation(InlineQueryResultLocation),
    /// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound).
    /// By default, this animated MPEG-4 file will be sent by the user with optional caption.
    /// Alternatively, you can use input_message_content to send a message with the specified content
    /// instead of the animation.
    #[serde(rename = "mpeg4_gif")]
    InlineQueryResultMpeg4Gif(InlineQueryResultMpeg4Gif),
    /// Represents a link to a photo.
    /// By default, this photo will be sent by the user with optional caption.
    /// Alternatively, you can use input_message_content to send a message with the specified content
    /// instead of the photo.
    #[serde(rename = "photo")]
    InlineQueryResultPhoto(InlineQueryResultPhoto),
    /// Represents a venue.
    /// By default, the venue will be sent by the user.
    /// Alternatively, you can use input_message_content to send a message with the specified content
    /// instead of the venue.
    #[serde(rename = "venue")]
    InlineQueryResultVenue(InlineQueryResultVenue),
    /// Represents a link to a page containing an embedded video player or a video file.
    /// By default, this video file will be sent by the user with an optional caption.
    /// Alternatively, you can use input_message_content to send a message with the specified content
    /// instead of the video.
    #[serde(rename = "video")]
    InlineQueryResultVideo(InlineQueryResultVideo),
    /// Represents a link to a voice recording in an .ogg container encoded with OPUS.
    /// By default, this voice recording will be sent by the user.
    /// Alternatively, you can use input_message_content to send a message with the specified content
    /// instead of the the voice message.
    #[serde(rename = "voice")]
    InlineQueryResultVoice(InlineQueryResultVoice),
}

#[derive(Serialize, Debug)]
pub struct InlineQueryResultArticle {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Title of the result
    pub title: String,
    /// Content of the message to be sent
    pub input_message_content: InputMessageContent,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// URL of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Pass True, if you don't want the URL to be shown in the message
    #[serde(skip_serializing_if = "Not::not")]
    pub hide_url: bool,
    /// Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    /// Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<Integer>,
    /// Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<Integer>,
}

#[derive(Serialize, Debug)]
pub struct InlineQueryResultPhoto {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid URL of the photo. Photo must be in jpeg format. Photo size must not exceed 5MB
    pub photo_url: String,
    /// Url of the thumbnail for the result
    pub thumb_url: String,
    /// Width of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<Integer>,
    /// Height of hte photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<Integer>,
    /// Title of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Caption of the photo to be sent, 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Serialize, Debug)]
pub struct InlineQueryResultGif {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid URL for the GIF file. File size must not exceed 1MB
    pub gif_url: String,
    /// Width of the GIF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_width: Option<Integer>,
    /// Heightof the GIF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_height: Option<Integer>,
    /// Duration of the GIF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_duration: Option<Integer>,
    /// Url of the thumbnail for the result (jpeg or gif)
    pub thumb_url: String,
    /// Title of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Caption of the GIF file to be sent, 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Serialize, Debug)]
pub struct InlineQueryResultMpeg4Gif {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid URL for the MP4 file. File size must not exceed 1MB
    pub mpeg4_url: String,
    /// Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_width: Option<Integer>,
    /// Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_height: Option<Integer>,
    /// Video duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_duration: Option<Integer>,
    /// Url of the thumbnail (jpeg or gif) for the result
    pub thumb_url: String,
    /// Title of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Serialize, Debug)]
pub struct InlineQueryResultVideo {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid URL for the embedded video player or video file
    pub video_url: String,
    /// Mime type of the content of video url, “text/html” or “video/mp4”
    pub mime_type: String,
    /// Url of the thumbnail (jpeg only) for the video
    pub thumb_url: String,
    /// Title of the result
    pub title: String,
    /// Caption of the video to be sent, 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_width: Option<Integer>,
    /// Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_height: Option<Integer>,
    /// Video duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_duration: Option<Integer>,
    /// Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Serialize, Debug)]
pub struct InlineQueryResultAudio {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid URL for the audio file
    pub audio_url: String,
    /// Title of the result
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Performer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Audio duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_duration: Option<Integer>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Serialize, Debug)]
pub struct InlineQueryResultVoice {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    pub voice_url: String,
    /// Title of the result
    pub title: String,
    /// Caption, 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Audio duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_duration: Option<Integer>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Serialize, Debug)]
pub struct InlineQueryResultDocument {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Title of the result
    pub title: String,
    /// Caption, 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// A valid URL for the file
    pub document_url: String,
    /// Mime type of the content of the file, either “application/pdf” or “application/zip”
    pub mime_type: String,
    /// Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    /// Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<Integer>,
    /// Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<Integer>,
}

#[derive(Serialize, Debug)]
pub struct InlineQueryResultLocation {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Location latitude in degrees
    pub latitude: Float,
    /// Location longitude in degrees
    pub longitude: Float,
    /// Location title
    pub title: String,
    /// Period in seconds for which the location can be updated, should be between 60 and 86400
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<Integer>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    /// Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<Integer>,
    /// Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<Integer>,
}

#[derive(Serialize, Debug)]
pub struct InlineQueryResultVenue {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Latitude of the venu location in degrees
    pub latitude: Float,
    /// Longitude of the venue location in degrees
    pub longitude: Float,
    /// Title of the result
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue if known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known. (For example, “arts_entertainment/default”,
    /// “arts_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    /// Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<Integer>,
    /// Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<Integer>,
}

#[derive(Serialize, Debug)]
pub struct InlineQueryResultContact {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    pub last_name: String,
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    pub vcard: String,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    /// Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<Integer>,
    /// Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<Integer>,
}

#[derive(Serialize, Debug)]
pub struct InlineQueryResultGame {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Short name of the game
    pub game_short_name: String,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Serialize, Debug)]
pub struct InlineQueryResultCachedPhoto {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid file identifier of the photo
    pub photo_file_id: String,
    /// Title of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Caption of the photo to be sent, 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Serialize, Debug)]
pub struct InlineQueryResultCachedGif {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid file identifier for the GIF file
    pub gif_file_id: String,
    /// Title of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Caption of the GIF to be sent, 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Serialize, Debug)]
pub struct InlineQueryResultCachedMpeg4Gif {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid file identifier for the MP4 file
    pub mpeg4_file_id: String,
    /// Title of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Caption of the MPEG-4 file to be sent, 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Serialize, Debug)]
pub struct InlineQueryResultCachedSticker {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid file identifier of the sticker
    pub sticker_file_id: String,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Serialize, Debug)]
pub struct InlineQueryResultCachedDocument {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Title of the result
    pub title: String,
    /// A valid file identifier for the file
    pub document_file_id: String,
    /// Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Caption of the document to be sent, 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Serialize, Debug)]
pub struct InlineQueryResultCachedVideo {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid file identifier for the video
    pub video_file_id: String,
    /// Title of the result
    pub title: String,
    /// Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Caption of the video to be sent, 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Serialize, Debug)]
pub struct InlineQueryResultCachedVoice {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid file identifier for the audio file
    pub voice_file_id: String,
    /// Voice message title
    pub title: String,
    /// Caption, 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Serialize, Debug)]
pub struct InlineQueryResultCachedAudio {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid file identifier for the audio file
    pub audio_file_id: String,
    /// Caption, 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultArticle {
    pub fn new<T: Into<String>, U: Into<String>, V: Into<InputMessageContent>>(
        id: T,
        title: U,
        input_message_content: V,
    ) -> InlineQueryResultArticle {
        InlineQueryResultArticle {
            id: id.into(),
            title: title.into(),
            input_message_content: input_message_content.into(),
            reply_markup: None,
            url: None,
            hide_url: false,
            description: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }

    pub fn reply_markup<T: Into<InlineKeyboardMarkup>>(&mut self, reply_markup: T) -> &mut Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }

    pub fn url<T: Into<String>>(&mut self, url: T) -> &mut Self {
        self.url = Some(url.into());
        self
    }

    pub fn hide_url<T: Into<bool>>(&mut self, hide_url: T) -> &mut Self {
        self.hide_url = hide_url.into();
        self
    }

    pub fn description<T: Into<String>>(&mut self, description: T) -> &mut Self {
        self.description = Some(description.into());
        self
    }

    pub fn thumb_url<T: Into<String>>(&mut self, thumb_url: T) -> &mut Self {
        self.thumb_url = Some(thumb_url.into());
        self
    }

    pub fn thumb_width<T: Into<Integer>>(&mut self, thumb_width: T) -> &mut Self {
        self.thumb_width = Some(thumb_width.into());
        self
    }

    pub fn thumb_height<T: Into<Integer>>(&mut self, thumb_height: T) -> &mut Self {
        self.thumb_height = Some(thumb_height.into());
        self
    }
}

// TODO: builders for InlineQueryResult types

impl From<InlineQueryResultCachedAudio> for InlineQueryResult {
    fn from(audio: InlineQueryResultCachedAudio) -> Self {
        InlineQueryResult::InlineQueryResultCachedAudio(audio)
    }
}
impl From<InlineQueryResultCachedDocument> for InlineQueryResult {
    fn from(document: InlineQueryResultCachedDocument) -> Self {
        InlineQueryResult::InlineQueryResultCachedDocument(document)
    }
}
impl From<InlineQueryResultCachedGif> for InlineQueryResult {
    fn from(gif: InlineQueryResultCachedGif) -> Self {
        InlineQueryResult::InlineQueryResultCachedGif(gif)
    }
}
impl From<InlineQueryResultCachedMpeg4Gif> for InlineQueryResult {
    fn from(mpeg4_gif: InlineQueryResultCachedMpeg4Gif) -> Self {
        InlineQueryResult::InlineQueryResultCachedMpeg4Gif(mpeg4_gif)
    }
}
impl From<InlineQueryResultCachedPhoto> for InlineQueryResult {
    fn from(photo: InlineQueryResultCachedPhoto) -> Self {
        InlineQueryResult::InlineQueryResultCachedPhoto(photo)
    }
}
impl From<InlineQueryResultCachedSticker> for InlineQueryResult {
    fn from(sticker: InlineQueryResultCachedSticker) -> Self {
        InlineQueryResult::InlineQueryResultCachedSticker(sticker)
    }
}
impl From<InlineQueryResultCachedVideo> for InlineQueryResult {
    fn from(video: InlineQueryResultCachedVideo) -> Self {
        InlineQueryResult::InlineQueryResultCachedVideo(video)
    }
}
impl From<InlineQueryResultCachedVoice> for InlineQueryResult {
    fn from(voice: InlineQueryResultCachedVoice) -> Self {
        InlineQueryResult::InlineQueryResultCachedVoice(voice)
    }
}
impl From<InlineQueryResultArticle> for InlineQueryResult {
    fn from(article: InlineQueryResultArticle) -> Self {
        InlineQueryResult::InlineQueryResultArticle(article)
    }
}
impl From<InlineQueryResultAudio> for InlineQueryResult {
    fn from(audio: InlineQueryResultAudio) -> Self {
        InlineQueryResult::InlineQueryResultAudio(audio)
    }
}
impl From<InlineQueryResultContact> for InlineQueryResult {
    fn from(contact: InlineQueryResultContact) -> Self {
        InlineQueryResult::InlineQueryResultContact(contact)
    }
}
impl From<InlineQueryResultGame> for InlineQueryResult {
    fn from(game: InlineQueryResultGame) -> Self {
        InlineQueryResult::InlineQueryResultGame(game)
    }
}
impl From<InlineQueryResultDocument> for InlineQueryResult {
    fn from(document: InlineQueryResultDocument) -> Self {
        InlineQueryResult::InlineQueryResultDocument(document)
    }
}
impl From<InlineQueryResultGif> for InlineQueryResult {
    fn from(gif: InlineQueryResultGif) -> Self {
        InlineQueryResult::InlineQueryResultGif(gif)
    }
}
impl From<InlineQueryResultLocation> for InlineQueryResult {
    fn from(location: InlineQueryResultLocation) -> Self {
        InlineQueryResult::InlineQueryResultLocation(location)
    }
}
impl From<InlineQueryResultMpeg4Gif> for InlineQueryResult {
    fn from(mpeg4_gif: InlineQueryResultMpeg4Gif) -> Self {
        InlineQueryResult::InlineQueryResultMpeg4Gif(mpeg4_gif)
    }
}
impl From<InlineQueryResultPhoto> for InlineQueryResult {
    fn from(photo: InlineQueryResultPhoto) -> Self {
        InlineQueryResult::InlineQueryResultPhoto(photo)
    }
}
impl From<InlineQueryResultVenue> for InlineQueryResult {
    fn from(venue: InlineQueryResultVenue) -> Self {
        InlineQueryResult::InlineQueryResultVenue(venue)
    }
}
impl From<InlineQueryResultVideo> for InlineQueryResult {
    fn from(video: InlineQueryResultVideo) -> Self {
        InlineQueryResult::InlineQueryResultVideo(video)
    }
}
impl From<InlineQueryResultVoice> for InlineQueryResult {
    fn from(voice: InlineQueryResultVoice) -> Self {
        InlineQueryResult::InlineQueryResultVoice(voice)
    }
}

/// This object represents the content of a message to be sent as a result of an inline query.
/// Telegram clients currently support the following 4 types:
#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum InputMessageContent {
    /// Represents the content of a text message to be sent as the result of an inline query.
    InputTextMessageContent(InputTextMessageContent),
    /// Represents the content of a location message to be sent as the result of an inline query.
    InputLocationMessageContent(InputLocationMessageContent),
    /// Represents the content of a venue message to be sent as the result of an inline query.
    InputVenueMessageContent(InputVenueMessageContent),
    /// Represents the content of a contact message to be sent as the result of an inline query.
    InputContactMessageContent(InputContactMessageContent),
}

#[derive(Serialize, Debug)]
pub struct InputTextMessageContent {
    /// Text of the message to be sent, 1-4096 characters
    pub message_text: String,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Disables link previews for links in the sent message
    #[serde(skip_serializing_if = "Not::not")]
    pub disable_web_page_preview: bool,
}

#[derive(Serialize, Debug)]
pub struct InputLocationMessageContent {
    /// Latitude of the location in degrees
    pub latitude: Float,
    /// Longitude of the location in degrees
    pub longitude: Float,
    /// Period in seconds for which the location can be updated, should be between 60 and 86400.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<Integer>,
}

#[derive(Serialize, Debug)]
pub struct InputVenueMessageContent {
    /// Latitude of the venue in degrees
    pub latitude: Float,
    /// Longitude of the venue in degrees
    pub longitude: Float,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue, if known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known. (For example, “arts_entertainment/default”,
    /// “arts_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct InputContactMessageContent {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}

impl From<InputTextMessageContent> for InputMessageContent {
    fn from(value: InputTextMessageContent) -> Self {
        InputMessageContent::InputTextMessageContent(value)
    }
}

impl From<InputLocationMessageContent> for InputMessageContent {
    fn from(value: InputLocationMessageContent) -> Self {
        InputMessageContent::InputLocationMessageContent(value)
    }
}

impl From<InputVenueMessageContent> for InputMessageContent {
    fn from(value: InputVenueMessageContent) -> Self {
        InputMessageContent::InputVenueMessageContent(value)
    }
}

impl From<InputContactMessageContent> for InputMessageContent {
    fn from(value: InputContactMessageContent) -> Self {
        InputMessageContent::InputContactMessageContent(value)
    }
}
