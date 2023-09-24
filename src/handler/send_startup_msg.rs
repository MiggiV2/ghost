use std::time::Duration;

use matrix_sdk::Client;
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use matrix_sdk::ruma::RoomId;
use tokio::time::sleep;

use crate::status_checker::HealthChecker;

pub struct HealthStatus {
    pub content: String,
    pub code: u8,
}

pub async fn on_startup_message(room: String, client: &Client) {
    let home_server_url = client.homeserver().await;
    let home_server = String::from(":") + home_server_url.domain().unwrap_or_default();
    let room_id = RoomId::parse(room + home_server.as_str()).unwrap();
    let room = client.get_room(room_id.as_ref()).unwrap();

    tokio::spawn(async move {
        let content = RoomMessageEventContent::text_plain("Bot is up and running! 👟");
        if let Err(e) = room.send(content, None).await {
            eprintln!("Failed to send message! {}", e);
        }

        let mut code = 8; // only matrix is reach able
        let checker = HealthChecker {
            portainer_url: String::from("https://vmd116727.contaboserver.net"),
            forgejo_url: String::from("https://gitea.familyhainz.de"),
            nextcloud_url: String::from("https://nextcloud.mymiggi.de"),
            matrix_url: String::from("https://matrix.familyhainz.de"),
        };

        loop {
            sleep(Duration::from_secs(60 * 1)).await;
            let healthy_content = build_health_message(&checker).await;
            if healthy_content.code == code {
                continue;
            }
            code = healthy_content.code;
            let content = RoomMessageEventContent::text_plain(healthy_content.content);
            if let Err(e) = room.send(content, None).await {
                eprintln!("Failed to send message! {}", e);
            }
        }
    });
}

pub async fn build_health_message(checker: &HealthChecker) -> HealthStatus {
    let mut content = String::from("🐋 Here is an overview of the accessible web services and their status:\n");

    let is_running_1 = checker.check_matrix().await;
    let status_line = format!("{} Matrix - {}\n", get_status_emoji(is_running_1), get_nl_text(is_running_1));
    content.push_str(status_line.as_str());

    let is_running_2 = checker.check_forgejo().await;
    let status_line = format!("{} Forgejo - {}\n", get_status_emoji(is_running_2), get_nl_text(is_running_2));
    content.push_str(status_line.as_str());

    let is_running_3 = checker.check_portainer().await;
    let status_line = format!("{} Portainer - {}\n", get_status_emoji(is_running_3), get_nl_text(is_running_3));
    content.push_str(status_line.as_str());

    let is_running_4 = checker.check_nextcloud().await;
    let status_line = format!("{} Nextcloud - {}\n", get_status_emoji(is_running_4), get_nl_text(is_running_4));
    content.push_str(status_line.as_str());

    let code = checker.get_status_id(is_running_1, is_running_2, is_running_3, is_running_4);
    HealthStatus {
        content,
        code,
    }
}

fn get_status_emoji(is_healthy: bool) -> String {
    if is_healthy {
        return String::from("🟢");
    }
    return String::from("🔴");
}

fn get_nl_text(is_healthy: bool) -> String {
    if is_healthy {
        return String::from("Online and ready to go");
    }
    String::from("Offline 💀")
}

#[cfg(test)]
mod msg_builder_tests {
    use crate::handler::send_startup_msg::build_health_message;
    use crate::status_checker::HealthChecker;

    #[test]
    fn test_one() {
        let checker = HealthChecker {
            portainer_url: String::from("https://vmd116727.contaboserver.net"),
            forgejo_url: String::from("https://gitea.familyhainz.de"),
            nextcloud_url: String::from("https://nextcloud.mymiggi.de"),
            matrix_url: String::from("https://matrix.familyhainz.de"),
        };
        let content = tokio_test::block_on(build_health_message(&checker));
        let expected = String::from("🐋 Here is an overview of the accessible web services and their status:
🟢 Matrix - Online and ready to go
🟢 Forgejo - Online and ready to go
🟢 Portainer - Online and ready to go
🟢 Nextcloud - Online and ready to go\n");

        assert_eq!(expected, content);
    }
}