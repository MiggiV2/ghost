use std::{env, process::exit};

use matrix_sdk::{
    config::SyncSettings, ruma::events::room::member::StrippedRoomMemberEvent, Client,
};
use matrix_sdk::room::Room;
use matrix_sdk::ruma::user_id;
use tokio::time::{sleep, Duration};

async fn on_stripped_state_member(
    room_member: StrippedRoomMemberEvent,
    client: Client,
    room: Room,
) {
    if room_member.state_key != client.user_id().unwrap() {
        return;
    }

    tokio::spawn(async move {
        println!("Autojoining room {}", room.room_id());
        let mut delay = 2;

        while let Err(err) = client.join_room_by_id(room.room_id()).await {
            // retry autojoin due to synapse sending invites, before the
            // invited user can join for more information see
            // https://github.com/matrix-org/synapse/issues/4345
            eprintln!("Failed to join room {} ({err:?}), retrying in {delay}s", room.room_id());

            sleep(Duration::from_secs(delay)).await;
            delay *= 2;

            if delay > 3600 {
                eprintln!("Can't join room {} ({err:?})", room.room_id());
                break;
            }
        }
        println!("Successfully joined room {}", room.room_id());
    });
}

async fn login_and_sync(
    username: &str,
    password: &str,
) -> anyhow::Result<()> {
    // Note that when encryption is enabled, you should use a persistent store to be
    // able to restore the session with a working encryption setup.
    // See the `persist_session` example.
    let alice = user_id!("@ghost:matrix.familyhainz.de");
    let client = Client::builder().server_name(alice.server_name()).build().await?;

    // First we need to log in.
    client.login_username(alice, password).send().await?;

    println!("logged in as {username}");

    client.add_event_handler(on_stripped_state_member);

    client.sync(SyncSettings::default()).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let ( username, password) =
        match (env::args().nth(1), env::args().nth(2)) {
            (Some(a), Some(b)) => (a, b),
            _ => {
                eprintln!(
                    "Usage: {} <username> <password>",
                    env::args().next().unwrap()
                );
                exit(1)
            }
        };

    login_and_sync(&username, &password).await?;
    Ok(())
}