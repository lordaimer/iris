fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn get_home_dir() -> String {
    let home_dir: std::path::PathBuf = dirs::home_dir().expect("Failed to get home directory");
    // Convert PathBuf to Cow<str> to String
    home_dir.to_string_lossy().into()
}

pub fn init_defaults(path: &std::path::PathBuf) -> std::io::Result<()> {
    let project_name = capitalize_first(env!("CARGO_PKG_NAME"));
    let version = env!("CARGO_PKG_VERSION");
    let home_dir = get_home_dir();

    let template = include_str!("../../assets/defaults/iris.toml");
    let content = template
        .replace("{project_name}", &project_name)
        .replace("{version}", &version)
        .replace("{author}", "lordaimer")
        .replace("{home_dir}", &home_dir);

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_init_defaults_creates_file_and_toml() {
        let tmp_file = std::env::temp_dir().join("test_iris_config.toml");

        if tmp_file.exists() {
            std::fs::remove_file(&tmp_file).unwrap();
        }

        super::init_defaults(&tmp_file).unwrap();

        assert!(tmp_file.exists());

        let content = std::fs::read_to_string(&tmp_file).unwrap();
        assert!(content.starts_with("# Iris"));

        let parsed: toml::Value = content.parse().unwrap();
        let general = parsed.get("general").unwrap();
        let org_mode = general.get("mode").unwrap().as_str().unwrap();
        assert_eq!(org_mode, "local");

        std::fs::remove_file(&tmp_file).unwrap();
    }

    #[test]
    fn test_capitalize_first() {
        assert_eq!(super::capitalize_first("iris"), "Iris");
        assert_eq!(super::capitalize_first(""), "");
        assert_eq!(super::capitalize_first("a"), "A");
        assert_eq!(super::capitalize_first("IRIS"), "IRIS");
    }
}