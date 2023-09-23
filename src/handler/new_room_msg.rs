use std::time::Duration;

use matrix_sdk::{Room, RoomState};
use matrix_sdk::ruma::events::room::message::{MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent};
use tokio::time::sleep;

use crate::handler::send_startup_msg::build_health_message;
use crate::status_checker::HealthChecker;

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
            let checker = HealthChecker {
                portainer_url: String::from("https://vmd116727.contaboserver.net"),
                forgejo_url: String::from("https://gitea.familyhainz.de"),
                nextcloud_url: String::from("https://nextcloud.mymiggi.de"),
                matrix_url: String::from("https://matrix.familyhainz.de"),
            };
            let healthy_content = build_health_message(&checker).await;
            let content = RoomMessageEventContent::text_plain(healthy_content);
            if let Err(e) = room.send(content, None).await {
                eprintln!("Failed to send message! {}", e);
            }
        });
    }
}