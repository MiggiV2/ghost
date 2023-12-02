use std::{env, fs};

use strum::IntoEnumIterator;
use toml::Table;

use crate::health_monitor::services::Service;
use crate::health_monitor::ServiceType;

pub struct ConfBuilder {
    file_path: String,
}

pub struct HealthConfig {
    pub services: Vec<Service>,
    pub room_id: Option<String>,
    pub gotosocial_token: Option<String>,
}

impl HealthConfig {
    pub fn empty() -> Self {
        HealthConfig {
            services: Vec::new(),
            room_id: None,
            gotosocial_token: None,
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

    pub fn build(&self) -> HealthConfig {
        let content = fs::read_to_string(self.file_path.to_string());
        if let Err(e) = content {
            eprintln!("Cloud not read config file {}! {}", self.file_path, e);
            return HealthConfig::empty();
        }

        let config = content.unwrap_or_default();
        let value = config.parse::<Table>();

        if let Err(e) = value {
            eprintln!("Failed to parse config file! {}", e);
            return HealthConfig::empty();
        }

        let value = value.unwrap_or_default();
        let mut services: Vec<Service> = Vec::new();
        let room_id = value.get("room-id");

        if let None = room_id {
            return HealthConfig::empty();
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

        let token =
            if let Some(notifications) = value.get("notifications").and_then(|v| v.as_table()) {
                Self::get_token(notifications)
            } else {
                None
            };

        HealthConfig {
            services,
            room_id: Some(room_id),
            gotosocial_token: token,
        }
    }

    fn get_token(notifications: &Table) -> Option<String> {
        if let Some(enabled) = notifications.get("gotosocial_enable").and_then(|v| v.as_bool()) {
            if enabled {
                if let Some(gotosocial_token) = notifications.get("gotosocial_token").and_then(|v| v.as_str()) {
                    return Some(gotosocial_token.to_string());
                }
            } else {
                println!("Is disabled!");
            }
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