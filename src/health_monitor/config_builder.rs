use std::{env, fs};

use strum::IntoEnumIterator;
use toml::Table;

use crate::health_monitor::services::Service;
use crate::health_monitor::ServiceType;

pub struct ConfBuilder {
    file_path: String,
}

pub struct GhostConfig {
    pub services: Vec<Service>,
    pub room_id: Option<String>,
    pub gotosocial_token: Option<String>,
    pub notification_refresh: Option<i64>,
}

impl GhostConfig {
    pub fn empty() -> Self {
        GhostConfig {
            services: Vec::new(),
            room_id: None,
            gotosocial_token: None,
            notification_refresh: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.room_id.is_none() || self.services.is_empty()
    }
}

impl ConfBuilder {
    pub fn new() -> Self {
        let default = String::from("checker.toml");
        Self {
            file_path: env::var("CHECKER_CONF").unwrap_or(default)
        }
    }

    pub fn build(&self) -> GhostConfig {
        let content = fs::read_to_string(self.file_path.to_string());
        if let Err(e) = content {
            eprintln!("Cloud not read config file {}! {}", self.file_path, e);
            return GhostConfig::empty();
        }

        let config = content.unwrap_or_default();
        let value = config.parse::<Table>();

        if let Err(e) = value {
            eprintln!("Failed to parse config file! {}", e);
            return GhostConfig::empty();
        }

        let value = value.unwrap_or_default();
        let mut services: Vec<Service> = Vec::new();
        let room_id = value.get("room-id");

        if let None = room_id {
            return GhostConfig::empty();
        }

        let room_id = room_id.expect("Checked")
            .as_str()
            .unwrap_or_default()
            .to_string();

        for service in ServiceType::iter() {
            let conf_key = service.to_string().to_lowercase();
            if let Some(conf_value) = value.get(conf_key.as_str()) {
                let url = conf_value.as_str().unwrap_or_default().to_string();
                services.push(Service::new(url, service));
            }
        }

        let mut gotosocial_token = None;
        let mut notification_refresh = None;

        if let Some(notifications) = value.get("notifications").and_then(|v| v.as_table()) {
            gotosocial_token = Self::get_token(notifications);
            notification_refresh = Self::get_refresh(notifications);
        }

        GhostConfig {
            services,
            room_id: Some(room_id),
            gotosocial_token,
            notification_refresh,
        }
    }

    fn get_token(notifications: &Table) -> Option<String> {
        if let Some(gotosocial_token) = notifications.get("gotosocial_token").and_then(|v| v.as_str()) {
            return Some(gotosocial_token.to_string());
        }
        return None;
    }

    fn get_refresh(notifications: &Table) -> Option<i64> {
        if let Some(refresh_interval) = notifications.get("refresh_interval").and_then(|v| v.as_integer()) {
            return Some(refresh_interval);
        }
        return None;
    }
}

#[cfg(test)]
mod build_tests {
    use crate::health_monitor::config_builder::ConfBuilder;

    #[test]
    fn read_config_correctly() {
        let builder = ConfBuilder::new();
        let config = builder.build();

        assert_eq!(7, config.services.len(), "Expected more or less services in config file!");
        for service in config.services {
            assert!(!service.get_url().is_empty())
        }
    }
}