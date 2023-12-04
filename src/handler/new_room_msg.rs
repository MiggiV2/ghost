use matrix_sdk::{Room, RoomState};
use matrix_sdk::ruma::events::room::message::{MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent};

use crate::health_monitor::config_builder::ConfBuilder;
use crate::health_monitor::message_builder::build_health_message;

pub async fn on_room_message(event: OriginalSyncRoomMessageEvent, room: Room) {
    if room.state() != RoomState::Joined {
        return;
    }
    let MessageType::Text(text_content) = event.content.msgtype else { return; };

    if text_content.body.contains("!ping") {
        let content = RoomMessageEventContent::text_plain("Hi 🥹 It's me!");
        if let Err(e) = room.send(content).await {
            eprintln!("Failed to send message! {}", e);
        }
    }

    if text_content.body.contains("!version") {
        let msg = format!("Current version of Ghost-Bot is {} 👻", env!("CARGO_PKG_VERSION"));
        let content = RoomMessageEventContent::text_plain(msg);
        if let Err(e) = room.send(content).await {
            eprintln!("Failed to send message! {}", e);
        }
    }

    if text_content.body.contains("!health") {
        tokio::spawn(async move {
            let config = ConfBuilder::new().build();
            let healthy_content = build_health_message(&config.services).await;
            let content = RoomMessageEventContent::text_plain(healthy_content.content);
            if let Err(e) = room.send(content).await {
                eprintln!("Failed to send message! {}", e);
            }
        });
    }
}