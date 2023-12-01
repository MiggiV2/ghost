use reqwest::header::USER_AGENT;

use crate::status_checker::ServiceType;

pub struct Service {
    service_type: ServiceType,
    url: String,
}

impl Service {
    pub fn new(url: String, service_type: ServiceType) -> Self {
        Service {
            url,
            service_type,
        }
    }

    pub fn get_url(&self) -> String {
        self.url.to_string()
    }

    pub fn get_type(&self) -> &ServiceType {
        &self.service_type
    }

    pub async fn is_okay(&self) -> bool {
        let full_url = self.get_url() + self.get_endpoint().as_str();
        let client = reqwest::Client::new();
        let agent = format!("Ghost-Bot {}", env!("CARGO_PKG_VERSION"));
        let response = client
            .get(full_url)
            .header(USER_AGENT, agent)
            .send()
            .await;

        if let Ok(r) = response {
            if !r.status().is_success() {
                return false;
            }

            let body = r.text().await.unwrap_or_default()
                .replace("\n", "")
                .replace(" ", "");

            return self.check_body(body);
        }
        false
    }

    fn get_endpoint(&self) -> String {
        match self.service_type {
            ServiceType::Synapse => { String::from("/health") }
            ServiceType::Nextcloud => { String::from("/status.php") }
            ServiceType::Forgejo => { String::from("/api/healthz") }
            ServiceType::Portainer => { String::from("/api/system/status") }
            ServiceType::Keycloak => { String::from("/health") }
            ServiceType::Bitwarden => { String::from("/alive") }
            ServiceType::Wordpress => { String::from("/robots.txt") }
            ServiceType::Gotosocial => { String::from("/nodeinfo/2.0") }
        }
    }

    fn check_body(&self, body: String) -> bool {
        match self.service_type {
            ServiceType::Synapse => {
                body == "OK"
            }
            ServiceType::Nextcloud => {
                body.contains("\"installed\":true") && body.contains("\"productname\":\"Nextcloud\"")
            }
            ServiceType::Forgejo => {
                body.starts_with("{\"status\":\"pass\"")
            }
            ServiceType::Portainer => {
                body.contains("Version") && body.contains("InstanceID")
            }
            ServiceType::Keycloak => {
                body.contains("\"status\":\"UP\"")
            }
            ServiceType::Bitwarden => {
                !body.is_empty()
            }
            ServiceType::Wordpress => {
                !body.is_empty()
            }
            ServiceType::Gotosocial => {
                body.contains("name\":\"gotosocial")
            }
        }
    }
}