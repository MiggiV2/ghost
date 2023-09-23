use std::time::Duration;

use matrix_sdk::Room;
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use tokio::time::sleep;

pub fn on_startup_message(room: Room) {
    tokio::spawn(async move {
        for i in 0..10 {
            let body = format!("Bot is up and running! 👟 {}", i);
            let content = RoomMessageEventContent::text_plain(body);

            sleep(Duration::from_secs(i * 5)).await;

            println!("Sending start msg...");
            room.send(content, None).await.unwrap();
            println!("Sent!");
        }
    });
}