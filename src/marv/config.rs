use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{fs, sync::Mutex};
use toml;

#[derive(Deserialize, Debug, Clone)]
pub struct MarvConfig {
    pub hostname: String,
    pub nickname: String,
    pub channel: String,
    pub broker: String,
    pub topic: String,
    pub group: String,
    pub messages_log: String,
    pub database_url: String,
    pub mode: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MarvSetup {
    pub config: MarvConfig,
}

pub fn read_configuration() -> Result<MarvSetup, Box<dyn std::error::Error>> {
    let toml_str = fs::read_to_string("Marv.toml")?;
    let config: MarvSetup = toml::from_str(&toml_str)?;

    return Ok(config);
}

#[test]
fn test_read_configuration() -> Result<(), Box<dyn std::error::Error>> {
    let setup = read_configuration()?;
    let config = &setup.config;

    assert_eq!(config.hostname, "127.0.0.1:6667");
    assert_eq!(config.nickname, "marvy");
    assert_eq!(config.channel, "#acme");
    assert_eq!(config.broker, "localhost:9092");
    assert_eq!(config.topic, "MARV.MESSAGES");
    assert_eq!(config.group, "MARV");
    assert_eq!(config.messages_log, "messages.txt");
    assert_eq!(config.messages_log, "thread");
    assert_eq!(
        config.database_url,
        "postgres://deploy42:deploy42@localhost:5432/deploy42"
    );

    Ok(())
}

pub static CONFIG: Lazy<Mutex<MarvSetup>> = Lazy::new(|| {
    Mutex::new(
        read_configuration().expect("Problems trying to process Marv.toml configuration file"),
    )
});
