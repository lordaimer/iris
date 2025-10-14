mod cli;
mod paths;
mod config;

use clap::Parser;
use cli::cli_parser::{Cli, Commands, ConfigAction};
use colored::Colorize;
use config::{config_init, config_edit, config_reset, config_show, config_parser, config_validator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Config file path
    let config_path = paths::config_path::get_config_path();
    // Check if config file path exists, if not initialize a default file
    if !config_path.exists() {
        config_init::init_defaults(&config_path)?;
    }

    let mut args: Vec<String> = std::env::args().collect();

    // if first argument is "help", replace it with "--help"
    if args.len() > 1 && args[1] == "help" {
        args[1] = "--help".to_string();
    }

    // parse args using the overridden Command
    let cli = Cli::parse_from(&args);

    match &cli.command {
        // Config commands do NOT require a valid config
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
        Commands::Update => {
            println!("Updating Iris");
        },

        // Commands that require a valid config
        Commands::Sort { .. } => {
            // parse the config
            let value = match config_parser::parse_config() {
                Ok(v) => {
                    println!("{}", "Successfully parsed config file, validating...".green());
                    v
                },
                Err(_) => {
                    eprintln!("failed to parse config file.");
                    std::process::exit(1);
                }
            };

            // validate the config
            if let Err(err_msg) = config_validator::validate_config(&value) {
                eprintln!("error: {}", err_msg);
                std::process::exit(1);
            }

            println!("{}", "Config file is valid".green());

            match &cli.command {
                Commands::Sort { path } => println!("Sorting {}", path),
                _ => unreachable!(),
            }
        }
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
