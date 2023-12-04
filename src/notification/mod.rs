use matrix_sdk::ruma::exports::http::header::USER_AGENT;
use reqwest::Response;

use crate::health_monitor::services::Service;
use crate::notification::notification::{Notification, NotificationList};

mod notification;

pub async fn get_notifications(go2social: &Service, token: &String) -> Result<NotificationList, String> {
    let full_url = go2social.get_url() + "/api/v1/notifications?limit=5";
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
                Err(e.to_string())
            }
        }
    } else {
        Err(String::from("Failed to load body!"))
    })
}

pub fn build_notification_msg(notification: Notification) -> String {
    let display_name = notification.account.display_name.to_string();
    match notification.type_field.as_str() {
        "status" => {
            let content_html = notification.status.expect("Expected status in type 'status'").content;
            format!("<p>🗨 {} posted</p>\n{}",
                    display_name,
                    content_html
            )
        }
        "mention" => {
            let content_html = notification.status.expect("Expected status in type 'mention'").content;
            format!("<p>🥰 New comment from {}</p>\n{}",
                    display_name,
                    content_html
            )
        }
        "favourite" => {
            format!("<p>😘 {} just liked your post!</p>",
                    display_name
            )
        }
        "follow" => {
            format!("<p>😊 {} follows you now!</p>",
                    display_name
            )
        }
        _ => {
            format!("<p>🙄 Unknown type?!</p>")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::health_monitor::config_builder::ConfBuilder;
    use crate::health_monitor::services::Service;
    use crate::health_monitor::ServiceType;
    use crate::notification::get_notifications;

    #[test]
    pub fn test() {
        let mut gotosocial = Service::new(String::new(), ServiceType::Wordpress);
        let config = ConfBuilder::new().build();
        let token = config.gotosocial_token
            .expect("Expected gotosocial_token in checker.toml!");
        // load gotosocial from config
        for service in config.services {
            if let ServiceType::Gotosocial = service.service_type {
                gotosocial = service;
            }
        }

        assert!(!token.is_empty());

        let response = tokio_test::block_on(get_notifications(&gotosocial, &token));

        match response {
            Ok(notifications) => {
                assert_eq!(notifications.len(), 5);
                for n in notifications {
                    let time = n.parse_created_at();
                    println!("[{}]\t{} - {}", n.type_field, time, n.account.display_name);
                }
            }
            Err(e) => {
                panic!("Expected responses! -> {}", e);
            }
        }
    }
}