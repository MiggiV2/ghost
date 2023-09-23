pub struct HealthChecker {
    pub matrix_url: String,
    pub nextcloud_url: String,
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

        println!("Homeserver looks healthy!");
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
            return false;
        }

        return true;
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
        };
        let is_matrix_healthy = tokio_test::block_on(checker.check_matrix());
        assert!(is_matrix_healthy);
    }

    #[test]
    fn test_nextcloud() {
        let checker = HealthChecker {
            nextcloud_url: String::from("https://nextcloud.mymiggi.de"),
            matrix_url: String::new(),
        };
        let is_nextcloud_healthy = tokio_test::block_on(checker.check_nextcloud());
        assert!(is_nextcloud_healthy);
    }
}