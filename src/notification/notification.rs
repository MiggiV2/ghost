use chrono::DateTime;
use serde_derive::Deserialize;
use serde_derive::Serialize;

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
    #[serde(rename = "display_name")]
    pub display_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub id: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub content: String,
}