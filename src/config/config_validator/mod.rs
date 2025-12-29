#[cfg(test)]
mod tests;
mod validate_general;
mod validate_presets;

use toml::Value;
use validate_general::validate_general;
use validate_presets::validate_presets;

#[derive(Debug)]
pub enum ValidationError {
    MissingSection { section: &'static str },
    MissingKey { key: String, section: String },
    InvalidValue { key: String, value: String },
    InvalidKey { preset: String, key: String },
    NoEntries { section: String },
    NoEnabledPresets,
    Io(std::io::Error),
}

impl From<std::io::Error> for ValidationError {
    fn from(e: std::io::Error) -> Self {
        ValidationError::Io(e)
    }
}

// Implement the display trait for ConfigError variants
impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::MissingSection { section } => {
                write!(f, "missing {} section in config file", section)
            }
            ValidationError::MissingKey { key, section } => {
                write!(f, "missing key '{}' in {} section", key, section)
            }
            ValidationError::InvalidValue { key, value } => {
                write!(f, "invalid value '{}' for key '{}'", value, key)
            }
            ValidationError::InvalidKey { preset, key } => {
                write!(f, "invalid key '{}' in {}", key, preset)
            }
            ValidationError::NoEntries { section } => {
                write!(f, "no entries for {} section", section)
            }
            ValidationError::NoEnabledPresets => {
                write!(
                    f,
                    "There are no enabled presets in the [preset] section of the config file."
                )
            }
            ValidationError::Io(e) => {
                write!(f, "input/output error while validating config: {}", e)
            }
        }
    }
}

// Implement the error trait for ConfigError
impl std::error::Error for ValidationError {}

pub fn validate_config(value: &Value) -> Result<(), ValidationError> {
    validate_general(value)?;
    validate_presets(value)?;
    Ok(())
}
