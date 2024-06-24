use matrix_sdk::ruma::exports::http::header::USER_AGENT;
use reqwest::Response;

use crate::health_monitor::services::Service;
use crate::notification::notification::{Notification, NotificationList};

mod notification;
pub mod updater;
mod tests;

pub async fn get_notifications(go2social: &Service, token: &String, limit: i32) -> Result<NotificationList, String> {
    let full_url = format!("{}/api/v1/notifications?limit={}", go2social.get_url(), limit);
    let client = reqwest::Client::new();
    let agent = format!("Ghost-Bot {}", env!("CARGO_PKG_VERSION"));
    let response = client
        .get(full_url)
        .header(USER_AGENT, agent)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;

    match response {
        Ok(r) => {
            match parse_body(r).await {
                Ok(value) => value,
                Err(value) => return value,
            }
        }
        Err(e) => {
            Err(e.to_string())
        }
    }
}

async fn parse_body(r: Response) -> Result<Result<NotificationList, String>, Result<NotificationList, String>> {
    if !r.status().is_success() {
        return Err(Err(format!("Unexpected status code {}", r.status())));
    }
    Ok(if let Ok(body) = r.text().await {
        let response = serde_json::from_str(body.as_str());
        match response {
            Ok(list) => {
                Ok(list)
            }
            Err(e) => {
                if cfg!(debug_assertions) {
                    Err(e.to_string() + "\nJSON: " + body.as_str())
                } else {
                    Err(e.to_string())
                }
            }
        }
    } else {
        Err(String::from("Failed to load body!"))
    })
}

pub fn build_notification_html(notification: &Notification) -> String {
    let display_name = notification.account.display_name.to_string();
    match notification.type_field.as_str() {
        "status" => {
            if let Some(status) = &notification.status {
                return format!("<p>🗨 {} posted</p>\n<p>{}</p>",
                               display_name,
                               status.content
                );
            }
            // Fallback
            format!("<p>🗨 {} posted</p>", display_name)
        }
        "mention" => {
            if let Some(status) = &notification.status {
                return format!("<p>🥰 {} replied to your post!</p>\n<p>{}</p>",
                               display_name,
                               status.content
                );
            }
            format!("<p>🥰 {} replied to your post!</p>", display_name)
        }
        "favourite" => {
            if let Some(status) = &notification.status {
                return format!("<p>😘 {} just liked your post!</p>\n<p>{}</p>",
                               display_name,
                               status.content
                );
            }
            format!("<p>😘 {} just liked your post!</p>",
                    display_name
            )
        }
        "follow" => {
            format!("<p>😊 {} follows you now!</p>",
                    display_name
            )
        }
        "reblog" => {
            format!("<p>🔄 {} boosted your post!</p>",
                    display_name
            )
        }
        "poll" => {
            format!("<p>📊 {}'s poll ended!</p>",
                    display_name
            )
        }
        _ => {
            String::from("<p>🙄 Unknown type?!</p>")
        }
    }
}

pub fn build_notification_plain(notification: &Notification) -> String {
    let display_name = notification.account.display_name.to_string();
    match notification.type_field.as_str() {
        "status" => {
            format!("🗨 {} just tooted!", display_name, )
        }
        "mention" => {
            format!("🥰 A new comment from {}", display_name)
        }
        "favourite" => {
            format!("😘 {} just liked your post!", display_name)
        }
        "follow" => {
            format!("😊 {} follows you now!", display_name)
        }
        _ => {
            format!("🙄 Unknown type?!")
        }
    }
}