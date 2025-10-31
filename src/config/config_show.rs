/// Display the config file contents with `iris config show`
use std::process::Command;
use crate::paths::config_path::get_config_path;

/// Use bat if available on system or print config file contents directly
pub fn show_config() -> Result<(), std::io::Error> {
    let path = get_config_path();
    if which::which("bat").is_ok() {
        Command::new("bat")
            .arg("--paging=always")
            .arg("--style=plain")
            .arg(path)
            .status()?; // wait for bat to finish
    } else {
        let content = std::fs::read_to_string(get_config_path())?;
        println!("{}", content);
    }
    Ok(())
}