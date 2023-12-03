use std::time::Duration;

use chrono::Local;
use matrix_sdk::Client;
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use matrix_sdk::ruma::RoomId;
use tokio::task::yield_now;
use tokio::time::sleep;

use crate::health_monitor::config_builder::ConfBuilder;
use crate::health_monitor::services::Service;
use crate::health_monitor::ServiceType;
use crate::notification::{build_notification_msg, get_notifications};

pub struct HealthStatus {
    pub content: String,
    pub code: i32,
}

pub async fn on_startup_message(client: &Client) {
    let config = ConfBuilder::new().build();

    if config.is_empty() {
        eprintln!("No room id or services set!");
        return;
    }

    let room_id = RoomId::parse(config.room_id.unwrap_or_default())
        .expect("Can't parse room!");
    let room = client.get_room(&room_id)
        .expect("Failed to get room!");

    tokio::spawn(async move {
        let content = RoomMessageEventContent::text_plain("Bot is up and running! 👟");
        if let Err(e) = room.send(content).await {
            eprintln!("Failed to send message! {}", e);
        }

        let base: i32 = 2;
        // State - Health
        let mut code = base.pow(config.services.len() as u32) - 1; // every service is online
        let mut code_before = code;
        // State - Gotosocial
        let mut newest_ts = 0;
        let mut gotosocial = &Service::new(String::new(), ServiceType::Wordpress);
        let token = config.gotosocial_token.unwrap_or_default();
        for service in &config.services {
            if let ServiceType::Gotosocial = service.service_type {
                gotosocial = service;
            }
        }

        loop {
            sleep(Duration::from_secs(60 * 5)).await;

            let healthy_content = build_health_message(&config.services).await;
            let date = Local::now().format("[%Y-%m-%d] %H:%M:%S");
            yield_now().await;

            let no_change = healthy_content.code == code && code == code_before;
            if no_change {
                println!("{} No accessible update found.", date);
            } else {
                let change_1st_time = healthy_content.code != code;
                let is_false_positive = healthy_content.code == code_before;
                if change_1st_time {
                    if !is_false_positive {
                        println!("{} Found accessible update, but we are waiting...", date);
                    } else {
                        println!("{} Found accessible update, but it was a false positive.", date);
                        code = healthy_content.code;    // Correct it
                    }
                }

                let change_2nd_time = healthy_content.code == code && code != code_before;
                if change_2nd_time {
                    let content = RoomMessageEventContent::text_plain(healthy_content.content);
                    code = healthy_content.code;
                    println!("{} Found accessible update!", date);

                    if let Err(e) = room.send(content).await {
                        eprintln!("Failed to send message! {}", e);
                    }
                }

                code_before = code;
                code = healthy_content.code;
            }

            // Gotosocial
            if token.is_empty() {
                println!("{} Gotosocial disabled! No token found...", date);
                continue;
            }
            let notifications = get_notifications(&gotosocial, &token).await;
            if notifications.is_none() {
                println!("{} Failed to fetch notifications! -> 0 notifications...", date);
                continue;
            }

            let notifications = notifications.expect("Checked");
            let mut saved_ts = newest_ts;

            if let Some(newest) = &notifications.first() {
                saved_ts = newest.parse_created_at();
            }

            for notification in notifications {
                if newest_ts == 0 {
                    println!("{} Skipping init...", date);
                    break;
                }
                if notification.parse_created_at() <= newest_ts {
                    println!("{} No new notification!", date);
                    break;
                }
                println!("{} New Gotosocial notification! -> {}", date, notification.id.to_string());
                let content = RoomMessageEventContent::text_html(
                    "Your client not support html :-(", build_notification_msg(notification),
                );
                if let Err(e) = room.send(content).await {
                    eprintln!("Failed to send message! {}", e);
                }
            }
            if newest_ts != saved_ts {
                println!("{} Updating ts from {} to {}", date, newest_ts, saved_ts);
                newest_ts = saved_ts;
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
    use crate::health_monitor::config_builder::ConfBuilder;

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