use clap::{Parser, Subcommand, ValueHint};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "meltforge", version, about = "Universal converter")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Convert One or multiple files
    Convert {
        #[arg(value_hint = ValueHint::FilePath)]
        input: Vec<PathBuf>,

        #[arg(long = "to", value_name = "FORMAT", required = true)]
        to: String,

        #[arg(long = "output", short = 'o', value_hint = ValueHint::FilePath)]
        output: Option<PathBuf>,

        #[arg(long = "jobs", short = 'j', value_name = "N")]
        jobs: Option<usize>,
    },
}
