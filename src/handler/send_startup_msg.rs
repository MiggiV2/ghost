use matrix_sdk::Client;
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use matrix_sdk::ruma::RoomId;

use crate::health_monitor::config_builder::ConfBuilder;
use crate::health_monitor::updater::send_health_updates;
use crate::notification::updater::send_notification_updates;

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

    let content = RoomMessageEventContent::text_plain("Bot is up and running! 👟");
    if let Err(e) = room.send(content).await {
        eprintln!("Failed to send message! {}", e);
    }

    send_health_updates(&client, 15).await;
    send_notification_updates(&client, 2).await;
}