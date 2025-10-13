use clap::{Parser, Subcommand, crate_name, crate_version, crate_authors, crate_description};

#[derive(Parser)]
#[command(
    name = crate_name!(),
    version = crate_version!(),
    author = crate_authors!(),
    about = crate_description!(),
    long_about = None,
)]

// TODO: Implement -s as a shorthand alternative to sort
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Sort the files in the specified directory
    Sort {
        path: String,
    },
    /// Self-update iris
    Update, // TODO: Implement -u as a shorthand
    /// Manage configuration
    Config { // TODO: Implement -c as a shorthand
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Display the contents of config file
    Show,
    /// Edit the config file
    Edit, // TODO: Implement -ce as a shorthand
    /// Reset the config file to defaults
    Reset, // TODO: Implement -cr as a shorthand
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_verify_cli() {
        // this test verifies that CLI Structure is valid
        Cli::command().debug_assert();
    }
}