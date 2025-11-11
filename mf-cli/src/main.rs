mod cli;
mod commands;
mod util;

use std::process::exit;

use clap::Parser;
use commands::dispatch_command;
use mf_core::validation::error::MeltforgeError;

fn main() -> Result<(), MeltforgeError> {
    util::plugins::print_scanned_plugins("plugins")?;

    let cli = cli::Cli::parse();
    let exit_code = match dispatch_command(cli) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error: {}", e);
            e.exit_code() as i32
        }
    };
    exit(exit_code);
}
