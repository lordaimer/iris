// TODO: if two presets with same extension are enabled, they should have different relative_path and absolute_path
// TODO: implement a precedence system where the top most preset has the highest precedence
// TODO: implement a duplicate entry error for duplicate keys in a preset
// TODO: support for nested presets like [preset.docs.txt] for txt in /documents/txt
// TODO: implement a duplicate entry error for duplicate extensions in the same preset
// TODO: A global "*" catch-all extension support to sort files which don't match any presets into a misc folder
// TODO: support for recursive option which would recursively sort files inside a target directory
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
    allowed_entries.insert("enabled", (true, None));
    allowed_entries.insert("extension", (true, None));
    allowed_entries.insert("relative_path", (true, None));
    allowed_entries.insert("absolute_path", (true, None));

    let mut has_enabled = false;

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

            // strict type validation
            match key.as_str() {
                "enabled" => {
                    if let Value::Boolean(b) = value {
                        if *b {
                            has_enabled = true;
                        }
                    } else {
                        return Err(ValidationError::InvalidValue {
                            key: key.clone(),
                            value: value.to_string(),
                        });
                    }
                }
                "extension" => {
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
                    } else {
                        return Err(ValidationError::InvalidValue {
                            key: key.clone(),
                            value: "must be an array".to_string(),
                        });
                    }
                }
                "relative_path" | "absolute_path" => {
                    if !value.is_str() {
                        return Err(ValidationError::InvalidValue {
                            key: key.clone(),
                            value: value.to_string(),
                        });
                    }
                }
                _ => {}
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

    // check if at least one preset is enabled
    if !has_enabled {
        return Err(ValidationError::NoEnabledPresets);
    }

    Ok(())
}