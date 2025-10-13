// Dynamically parse the contents of the main config file
use crate::paths::config_path::get_config_path;

// Parse the config file and pass it to the validator
pub fn parse_config() -> Result<toml::Value, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(get_config_path())?;
    let value: toml::Value = toml::from_str(&content)?;
    Ok(value)
}