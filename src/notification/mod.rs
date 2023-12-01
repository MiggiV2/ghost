use matrix_sdk::ruma::exports::http::header::USER_AGENT;
use serde::ser::Error;

use crate::health_monitor::services::Service;
use crate::notification::notification::Notifications;

mod notification;

pub async fn get_notifications(go2social: Service, token: String) -> Option<Notifications> {
    let full_url = go2social.get_url() + "/api/v1/notifications?limit=5";
    let client = reqwest::Client::new();
    let agent = format!("Ghost-Bot {}", env!("CARGO_PKG_VERSION"));
    let response = client
        .get(full_url)
        .header(USER_AGENT, agent)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;

    if let Ok(r) = response {
        if !r.status().is_success() {
            return None;
        }

        if let Ok(body) = r.text().await {
            return serde_json::from_str(body.as_str()).unwrap_or_default();
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::health_monitor::services::Service;
    use crate::health_monitor::ServiceType;
    use crate::notification::get_notifications;

    #[test]
    pub fn test() {
        let gotosocial = Service {
            url: String::from("https://social.mymiggi.de"),
            service_type: ServiceType::Gotosocial,
        };
        let token = String::from("XXX");
        let notification_response = tokio_test::block_on(get_notifications(gotosocial, token));

        if let Some(notifications) = notification_response {
            assert_eq!(notifications.len(), 5);
            for n in notifications {
                println!("Post from {}", n.account.display_name);
            }
        }
    }
}