pub struct HealthChecker {
    pub matrix_url: String,
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
}

#[cfg(test)]
mod checker_tests {
    use crate::status_checker::HealthChecker;

    #[test]
    fn test_one() {
        let checker = HealthChecker {
            matrix_url: String::from("https://matrix.familyhainz.de"),
        };
        let matrix_is_healthy = tokio_test::block_on(checker.check_matrix());
        assert!(matrix_is_healthy);
    }
}