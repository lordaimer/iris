#[cfg(test)]
use super::*;
use toml::Value;

// Helper to parse TOML string
fn parse_toml(toml: &str) -> Value {
    toml::from_str(toml).unwrap()
}

// ===== GENERAL SECTION TESTS =====
#[test]
fn general_valid_cases() {
    let minimal = r#"
[general]
mode = "relative"
"#;
    let full = r#"
[general]
mode = "absolute"
target = "downloads"
presets_path = "./presets"
"#;

    // Minimal required keys
    validate_general(&parse_toml(minimal)).unwrap();
    // All keys present
    validate_general(&parse_toml(full)).unwrap();
}

#[test]
fn general_missing_required_key() {
    let toml = r#"
[general]
target = "downloads"
"#;
    let result = validate_general(&parse_toml(toml));
    assert!(matches!(result, Err(ValidationError::MissingKey { key: _, section: _ })));
}

#[test]
fn general_invalid_value() {
    let toml = r#"
[general]
mode = "invalid"
"#;
    let result = validate_general(&parse_toml(toml));
    assert!(matches!(result, Err(ValidationError::InvalidValue { key: _, value: _ })));
}

#[test]
fn general_extra_invalid_key() {
    let toml = r#"
[general]
mode = "relative"
extra = "oops"
"#;
    let result = validate_general(&parse_toml(toml));
    assert!(matches!(result, Err(ValidationError::InvalidKey { preset: _, key: _ })));
}

#[test]
fn general_empty_section() {
    let toml = r#"[general]"#;
    let result = validate_general(&parse_toml(toml));
    assert!(matches!(result, Err(ValidationError::NoEntries { section: _ })));
}

// ===== PRESETS SECTION TESTS =====
#[test]
fn presets_valid_case() {
    let toml = r#"
        [preset.docs]
        enabled = true
        extension = ["txt", "pdf"]
        relative_path = "docs"
        absolute_path = "/home/user/docs"
    "#;
    validate_presets(&parse_toml(toml)).unwrap();
}

#[test]
fn presets_missing_required_key() {
    let toml = r#"
        [preset.docs]
        extension = ["txt"]
        relative_path = "docs"
        absolute_path = "/home/user/docs"
    "#;
    let result = validate_presets(&parse_toml(toml));
    assert!(matches!(result, Err(ValidationError::MissingKey { key: _, section: _ })));
}

#[test]
fn presets_invalid_value() {
    let toml = r#"
        [preset.docs]
        enabled = "maybe"
        extension = ["txt"]
        relative_path = "docs"
        absolute_path = "/home/user/docs"
    "#;
    let result = validate_presets(&parse_toml(toml));
    assert!(matches!(result, Err(ValidationError::InvalidValue { key: _, value: _ })));
}

#[test]
fn presets_extra_invalid_key() {
    let toml = r#"
        [preset.docs]
        enabled = true
        extension = ["txt"]
        relative_path = "docs"
        absolute_path = "/home/user/docs"
        foo = "bar"
    "#;
    let result = validate_presets(&parse_toml(toml));
    assert!(matches!(result, Err(ValidationError::InvalidKey { preset: _, key: _ })));
}

#[test]
fn presets_empty_table() {
    let toml = r#"[preset.docs]"#;
    let result = validate_presets(&parse_toml(toml));
    assert!(matches!(result, Err(ValidationError::NoEntries { section: _ })));
}

#[test]
fn presets_no_presets_table() {
    let toml = r#""#; // empty TOML
    let result = validate_presets(&parse_toml(toml));
    assert!(matches!(result, Err(ValidationError::MissingSection { section: _ })));
}
