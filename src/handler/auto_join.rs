use std::time::Duration;

use matrix_sdk::{Client, Room};
use matrix_sdk::ruma::events::room::member::StrippedRoomMemberEvent;
use tokio::time::sleep;

pub fn on_stripped_state_member(
    room_member: StrippedRoomMemberEvent,
    client: Client,
    room: Room,
) {
    if room_member.state_key != client.user_id().unwrap() {
        return;
    }

    // The event handlers are called before the next sync begins, but
    // methods that change the state of a room (joining, leaving a room)
    // wait for the sync to return the new room state so we need to spawn
    // a new task for them.
    tokio::spawn(async move {
        println!("Autojoining room {}", room.room_id());
        let mut delay = 2;

        while let Err(err) = room.join().await {
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
