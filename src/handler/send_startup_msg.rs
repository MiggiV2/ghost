use std::time::Duration;

use chrono::Local;
use matrix_sdk::Client;
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use matrix_sdk::ruma::RoomId;
use tokio::task::yield_now;
use tokio::time::sleep;

use crate::status_checker::config_builder::ConfBuilder;
use crate::status_checker::services::Service;

pub struct HealthStatus {
    pub content: String,
    pub code: i32,
}

pub async fn on_startup_message(room: String, client: &Client) {
    let home_server_url = client.homeserver().await;
    let home_server = String::from(":") + home_server_url.domain().unwrap_or_default();
    let room_id = RoomId::parse(room + home_server.as_str()).unwrap();
    let room = client.get_room(room_id.as_ref()).unwrap();

    tokio::spawn(async move {
        let content = RoomMessageEventContent::text_plain("Bot is up and running! 👟");
        if let Err(e) = room.send(content, None).await {
            eprintln!("Failed to send message! {}", e);
        }

        let config = ConfBuilder::new("./checker.toml").build();
        let base: i32 = 2;
        let mut code = base.pow(config.len() as u32) - 1; // every service is online

        loop {
            sleep(Duration::from_secs(60 * 5)).await;

            let healthy_content = build_health_message(&config).await;
            let date = Local::now().format("[%Y-%m-%d] %H:%M:%S");
            yield_now().await;

            if healthy_content.code == code {
                println!("{} No accessible update found.", date);
                continue;
            }

            let content = RoomMessageEventContent::text_plain(healthy_content.content);
            code = healthy_content.code;
            println!("{} Found accessible update!", date);

            if let Err(e) = room.send(content, None).await {
                eprintln!("Failed to send message! {}", e);
            }
        }
    });
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
    use crate::handler::send_startup_msg::build_health_message;
    use crate::status_checker::config_builder::ConfBuilder;

    #[test]
    fn test_one() {
        let config = ConfBuilder::new("./checker.toml").build();
        let health_status = tokio_test::block_on(build_health_message(&config));

        assert!(health_status.content.len() > 250);
        assert!(health_status.content.contains("🐋"));
        assert!(health_status.content.contains("🟢"));
        assert!(health_status.content.contains("\n"));
    }
}