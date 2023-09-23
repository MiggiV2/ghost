use std::time::Duration;

use matrix_sdk::Client;
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use matrix_sdk::ruma::RoomId;
use tokio::time::sleep;

pub async fn on_startup_message(room: String, client: &Client) {
    let home_server_url = client.homeserver().await;
    let home_server = String::from(":") + home_server_url.domain().unwrap_or_default();
    let room_id = RoomId::parse(room + home_server.as_str()).unwrap();
    let room = client.get_room(room_id.as_ref()).unwrap();

    tokio::spawn(async move {
        for i in 0..2 {
            let body = format!("Bot is up and running! 👟 {}", i + 1);
            let content = RoomMessageEventContent::text_plain(body);

            sleep(Duration::from_secs(i * 5)).await;

            println!("Sending start msg...");
            room.send(content, None).await.unwrap();
            println!("Sent!");
        }
    });
}