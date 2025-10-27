// TODO: Implement right click context menu support on windows: Sort with Iris
mod cli;
mod paths;
mod config;
mod core;

use clap::Parser;
use cli::cli_parser::{Cli, Commands, ConfigAction};
use colored::Colorize;
use config::{config_init, config_edit, config_reset, config_show, config_parser, config_validator, config_processor};
use config_processor::IrisConfig;
use core::{resolver::target_resolver, sort::sort};

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
            ConfigAction::Reset { noconfirm} => {
                handle_result(config_reset::reset_config(*noconfirm));
            }
        },
        Commands::Update => {
            println!("Updating Iris");
        },

        // Commands that require a valid config
        Commands::Sort { path } => {
            // parse the config
            let value = match config_parser::parse_config() {
                Ok(v) => {
                    println!("{}", "Successfully parsed config file, validating...".green());
                    v
                },
                Err(e) => {
                    eprintln!("failed to parse config file. error: {}", e);
                    std::process::exit(1);
                }
            };

            // validate the config
            if let Err(err_msg) = config_validator::validate_config(&value) {
                eprintln!("Config file is invalid. Error: {}", err_msg);
                std::process::exit(1);
            }

            println!("{}", "Config file is valid".green());

            // process the config into IrisConfig struct
            let iris_config = match IrisConfig::from_value(&value) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("error processing config:\n {}", e);
                    std::process::exit(1);
                }
            };
            println!("{}", "Config processed successfully".green());

            // resolve the actual target path based on config and CLI args
            let target_path = match target_resolver::resolve_target(&iris_config, path.as_ref()) {
                Ok(p ) => p,
                Err(e) => {
                    eprintln!("error: {}", e);
                    std::process::exit(1);
                }
            };
            if let Err(e) = sort::sort(target_path.as_path(), &iris_config) {
                eprintln!("{}", format!("Error: {}", e).red());
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

fn handle_result<T, E: std::fmt::Display>(res: Result<T, E>) -> Option<T> {
    match res {
        Ok(val) => Some(val),
        Err(e) => {
            use colored::Colorize;
            eprintln!("{}", format!("Error: {}", e).red());
            None
        }
    }
}