use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    /// Add a new SSH host entry to ~/.ssh/config
    Add,
    /// List SSH host aliases from ~/.ssh/config
    Ls,
    /// Connect to an SSH host by alias
    #[command(visible_alias = "c")]
    #[command(visible_alias = "ssh")]
    Connect {
        #[arg(value_name = "HOST")]
        host: Option<String>,
    },
    /// Edit an existing SSH host entry in $EDITOR
    Edit {
        #[arg(value_name = "HOST")]
        host: Option<String>,
    },
    /// Remove an SSH host entry from ~/.ssh/config
    #[command(visible_alias = "rm")]
    Remove {
        #[arg(value_name = "HOST")]
        host: Option<String>,
    },
}
