pub enum ServiceType {
    Synapse,
    Nextcloud,
    Forgejo,
    Portainer,
    Keycloak,
}

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

    pub async fn is_okay(&self) -> bool {
        let full_url = self.get_url() + self.get_endpoint().as_str();
        let response = reqwest::get(full_url).await;

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
        }
    }
}