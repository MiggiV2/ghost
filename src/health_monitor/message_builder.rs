use crate::health_monitor::services::Service;

pub struct HealthStatus {
    pub content: String,
    pub code: i32,
}

pub async fn build_health_message(services: &Vec<Service>) -> HealthStatus {
    let mut content = String::from("🐋 Here is an update of the accessible web services and their status:\n");
    let base: i32 = 2;
    let mut index = 0;
    let mut status_code = 0;

    for service in services {
        let is_okay = service.is_okay().await;
        let emoji = get_status_emoji(is_okay);
        let text = get_nl_text(is_okay);

        let line;
        if is_okay {
            status_code += base.pow(index);
            line = format!("{} {} - {}\n", emoji, service.get_type().to_string(), text);
        } else {
            line = format!("{} {} - {} Check this service on {}\n", emoji, service.get_type().to_string(), text, service.get_url());
        }
        content.push_str(line.as_str());
        index += 1;
    }

    HealthStatus {
        content,
        code: status_code,
    }
}

fn get_status_emoji(is_healthy: bool) -> String {
    if is_healthy {
        return String::from("🟢");
    }
    return String::from("🔴");
}

fn get_nl_text(is_healthy: bool) -> String {
    if is_healthy {
        return String::from("Online and ready to go");
    }
    String::from("Offline 💀")
}

#[cfg(test)]
mod msg_builder_tests {
    use crate::health_monitor::config_builder::ConfBuilder;
    use crate::health_monitor::message_builder::build_health_message;

    #[test]
    fn test_one() {
        let config = ConfBuilder::new().build();
        let health_status = tokio_test::block_on(build_health_message(&config.services));

        assert!(health_status.content.len() > 250, "Message is to short");
        assert!(health_status.content.contains("🐋"), "There is a whale missing!");
        assert!(health_status.content.contains("🟢"), "Expected at least one green dot.");
        assert!(health_status.content.contains("\n"));
    }
}