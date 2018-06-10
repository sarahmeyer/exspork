extern crate exspork;

use exspork::read_cargo_toml;

#[test]
fn test_cargo_read() {
    let config = read_cargo_toml("Cargo.toml").unwrap();

    assert_eq!(config.package.name, "exspork".to_string());
    assert_eq!(config.package.version, "0.1.0".to_string());
    assert_eq!(config.package.license, Some("MIT/Apache-2.0".to_string()));
    assert_eq!(config.package.description, None);
}
