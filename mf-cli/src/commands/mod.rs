use crate::cli::{Cli, Commands};
use mf_core::validation::error::MeltforgeError;

mod convert;
pub use convert::run_convert;

pub fn dispatch_command(cli: Cli) -> Result<i32, MeltforgeError> {
    match cli.command {
        Commands::Convert {
            input,
            to,
            output,
            jobs,
        } => run_convert(input, to, output, jobs),
    }
}
