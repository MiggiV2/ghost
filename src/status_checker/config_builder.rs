use std::collections::HashMap;
use std::fs;

use toml::Table;

use crate::status_checker::services::Service;
use crate::status_checker::services::ServiceType::{Forgejo, Keycloak, Nextcloud, Portainer, Synapse};

pub struct ConfBuilder {
    file_path: String,
}

impl ConfBuilder {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string()
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
        map.insert("synapse", Synapse);
        map.insert("forgejo", Forgejo);
        map.insert("nextcloud", Nextcloud);
        map.insert("portainer", Portainer);
        map.insert("keycloak", Keycloak);

        for (conf_key, service_type) in map {
            if let Some(conf_value) = value.get(conf_key) {
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
    fn test_one() {
        let builder = ConfBuilder::new("checker.toml");
        let config = builder.build();

        assert_eq!(config.len(), 5);
        for service in config {
            assert!(!service.get_url().is_empty())
        }
    }
}