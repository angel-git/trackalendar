use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub title: String,
}

pub fn parse_config() -> Config {
    let config_file = std::fs::read_to_string("config.toml")
        .expect("Failed to read file configuration file: config.toml");
    let config: Config = toml::from_str(config_file.as_str())
        .expect("Failed to parse configuration file: config.toml");
    config
}
