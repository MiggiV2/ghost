use std::{env, fs};

use strum::IntoEnumIterator;
use toml::Table;

use crate::health_monitor::services::Service;
use crate::health_monitor::ServiceType;

pub struct ConfBuilder {
    file_path: String,
}

pub struct CheckerConfig {
    pub services: Vec<Service>,
    pub room_id: Option<String>,
}

impl CheckerConfig {
    pub fn empty() -> Self {
        CheckerConfig {
            services: Vec::new(),
            room_id: None,
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

    pub fn build(&self) -> CheckerConfig {
        let content = fs::read_to_string(self.file_path.to_string());
        if let Err(e) = content {
            eprintln!("Cloud not read config file {}! {}", self.file_path, e);
            return CheckerConfig::empty();
        }

        let config = content.unwrap_or_default();
        let value = config.parse::<Table>();

        if let Err(e) = value {
            eprintln!("Failed to parse config file! {}", e);
            return CheckerConfig::empty();
        }

        let value = value.unwrap_or_default();
        let mut services: Vec<Service> = Vec::new();
        let room_id = value.get("room-id");

        if let None = room_id {
            return CheckerConfig::empty();
        }

        let room_id = room_id.expect("Checked").as_str().unwrap_or_default().to_string();

        for service in ServiceType::iter() {
            let conf_key = service.to_string().to_lowercase();
            if let Some(conf_value) = value.get(conf_key.as_str()) {
                let url = conf_value.as_str().unwrap_or_default().to_string();
                services.push(Service::new(url, service));
            }
        }

        CheckerConfig {
            services,
            room_id: Some(room_id),
        }
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