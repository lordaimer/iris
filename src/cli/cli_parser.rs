use clap::{crate_authors, crate_description, crate_name, crate_version, Parser, Subcommand};

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
    /// Manage shell completion
    Completions {
        #[command(subcommand)]
        action: ShellCompletionAction,
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

#[derive(Subcommand, Debug)]
pub enum ShellCompletionAction {
    /// Automatically install shell completion
    #[command(alias = "add")]
    Install,
    /// Uninstall shell completion
    #[command(alias = "remove")]
    Uninstall,
    /// Print bash completion script
    Bash,
    /// Print zsh completion script
    Zsh,
    /// Print powershell completion script
    Powershell,
    /// Print fish completion script
    Fish,
    /// Print elvish completion script
    Elvish,
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
