use std::{env, fs};
use std::collections::HashMap;

use strum::IntoEnumIterator;
use toml::Table;

use crate::status_checker::services::Service;
use crate::status_checker::ServiceType;

pub struct ConfBuilder {
    file_path: String,
}

impl ConfBuilder {
    pub fn new() -> Self {
        let default = String::from("checker.toml");
        Self {
            file_path: env::var("CHECKER_CONF").unwrap_or(default)
        }
    }

    pub fn build(&self) -> Vec<Service> {
        let content = fs::read_to_string(self.file_path.to_string());
        if let Err(e) = content {
            eprintln!("Cloud not read config file {}! {}", self.file_path, e);
            return vec![];
        }

        let config = content.unwrap_or_default();
        let value = config.parse::<Table>();

        if let Err(e) = value {
            eprintln!("Failed to parse config file! {}", e);
            return vec![];
        }

        let value = value.unwrap_or_default();
        let mut config: Vec<Service> = Vec::new();

        let mut map = HashMap::new();
        for service in ServiceType::iter() {
            map.insert(service.to_string().to_lowercase(), service);
        }

        for (conf_key, service_type) in map {
            if let Some(conf_value) = value.get(conf_key.as_str()) {
                let url = conf_value.as_str().unwrap_or_default().to_string();
                config.push(Service::new(url, service_type));
            }
        }

        return config;
    }
}

#[cfg(test)]
mod build_tests {
    use crate::status_checker::config_builder::ConfBuilder;

    #[test]
    fn read_config_correctly() {
        let builder = ConfBuilder::new();
        let config = builder.build();

        assert_eq!(7, config.len(), "Expected more or less services in config file!");
        for service in config {
            assert!(!service.get_url().is_empty())
        }
    }
}