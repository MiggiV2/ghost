mod tests;

pub struct HealthChecker {
    pub matrix_url: String,
    pub nextcloud_url: String,
    pub forgejo_url: String,
    pub portainer_url: String,
    pub keycloak_url: String,
}

impl HealthChecker {
    pub fn new(matrix_url: &str, nextcloud_url: &str, forgejo_url: &str, portainer_url: &str,
               keycloak_url: &str) -> Self {
        Self {
            matrix_url: matrix_url.to_string(),
            nextcloud_url: nextcloud_url.to_string(),
            forgejo_url: forgejo_url.to_string(),
            portainer_url: portainer_url.to_string(),
            keycloak_url: keycloak_url.to_string(),
        }
    }

    pub async fn check_matrix(&self) -> bool {
        let resp = reqwest::get(format!("{}/health", &self.matrix_url))
            .await
            .expect("Request failed!");

        if !resp.status().is_success() {
            eprintln!("Unexpected status code: {}", resp.status());
            return false;
        }

        let body = resp.text().await.unwrap_or_default();
        if body != "OK" {
            eprintln!("Unexpected body: {}", body);
            return false;
        }

        return true;
    }

    pub async fn check_nextcloud(&self) -> bool {
        let resp = reqwest::get(format!("{}/status.php", &self.nextcloud_url))
            .await
            .expect("Request failed!");

        if !resp.status().is_success() {
            eprintln!("Unexpected status code: {}", resp.status());
            return false;
        }

        let body = resp.text().await.unwrap_or_default();
        if !body.contains("\"installed\":true") || !body.contains("\"productname\":\"Nextcloud\"") {
            eprintln!("Unexpected body: {}", body);
            return false;
        }

        return true;
    }

    pub async fn check_forgejo(&self) -> bool {
        let resp = reqwest::get(format!("{}/api/healthz", &self.forgejo_url))
            .await
            .expect("Request failed!");

        if !resp.status().is_success() {
            eprintln!("Unexpected status code: {}", resp.status());
            return false;
        }

        let body = resp.text().await.unwrap_or_default().replace("\n", "").replace(" ", "");
        if !body.starts_with("{\"status\":\"pass\"") {
            eprintln!("Unexpected body: {}", body);
            return false;
        }

        return true;
    }

    pub async fn check_portainer(&self) -> bool {
        let resp = reqwest::get(format!("{}/api/system/status", &self.portainer_url))
            .await
            .expect("Request failed!");

        if !resp.status().is_success() {
            eprintln!("Unexpected status code: {}", resp.status());
            return false;
        }

        let body = resp.text().await.unwrap_or_default();
        if !body.contains("Version") || !body.contains("InstanceID") {
            eprintln!("Unexpected body: {}", body);
            return false;
        }

        return true;
    }

    /// 1 1 1 1 -> 15
    /// 1 0 1 1 -> 11
    pub fn get_status_id(&self, is_alive_1: bool, is_alive_2: bool, is_alive_3: bool, is_alive_4: bool) -> u8 {
        let mut code = 0;
        if is_alive_1 {
            code += 8;
        }
        if is_alive_2 {
            code += 4;
        }
        if is_alive_3 {
            code += 2;
        }
        if is_alive_4 {
            code += 1;
        }
        code
    }
}