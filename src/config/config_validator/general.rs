use std::collections::HashMap;
use toml::Value;
use super::ValidationError;
pub fn validate_general(value: &Value) -> Result<(), ValidationError> {
    // Check if "general" table exists
    let general = match value.get("general") {
        Some(Value::Table(general)) => general,
        _ => return Err(ValidationError::MissingSection {
            section: "general"
        }),
    };

    if general.is_empty() {
        return Err(ValidationError::NoEntries {
            section: "general".to_string(),
        })
    }

    let mut allowed_entries: HashMap<&str, (bool, Option<Vec<&str>>)> = HashMap::new();
    allowed_entries.insert("target", (false, Some(vec!["required", "downloads", "current"])));
    allowed_entries.insert("mode", (true, Some(vec!["relative", "absolute"])));
    allowed_entries.insert("presets_path", (false, None));


    // iterate through fields in general
    for (key, value) in general {
        if !allowed_entries.contains_key(key.as_str()) {
            return Err(ValidationError::InvalidKey {
                preset: "general".to_string(),
                key: key.to_string(),
            })
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
            }
        }
    }

    for (allowed_key, (required, _)) in &allowed_entries {
        if *required && !general.contains_key(*allowed_key) {
            return Err(ValidationError::MissingKey {
                key: allowed_key.to_string(),
                section: "general".to_string(),
            })
        }
    }
    Ok(())
}