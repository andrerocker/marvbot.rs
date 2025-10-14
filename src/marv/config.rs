use serde::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub hostname: String,
    pub nickname: String,
    pub channel: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MarvSetup {
    pub config: Config,
}

pub fn read_configuration() -> Result<MarvSetup, Box<dyn std::error::Error>> {
    let toml_str = fs::read_to_string("Marv.toml")?;
    let config: MarvSetup = toml::from_str(&toml_str)?;

    return Ok(config);
}
