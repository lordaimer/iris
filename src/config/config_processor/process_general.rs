use super::{process_utils::resolve_path, GeneralConfig, Mode, Target};
use toml::Value;

pub fn process_general(value: &Value) -> GeneralConfig {
    let general = value.get("general").unwrap().as_table().unwrap();

    let mode = match general.get("mode").unwrap().as_str().unwrap() {
        "relative" => Mode::Relative,
        "absolute" => Mode::Absolute,
        _ => unreachable!(), // already validated
    };

    let target = general
        .get("target")
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .map(|t| match t {
            "downloads" => Target::Downloads,
            "current" => Target::CurrentDir,
            "required" => Target::Required,
            _ => unreachable!(), // already validated
        });

    let presets_path = general
        .get("presets_path")
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .map(resolve_path)
        .unwrap_or_else(|| None);

    GeneralConfig {
        target,
        mode,
        presets_path,
    }
}
