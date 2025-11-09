use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    /// Add SSH host
    Add,
    /// List SSH hosts
    Ls,
    /// Connect to SSH host
    #[command(visible_alias = "c")]
    #[command(visible_alias = "ssh")]
    Connect {
        #[arg(value_name = "HOST")]
        host: Option<String>,
    },
    /// Edit SSH host
    Edit {
        #[arg(value_name = "HOST")]
        host: Option<String>,
    },
    /// Remove SSH host
    #[command(visible_alias = "rm")]
    Remove {
        #[arg(value_name = "HOST")]
        host: Option<String>,
    },
}
