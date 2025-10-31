use clap::{Parser, ValueHint};
use mf_core::convert::convert;
use mf_core::error::MeltforgeError;
use mf_core::format::FormatType;
use mf_core::job::ConvertJob;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "meltforge", version, about = "Universal converter")]
struct Cli {
    #[arg(value_hint = ValueHint::FilePath)]
    input: PathBuf,

    #[arg(long = "to", value_name = "FORMAT")]
    to: String,

    #[arg(long = "output", short = 'o', value_hint = ValueHint::FilePath)]
    output: Option<PathBuf>,
}

fn main() {
    let args = Cli::parse();

    println!("input : {}", args.input.display());
    println!("output: {}", args.to);

    let format_type = match args.to.to_lowercase().as_str() {
        "jpg" | "jpeg" => FormatType::JPEG,
        "png" => FormatType::PNG,
        _ => {
            eprintln!("Unsupported format: {}", args.to);
            std::process::exit(3);
        }
    };

    let job = ConvertJob {
        input: args.input.clone(),
        output: args.output.clone(),
        format_type,
    };
    match convert(job) {
        Ok(out_path) => {
            println!("Conversion was successfull");
            println!("{}", out_path.display())
        }
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(e.exit_code().into());
        }
    }
}
