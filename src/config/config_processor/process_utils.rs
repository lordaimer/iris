use std::path::PathBuf;
use crate::paths::config_path::get_config_path;

pub fn resolve_path(path_str: &str) -> Option<PathBuf> {
    if path_str.trim().is_empty() {
        return None;
    }

    let mut path = if path_str.starts_with("~/") {
        dirs::home_dir()?.join(&path_str[2..])
    } else if path_str.starts_with("./") {
        let config_path = get_config_path();
        let config_dir = config_path.parent()?;
        config_dir.join(path_str.trim_start_matches("./"))
    } else {
        PathBuf::from(path_str)
    };

    // Normalize separators (Windows -> use `\`, Unix -> `/`)
    #[cfg(windows)]
    {
        use std::path::MAIN_SEPARATOR;
        let s = path.to_string_lossy().replace(['/', '\\'], &MAIN_SEPARATOR.to_string());
        path = PathBuf::from(s);
    }

    Some(path)
}