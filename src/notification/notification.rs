use chrono::DateTime;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

/**
Generated with https://transform.tools/json-to-rust-serde
 */
pub type NotificationList = Vec<Notification>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub account: Account,
    pub status: Option<Status>,
}

impl Notification {
    pub fn parse_created_at(&self) -> i64 {
        DateTime::parse_from_rfc3339(self.created_at.as_str())
            .unwrap_or_default()
            .timestamp()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: String,
    pub username: String,
    pub acct: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    pub locked: bool,
    pub discoverable: bool,
    pub bot: bool,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub note: String,
    pub url: String,
    pub avatar: String,
    #[serde(rename = "avatar_static")]
    pub avatar_static: String,
    pub header: String,
    #[serde(rename = "header_static")]
    pub header_static: String,
    #[serde(rename = "followers_count")]
    pub followers_count: i64,
    #[serde(rename = "following_count")]
    pub following_count: i64,
    #[serde(rename = "statuses_count")]
    pub statuses_count: i64,
    #[serde(rename = "last_status_at")]
    pub last_status_at: Option<String>,
    pub emojis: Vec<Value>,
    pub fields: Vec<Field>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub name: String,
    pub value: String,
    #[serde(rename = "verified_at")]
    pub verified_at: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub id: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "in_reply_to_id")]
    pub in_reply_to_id: Option<String>,
    #[serde(rename = "in_reply_to_account_id")]
    pub in_reply_to_account_id: Option<String>,
    pub sensitive: bool,
    #[serde(rename = "spoiler_text")]
    pub spoiler_text: String,
    pub visibility: String,
    pub language: Option<String>,
    pub uri: String,
    pub url: String,
    #[serde(rename = "replies_count")]
    pub replies_count: i64,
    #[serde(rename = "reblogs_count")]
    pub reblogs_count: i64,
    #[serde(rename = "favourites_count")]
    pub favourites_count: i64,
    pub favourited: bool,
    pub reblogged: bool,
    pub muted: bool,
    pub bookmarked: bool,
    pub pinned: bool,
    pub content: String,
    pub reblog: Value,
    pub account: Account2,
    #[serde(rename = "media_attachments")]
    pub media_attachments: Vec<MediaAttachment>,
    pub mentions: Vec<Mention>,
    pub tags: Vec<Tag>,
    pub emojis: Vec<Value>,
    pub card: Value,
    pub poll: Value,
    pub application: Option<Application>,
    pub text: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account2 {
    pub id: String,
    pub username: String,
    pub acct: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    pub locked: bool,
    pub discoverable: bool,
    pub bot: bool,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub note: String,
    pub url: String,
    pub avatar: String,
    #[serde(rename = "avatar_static")]
    pub avatar_static: String,
    pub header: String,
    #[serde(rename = "header_static")]
    pub header_static: String,
    #[serde(rename = "followers_count")]
    pub followers_count: i64,
    #[serde(rename = "following_count")]
    pub following_count: i64,
    #[serde(rename = "statuses_count")]
    pub statuses_count: i64,
    #[serde(rename = "last_status_at")]
    pub last_status_at: String,
    pub emojis: Vec<Value>,
    pub fields: Vec<Field2>,
    pub role: Option<Role>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field2 {
    pub name: String,
    pub value: String,
    #[serde(rename = "verified_at")]
    pub verified_at: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaAttachment {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    #[serde(rename = "text_url")]
    pub text_url: String,
    #[serde(rename = "preview_url")]
    pub preview_url: String,
    #[serde(rename = "remote_url")]
    pub remote_url: Option<String>,
    #[serde(rename = "preview_remote_url")]
    pub preview_remote_url: Value,
    pub meta: Meta,
    pub description: Value,
    pub blurhash: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub original: Original,
    pub small: Small,
    pub focus: Focus,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Original {
    pub width: i64,
    pub height: i64,
    pub size: String,
    pub aspect: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Small {
    pub width: i64,
    pub height: i64,
    pub size: String,
    pub aspect: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Focus {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mention {
    pub id: String,
    pub username: String,
    pub url: String,
    pub acct: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    pub name: String,
    pub website: String,
}
