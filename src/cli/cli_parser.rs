use clap::{Parser, Subcommand, crate_name, crate_version, crate_authors, crate_description};

#[derive(Parser)]
#[command(
    name = crate_name!(),
    version = crate_version!(),
    author = crate_authors!(),
    about = crate_description!(),
    long_about = None,
    help_template = "\
Iris {version}
Intelligent CLI file organizer

USAGE:
  iris <COMMANDS> <FLAGS>

COMMANDS:
{subcommands}

OPTIONS:
{options}
"
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
    Update,
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