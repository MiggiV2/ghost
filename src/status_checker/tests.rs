#[cfg(test)]
mod checker_tests {
    use crate::status_checker::config_builder::ConfBuilder;

    #[test]
    fn test_all() {
        let config = ConfBuilder::new().build();

        assert_eq!(config.services.len(), 7);

        for service in config.services {
            assert!(!service.get_url().is_empty());
            let is_okay = tokio_test::block_on(service.is_okay());
            assert!(is_okay, "Services {} is not okay", service.get_type());
        }
    }
}