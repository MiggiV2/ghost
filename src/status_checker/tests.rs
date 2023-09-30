#[cfg(test)]
mod checker_tests {
    use crate::status_checker::config_builder::ConfBuilder;

    #[test]
    fn test_all() {
        let config = ConfBuilder::new("checker.toml").build();

        assert_eq!(config.len(), 5);

        for service in config {
            let is_okay = tokio_test::block_on(service.is_okay());
            assert!(is_okay);
        }
    }
}