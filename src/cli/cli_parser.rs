use clap::{Parser, Subcommand, crate_name, crate_version, crate_authors, crate_description};

#[derive(Parser)]
#[command(
    name = crate_name!(),
    version = crate_version!(),
    author = crate_authors!(),
    about = crate_description!(),
    long_about = None,
)]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Sort files in a directory or use the config target if none is given.
    Sort {
        /// Directory to sort (optional unless target = "required").
        path: Option<String>,
    },
    /// Self-update iris
    Update,
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// Manage Windows context menu integration
    #[cfg(target_os = "windows")]
    Context {
        #[command(subcommand)]
        action: ContextAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Display the contents of config file
    Show,
    /// Edit the config file
    Edit,
    /// Reset the config file to defaults
    Reset {
        /// Skip confirmation prompt
        #[arg(short = 'y', long = "yes")]
        noconfirm: bool,
    },
}

#[cfg(target_os = "windows")]
#[derive(Subcommand, Debug)]
pub enum ContextAction {
    /// Install the Windows right-click menu entry
    #[command(alias = "add")]
    Install,
    /// Uninstall the Windows right-click menu entry
    #[command(alias = "remove")]
    Uninstall,
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