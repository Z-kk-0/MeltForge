use clap::{Parser, Subcommand, ValueHint};
use mf_core::plugin::scan_plugins;
use std::env::args;
use std::path::PathBuf;

use mf_core::convert::convert;
use mf_core::format::FormatType;
use mf_core::job::ConvertJob;
use mf_core::validation::error::{FormatError, IoError, MeltforgeError};

#[derive(Parser, Debug)]
#[command(name = "meltforge", version, about = "Universal converter")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Convert {
        #[arg(value_hint = ValueHint::FilePath)]
        input: PathBuf,

        #[arg(long = "to", value_name = "FORMAT", required = true)]
        to: String,

        #[arg(long = "output", short = 'o', value_hint = ValueHint::FilePath)]
        output: Option<PathBuf>,
    },
}

fn main() -> Result<(), MeltforgeError> {
    let plugins = scan_plugins("plugins")?;
    println!("Plugins found: {:?}", plugins.len());

    for man in &plugins {
        println!("- {} v{} at {}", man.name, man.version, man.path.display());
        println!("  inputs:  {:?}", man.inputs);
        println!("  outputs: {:?}", man.outputs);
    }

    let cli = Cli::parse();

    let exit_code = match cli.command {
        Commands::Convert { input, to, output } => {
            println!("input : {}", input.display());
            println!("to    : {}", to);
            if let Some(p) = &output {
                println!("output: {}", p.display());
            }

            fn parse_format(to: &str) -> Result<FormatType, MeltforgeError> {
                match to.to_lowercase().as_str() {
                    "jpg" | "jpeg" => Ok(FormatType::JPEG),
                    "png" => Ok(FormatType::PNG),
                    _ => Err(MeltforgeError::Format(FormatError::UnsupportedOutput(
                        to.to_string(),
                    ))),
                }
            }
            let format_type = parse_format(&to)?;

            let job = ConvertJob {
                input,
                output,
                format_type,
            };
            match convert(job) {
                Ok(out_path) => {
                    println!("Conversion was successful");
                    println!("{}", out_path.display());
                    0
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    if let MeltforgeError::Io(ioe) = &e {
                        match ioe {
                            IoError::AlreadyExists(p) => {
                                eprintln!("File already exists: {}", p.display())
                            }
                            IoError::MissingParent(p) => {
                                eprintln!("Target directory not found: {}", p.display())
                            }
                            IoError::PermissionDenied(p) => {
                                eprintln!("No permission for: {}", p.display())
                            }
                            _ => {}
                        }
                    }
                    e.exit_code()
                }
            }
        }
    };

    std::process::exit(exit_code.into());
}
