use std::path::PathBuf;
use std::fs;
use dirs;

/// Get platform-specific user config directory path
fn get_config_dir() -> PathBuf {
    let dir = if cfg!(target_os = "windows") {
        dirs::config_dir().expect("Cannot determine config directory on Windows")
    } else if cfg!(target_os = "linux") {
        dirs::config_dir().expect("Cannot determine config directory on Linux")
    } else if cfg!(target_os = "macos") {
        dirs::config_dir().expect("Cannot determine config directory on macOS")
    } else {
        panic!("Unsupported OS");
    };

    if cfg!(target_os = "windows") {
        dir.join("Iris")
    } else if cfg!(target_os = "macos") {
        dir.join("Iris")
    } else {
        dir.join("iris")
    }
}

/// Get the full config file path and create directories if missing
pub fn get_config_path() -> PathBuf {
    let dir = get_config_dir();
    fs::create_dir_all(&dir).expect("Failed to create config directory");
    dir.join("iris.toml")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config_dir() {
        let dir = get_config_dir();
        // just verify it returns a path - actual path depends on OS
        assert!(!dir.as_os_str().is_empty());
    }

    #[test]
    fn test_get_config_path() {
        let path = get_config_path();
        assert!(path.ends_with("iris.toml"));
    }
}