use matrix_sdk::{Room, RoomState};
use matrix_sdk::ruma::events::room::message::{MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent};

use crate::handler::send_startup_msg::build_health_message;
use crate::status_checker::config_builder::ConfBuilder;

pub async fn on_room_message(event: OriginalSyncRoomMessageEvent, room: Room) {
    if room.state() != RoomState::Joined {
        return;
    }
    let MessageType::Text(text_content) = event.content.msgtype else { return; };

    if text_content.body.contains("!ping") {
        let content = RoomMessageEventContent::text_plain("Hi 🥹 It's me!");
        println!("sending");
        room.send(content).await.unwrap();
        println!("message sent");
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