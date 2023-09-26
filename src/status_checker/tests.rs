#[cfg(test)]
mod checker_tests {
    use crate::status_checker::HealthChecker;

    #[test]
    fn test_all() {
        let checker = HealthChecker {
            matrix_url: String::from("https://matrix.familyhainz.de"),
            nextcloud_url: String::from("https://nextcloud.mymiggi.de"),
            forgejo_url: String::from("https://gitea.familyhainz.de"),
            portainer_url: String::from("https://vmd116727.contaboserver.net"),
            keycloak_url: String::from("https://auth.familyhainz.de"),
        };

        assert!(tokio_test::block_on(checker.check_portainer()));
        assert!(tokio_test::block_on(checker.check_forgejo()));
        assert!(tokio_test::block_on(checker.check_nextcloud()));
        assert!(tokio_test::block_on(checker.check_matrix()));
    }
}