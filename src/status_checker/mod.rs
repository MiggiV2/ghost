pub struct HealthChecker {
    pub matrix_url: String,
    pub nextcloud_url: String,
    pub forgejo_url: String,
    pub portainer_url: String,
}

impl HealthChecker {
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

#[cfg(test)]
mod checker_tests {
    use crate::status_checker::HealthChecker;

    #[test]
    fn test_matrix() {
        let checker = HealthChecker {
            matrix_url: String::from("https://matrix.familyhainz.de"),
            nextcloud_url: String::new(),
            forgejo_url: String::new(),
            portainer_url: String::new(),
        };
        let is_matrix_healthy = tokio_test::block_on(checker.check_matrix());
        assert!(is_matrix_healthy);
    }

    #[test]
    fn test_nextcloud() {
        let checker = HealthChecker {
            nextcloud_url: String::from("https://nextcloud.mymiggi.de"),
            matrix_url: String::new(),
            forgejo_url: String::new(),
            portainer_url: String::new(),
        };
        let is_nextcloud_healthy = tokio_test::block_on(checker.check_nextcloud());
        assert!(is_nextcloud_healthy);
    }

    #[test]
    fn test_forgejo() {
        let checker = HealthChecker {
            forgejo_url: String::from("https://gitea.familyhainz.de"),
            matrix_url: String::new(),
            nextcloud_url: String::new(),
            portainer_url: String::new(),
        };
        let is_nextcloud_healthy = tokio_test::block_on(checker.check_forgejo());
        assert!(is_nextcloud_healthy);
    }

    #[test]
    fn test_portainer() {
        let checker = HealthChecker {
            forgejo_url: String::new(),
            matrix_url: String::new(),
            nextcloud_url: String::new(),
            portainer_url: String::from("https://vmd116727.contaboserver.net"),
        };
        let is_nextcloud_healthy = tokio_test::block_on(checker.check_portainer());
        assert!(is_nextcloud_healthy);
    }
}