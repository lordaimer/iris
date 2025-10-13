mod cli;
mod paths;
mod config;

use clap::Parser;
use cli::cli_parser::{Cli, Commands};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = paths::config_path::get_config_path();
    if !config_path.exists() {
        config::config_init::init_defaults(&config_path)?;
    }

    let mut args: Vec<String> = std::env::args().collect();

    // if first argument is "help", replace it with "--help"
    if args.len() > 1 && args[1] == "help" {
        args[1] = "--help".to_string();
    }

    // parse args using the overridden Command
    let cli = Cli::parse_from(&args);

    match cli.command {
        Commands::Sort { path } => println!("Sorting {}", path),
        Commands::Update => println!("updating iris..."),
    }
    Ok(())
}
