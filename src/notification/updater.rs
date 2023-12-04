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
use crate::notification::{build_notification_html, build_notification_plain, get_notifications};

pub async fn send_notification_updates(client: &Client, minutes: u64) {
    let config = ConfBuilder::new().build();
    let room_id = RoomId::parse(config.room_id.unwrap_or_default())
        .expect("Can't parse room!");
    let room = client.get_room(&room_id)
        .expect("Failed to get room!");

    tokio::spawn(async move {
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
            sleep(Duration::from_secs(60 * minutes)).await;

            // Basic
            let date = Local::now().format("[%Y-%m-%d] %H:%M:%S");
            yield_now().await;

            // Gotosocial
            if token.is_empty() {
                println!("{} Gotosocial disabled! No token found...", date);
                break;
            }
            let notifications = get_notifications(&gotosocial, &token).await;
            if notifications.is_err() {
                println!("{} Failed to fetch notifications! -> 0 notifications...", date);
                continue;
            }

            let notifications = notifications.expect("Checked");
            let mut saved_ts = newest_ts;

            if let Some(newest) = &notifications.first() {
                saved_ts = newest.parse_created_at();
            }

            // println!("{} Fetched notifications {}!", date, notifications.len());

            for notification in notifications {
                if newest_ts == 0 {
                    println!("{} Skipping init...", date);
                    break;
                }
                if notification.parse_created_at() <= newest_ts {
                    // println!("{} No new notification!", date);
                    break;
                }
                println!("{} New Gotosocial notification! -> {}", date, notification.id.to_string());
                let plain = build_notification_plain(&notification);
                let html = build_notification_html(&notification);
                let content = RoomMessageEventContent::text_html(plain, html);
                if let Err(e) = room.send(content).await {
                    eprintln!("Failed to send message! {}", e);
                }
            }
            if newest_ts != saved_ts {
                // println!("{} Updating ts from {} to {}", date, newest_ts, saved_ts);
                newest_ts = saved_ts;
            }
        }
    });
}