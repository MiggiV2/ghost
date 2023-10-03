use std::time::Duration;

use matrix_sdk::{Room, RoomState};
use matrix_sdk::ruma::events::room::message::{MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent};
use tokio::time::sleep;

use crate::handler::send_startup_msg::build_health_message;
use crate::status_checker::config_builder::ConfBuilder;

pub async fn on_room_message(event: OriginalSyncRoomMessageEvent, room: Room) {
    if room.state() != RoomState::Joined {
        return;
    }
    let MessageType::Text(text_content) = event.content.msgtype else { return; };

    if text_content.body.contains("!party") {
        let content = RoomMessageEventContent::text_plain("🎉🎊🥳 let's PARTY!! 🥳🎊🎉");
        println!("sending");
        room.send(content, None).await.unwrap();
        println!("message sent");
    }

    if text_content.body.contains("!sleep") {
        tokio::spawn(async move {
            let delay = 20;
            println!("Starting future task...");
            println!("Sleeping for {} sec...", delay);
            sleep(Duration::from_secs(delay)).await;
            println!("Ahhh, good power nap!\nSending msg...");
            let _content = RoomMessageEventContent::text_plain("Good morning! 😊");
            eprintln!("Disabled!");
            // room.send(content, None).await.expect("Failed to send msg!");
            println!("Message sent!");
        });
    }

    if text_content.body.contains("!health") {
        tokio::spawn(async move {
            let config = ConfBuilder::new().build();
            let healthy_content = build_health_message(&config).await;
            let content = RoomMessageEventContent::text_plain(healthy_content.content);
            if let Err(e) = room.send(content, None).await {
                eprintln!("Failed to send message! {}", e);
            }
        });
    }
}