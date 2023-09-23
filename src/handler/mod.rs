use matrix_sdk::{Client, Room};
use matrix_sdk::ruma::events::room::member::StrippedRoomMemberEvent;
use matrix_sdk::ruma::events::room::message::OriginalSyncRoomMessageEvent;

mod auto_join;
mod send_startup_msg;
mod new_room_msg;

pub struct Handler {}

impl Handler {
    pub async fn on_invite(room_member: StrippedRoomMemberEvent,
                           client: Client,
                           room: Room) {
        auto_join::on_stripped_state_member(room_member, client, room);
    }

    pub async fn on_startup(room: String, client: &Client) {
        send_startup_msg::on_startup_message(room, client).await;
    }

    pub async fn on_room_message(event: OriginalSyncRoomMessageEvent, room: Room) {
        new_room_msg::on_room_message(event, room).await;
    }
}