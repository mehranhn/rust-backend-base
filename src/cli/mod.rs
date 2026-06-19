use clap::{Parser, Subcommand, Args};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run the server
    Run {
        /// Path to config file
        #[arg(short, long, value_name = "FILE")]
        // #[cfg_attr(debug_assertions, arg(default_value_t = String::from(concat!(env!("CARGO_MANIFEST_DIR"), "/example_config/config.ron"))))]
        // #[cfg_attr(not(debug_assertions), arg(default_value_t = String::from(concat!("/etc/backend/config.ron"))))]
        config: Option<String>,
    },
    /// hash password
    #[command(subcommand)]
    HashPassword(HashPasswordCommands),
}

#[derive(Debug, Subcommand)]
pub enum HashPasswordCommands {
    /// computes sha256 hash for a password
    Sha256(Password),
}

#[derive(Debug, Args)]
pub struct Password {
    pub username: String,
    pub password: String,
}
