mod cli;
mod paths;
mod config;

use clap::Parser;
use cli::cli_parser::{Cli, Commands, ConfigAction};
use colored::Colorize;
use config::{config_init, config_edit, config_reset, config_show, config_parser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = paths::config_path::get_config_path();
    if !config_path.exists() {
        config_init::init_defaults(&config_path)?;
    }

    if let Some(_v) = handle_result(config_parser::parse_config()) {
        println!("{}", "Successfully parsed config file, validating...".green())
    };

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
        Commands::Config { action } => match action {
            ConfigAction::Show => {
                handle_result(config_show::show_config());
            }
            ConfigAction::Edit => {
                handle_result(config_edit::edit_config());
            }
            ConfigAction::Reset => {
                handle_result(config_reset::reset_config());
            }
        },
    }
    Ok(())
}

fn handle_result<T, E: std::fmt::Display>(res: Result<T, E>) -> Option<T> {
    match res {
        Ok(val) => Some(val),
        Err(e) => {
            eprintln!("Error: {}", e);
            None
        }
    }
}
