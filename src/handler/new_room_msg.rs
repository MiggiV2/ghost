use std::time::Duration;

use matrix_sdk::{Room, RoomState};
use matrix_sdk::ruma::events::room::message::{MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent};
use tokio::time::sleep;

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
            let content = RoomMessageEventContent::text_plain("Good morning! 😊");
            room.send(content, None).await.expect("Failed to send msg!");
            println!("Message sent!");
        });
    }
}