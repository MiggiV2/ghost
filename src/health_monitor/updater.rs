use std::time::Duration;

use chrono::Local;
use matrix_sdk::Client;
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use matrix_sdk::ruma::RoomId;
use tokio::task::yield_now;
use tokio::time::sleep;

use crate::health_monitor::config_builder::ConfBuilder;
use crate::health_monitor::message_builder::build_health_message;

pub async fn send_health_updates(client: &Client, minutes: u64) {
    let config = ConfBuilder::new().build();

    let room_id = RoomId::parse(config.room_id.unwrap_or_default())
        .expect("Can't parse room!");
    let room = client.get_room(&room_id)
        .expect("Failed to get room!");

    tokio::spawn(async move {
        let base: i32 = 2;

        // State - Health
        let mut code = base.pow(config.services.len() as u32) - 1; // every service is online
        let mut code_before = code;

        loop {
            sleep(Duration::from_secs(60 * minutes)).await;

            // Health
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
        }
    });
}