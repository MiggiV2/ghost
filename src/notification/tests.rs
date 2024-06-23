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
        let limit = 30;
        let response = tokio_test::block_on(get_notifications(&gotosocial, &token, limit));

        match response {
            Ok(notifications) => {
                assert_eq!(notifications.len(), limit as usize);
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