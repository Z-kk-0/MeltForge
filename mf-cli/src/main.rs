mod cli;
mod commands;
mod util;

use std::{path::PathBuf, process::exit};

use clap::Parser;
use commands::dispatch_command;
use mf_core::validation::error::MeltforgeError;
use mf_core::settings::initsettings;

fn main() -> Result<(), MeltforgeError> {
    util::plugins::print_scanned_plugins("plugins")?;
    let settings_path = PathBuf::from("settings.toml");
    if !settings_path.exists() {
        initsettings::init_settings()?;
    }
    let cli = cli::Cli::parse();
    let exit_code = match dispatch_command(cli) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("Error: {}", error);
            error.exit_code() as i32
        }
    };
    exit(exit_code);
}
