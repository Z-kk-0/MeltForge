mod cli;
mod commands;
mod util;

use clap::Parser;
use commands::dispatch_command;
use mf_core::validation::error::MeltforgeError;

fn main() -> Result<(), MeltforgeError> {

    util::plugins::print_scanned_plugins("plugins")?;

    let cli = cli::Cli::parse();
    let exit_code = dispatch_command(cli)?;
    std::process::exit(exit_code as i32);
}
