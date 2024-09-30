use clap::{Parser, Subcommand};
use clap_complete::Shell;

use crate::APP_NAME;

#[derive(Parser)]
#[command(name = APP_NAME)]
#[command(about = "A simple CLI for OAuth authentication and profile management")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Commands to handle logging in and out", subcommand)]
    Auth(Auth),
    #[command(
        about = "Commands to help generate or install tab completions",
        subcommand
    )]
    Completions(Completions),
    Profile,
}

#[derive(Subcommand)]
pub enum Auth {
    #[command(about = "Opens your browser so you can log in")]
    Login,
    Logout,
}

#[derive(Subcommand)]
pub enum Completions {
    #[command(about = "Writes completion script to standard out")]
    Generate {
        #[arg(long, short, help = "Optionally supply a target shell")]
        shell: Option<Shell>,
    },
    #[command(about = "Attempts to install completion script to the current shell")]
    Install,
}

impl Cli {
    pub fn parse_command() -> Commands {
        Cli::parse().command
    }
}
