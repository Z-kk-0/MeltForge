use mf_core::convert::convert;
use mf_core::format::FormatType;
use mf_core::job::ConvertJob;
use mf_core::validation::error::{FormatError, IoError, MeltforgeError};
use std::path::PathBuf;

use crate::util::parsing::parse_convert_args;


pub fn run_convert_job(job: ConvertJob) -> Result<i32, MeltforgeError> {
    println!("input : {}", job.input.display());
    println!("to    : {:?}", job.format_type);
    if let Some(p) = &job.output {
        println!("output: {}", p.display());
    }

    match convert(job) {
        Ok(out_path) => {
            println!("Conversion was successful");
            println!("{}", out_path.display());
            Ok(0)
        }
        Err(e) => {
            eprintln!("Error: {e}");
            if let MeltforgeError::Io(ioe) = &e {
                match ioe {
                    IoError::AlreadyExists(p)   => eprintln!("File already exists: {}", p.display()),
                    IoError::MissingParent(p)   => eprintln!("Target directory not found: {}", p.display()),
                    IoError::PermissionDenied(p)=> eprintln!("No permission for: {}", p.display()),
                    _ => {}
                }
            }
            Ok(e.exit_code().into())
        }
    }
}




pub fn run_convert(input: Vec<PathBuf>, to: String, output: Option<PathBuf>) -> Result<i32, MeltforgeError> {
    let jobs = parse_convert_args(inputs, &to, output.clone())?;

    if jobs.len() > 1 {
    if let Some(ref out) = output {
        if !out.is_dir() {
            return Err(MeltforgeError::Io(IoError::InvalidOutput(format!(
                "Output '{}' must be a directory when converting multiple files",
                out.display()
            ))));
        }
    }
    run_multi_convert_job(jobs)?
}
}


pub fn run_multi_convert_job(cj: Vec<ConvertJob>) {
    for c in cj {
        run_convert_job(c);
    }
}
