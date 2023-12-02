use matrix_sdk::ruma::exports::http::header::USER_AGENT;

use crate::health_monitor::services::Service;
use crate::notification::notification::Notifications;

mod notification;

pub async fn get_notifications(go2social: &Service, token: String) -> Option<Notifications> {
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
            let response = serde_json::from_str(body.as_str()).unwrap_or_default();
            return Some(response);
        }
    }

    None
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

        if let Some(notifications) = tokio_test::block_on(get_notifications(&gotosocial, token)) {
            assert_eq!(notifications.len(), 5);
            for n in notifications {
                println!("{}\n{}\n", n.type_field, n.account.display_name);
            }
        } else {
            panic!("Expected responses!");
        }
    }
}