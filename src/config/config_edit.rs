use crate::paths::config_path::get_config_path;
/// Edit the config file with `iris config edit`
use edit::{edit_file, get_editor};

/// Edit the config file in preferred editor
pub fn edit_config() -> Result<(), Box<dyn std::error::Error>> {
    let path = get_config_path();
    let editor_path = get_editor()?;
    let editor_name = editor_path
        .file_name()
        .map(|n| n.to_string_lossy())
        .unwrap_or_else(|| editor_path.to_string_lossy());

    println!(
        "editing config file: {} with {}",
        path.display(),
        editor_name
    );

    if let Err(e) = edit_file(&path) {
        eprintln!("editing failed: {}", e);
    }

    Ok(())
}
