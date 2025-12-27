use super::PresetConfig;
use crate::config::config_processor::process_utils::resolve_path;
use toml::Value;
pub fn process_presets(value: &Value) -> Vec<PresetConfig> {
    let mut presets = Vec::new();
    let Some(presets_table) = value.get("preset").and_then(Value::as_table) else {
        return presets;
    };

    for (name, table_value) in presets_table {
        let table = table_value.as_table().unwrap();
        presets.push(PresetConfig {
            name: name.to_string(),
            enabled: table.get("enabled").unwrap().as_bool().unwrap(),
            extension: table
                .get("extension")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .map(|v| v.as_str().unwrap().to_string())
                        .collect()
                })
                .unwrap_or_else(Vec::new),
            relative_path: table
                .get("relative_path")
                .and_then(|v| v.as_str()) // Option<&str>
                .filter(|s| !s.is_empty()) // filter out empty strings
                .map(resolve_path)
                .unwrap_or_else(|| None), // convert &str to String
            absolute_path: table
                .get("absolute_path")
                .and_then(|v| v.as_str()) // Option<&str>
                .filter(|s| !s.is_empty()) // filter out empty strings
                .map(resolve_path)
                .unwrap_or_else(|| None),
        });
    }

    presets
}
