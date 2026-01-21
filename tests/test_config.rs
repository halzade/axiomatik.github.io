use axiomatik_web::configuration;

#[test]
fn test_get_config() {
    let config = configuration::get_config();
    assert!(config.is_ok(), "Failed to read configuration: {:?}", config.err());
}
