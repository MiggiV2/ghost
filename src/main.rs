use std::{env, process::exit};

use matrix_sdk::{
    Client,
    config::SyncSettings,
};
use matrix_sdk::ruma::RoomId;

use crate::handler::Handler;

mod handler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let (homeserver_url, username, password) =
        match (env::args().nth(1), env::args().nth(2), env::args().nth(3)) {
            (Some(a), Some(b), Some(c)) => (a, b, c),
            _ => {
                eprintln!(
                    "Usage: {} <homeserver_url> <username> <password>",
                    env::args().next().unwrap()
                );
                exit(1)
            }
        };

    login_and_sync(homeserver_url, &username, &password).await?;
    Ok(())
}

async fn login_and_sync(
    homeserver_url: String,
    username: &str,
    password: &str,
) -> anyhow::Result<()> {
    let client = Client::builder()
        .homeserver_url(homeserver_url)
        .build()
        .await?;

    client
        .matrix_auth()
        .login_username(username, password)
        .initial_device_display_name("getting started bot")
        .await?;

    println!("logged in as {username}");

    client.add_event_handler(Handler::on_invite);

    let sync_token = client.sync_once(SyncSettings::default()).await.unwrap().next_batch;

    client.add_event_handler(Handler::on_room_message);

    let room_id = RoomId::parse("!hFekksusgjPusUvBbO:matrix.familyhainz.de").unwrap();
    let room = client.get_room(room_id.as_ref()).unwrap();
    Handler::on_startup(room);
    let settings = SyncSettings::default().token(sync_token);
    client.sync(settings).await?; // this essentially loops until we kill the bot

    Ok(())
}