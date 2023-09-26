use std::{env, process::exit};

use matrix_sdk::{
    Client,
    config::SyncSettings,
};

use crate::handler::Handler;

mod handler;
mod status_checker;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // tracing_subscriber::fmt::init();

    let (home_server_url, username, password) =
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

    login_and_sync(home_server_url, &username, &password).await?;
    Ok(())
}

async fn login_and_sync(
    home_server_url: String,
    username: &str,
    password: &str,
) -> anyhow::Result<()> {
    let client = Client::builder()
        .homeserver_url(home_server_url)
        .build()
        .await?;

    client
        .matrix_auth()
        .login_username(username, password)
        .initial_device_display_name("ghost-bot")
        .await?;

    println!("logged in as {username}");

    client.add_event_handler(Handler::on_invite);

    let sync_token = client.sync_once(SyncSettings::default()).await.unwrap().next_batch;

    client.add_event_handler(Handler::on_room_message);

    Handler::on_startup(String::from("!hFekksusgjPusUvBbO"), &client).await;
    let settings = SyncSettings::default().token(sync_token);
    client.sync(settings).await?; // this essentially loops until we kill the bot

    Ok(())
}