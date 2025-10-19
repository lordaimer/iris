use std::collections::HashMap;
use toml::Value;
use super::ValidationError;

pub fn validate_presets(value: &Value) -> Result<(), ValidationError> {
    // Check if "preset" table exists
    let presets = match value.get("preset") {
        Some(Value::Table(t)) => t,
        _ => {
            return Err(ValidationError::MissingSection {
                section: "presets",
            });
        }
    };

    // Check if [preset] is empty
    if presets.is_empty() {
        return Err(ValidationError::NoEntries {
            section: "presets".to_string(),
        });
    }

    let mut allowed_entries: HashMap<&str, (bool, Option<Vec<&str>>)> = HashMap::new();
    allowed_entries.insert("enabled", (true, Some(vec!["true", "false"])));
    allowed_entries.insert("extension", (true, None));
    allowed_entries.insert("relative_path", (true, None));
    allowed_entries.insert("absolute_path", (true, None));

    // iterate through each preset like [preset.docs]
    for (preset_name, preset_value) in presets {
        let preset_table = match preset_value {
            Value::Table(t) => t,
            _ => continue,
        };

        // check if preset table is empty
        if preset_table.is_empty() {
            return Err(ValidationError::NoEntries {
                section: format!("[preset.{}]", preset_name),
            });
        }

        // validate each key-value pair
        for (key, value) in preset_table {
            if !allowed_entries.contains_key(key.as_str()) {
                return Err(ValidationError::InvalidKey {
                    preset: preset_name.to_string(),
                    key: key.clone(),
                });
            }

            let (_, valid) = &allowed_entries[key.as_str()];

            if let Some(valid) = valid {
                if let Value::String(s) = value {
                    if !valid.contains(&s.as_str()) {
                        return Err(ValidationError::InvalidValue {
                            key: key.clone(),
                            value: s.clone(),
                        });
                    }
                } else if let Value::Boolean(b) = value {
                    let bool_str = if *b { "true" } else { "false" };
                    if !valid.contains(&bool_str) {
                        return Err(ValidationError::InvalidValue {
                            key: key.clone(),
                            value: bool_str.to_string(),
                        });
                    }
                }
            }

            if key == "extension" {
                if !value.is_array() {
                    return Err(ValidationError::InvalidValue {
                        key: key.clone(),
                        value: "must be an array".to_string(),
                    });
                }

                // check array is non-empty and contains only strings
                if let Value::Array(arr) = value {
                    if arr.is_empty() {
                        return Err(ValidationError::InvalidValue {
                            key: key.clone(),
                            value: "array cannot be empty".to_string(),
                        });
                    }

                    for item in arr {
                        if !item.is_str() {
                            return Err(ValidationError::InvalidValue {
                                key: key.clone(),
                                value: "array must contain only strings".to_string(),
                            });
                        }
                    }
                }
            }
        }

        // check for missing required keys
        for (allowed_key, (required, _)) in &allowed_entries {
            if *required && !preset_table.contains_key(*allowed_key) {
                return Err(ValidationError::MissingKey {
                    key: allowed_key.to_string(),
                    section: format!("[preset.{}]", preset_name),
                });
            }
        }
    }

    Ok(())
}