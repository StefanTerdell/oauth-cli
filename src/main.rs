mod cli;
mod commands;

use anyhow::*;
use cli::{Auth, Cli, Commands, Completions};
use commands::auth::{login, logout};
use commands::completions::{generate, install};
use commands::get_profile;

pub const APP_NAME: &str = env!("CARGO_BIN_NAME");

fn main() -> Result<()> {
    match Cli::parse_command() {
        Commands::Auth(auth_command) => match auth_command {
            Auth::Login => login(),
            Auth::Logout => logout(),
        },
        Commands::Profile => get_profile(),
        Commands::Completions(completions) => match completions {
            Completions::Generate { shell } => generate(shell),
            Completions::Install => install(),
        },
    }
}
