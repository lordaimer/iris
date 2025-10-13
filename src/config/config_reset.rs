/// Reset the config file to defaults with `iris config reset`
use std::io::{self, Write};
use colored::Colorize;
use crate::paths::config_path::get_config_path;
use crate::config::config_init::init_defaults;


/// Reset the config file to defaults based on user prompt
pub fn reset_config() -> Result<(), Box<dyn std::error::Error>> {
    if !confirm("Are you sure you want to reset the config?") {
        println!("{}", "Aborted.".yellow());
        return Ok(());
    }

    let config_path = get_config_path();
    if config_path.exists() {
        std::fs::remove_file(&config_path)?;
    }
    if let Ok(()) = init_defaults(&config_path) {
        println!("{}", "Successfully reset the config file.".green());
    };
    Ok(())
}

/// Confirmation for resetting config file to defaults
fn confirm(prompt: &str) -> bool {
    print!("{} [Y/N]: ", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
    } else {
        false
    }
}

// TODO: Need to write tests for config_reset