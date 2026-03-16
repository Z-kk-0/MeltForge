use mf_core::runner::{JobResult, run_jobs};
use mf_core::validation::error::{IoError, MeltforgeError};
use std::path::PathBuf;

use crate::util::parsing::parse_convert_args;

pub fn run_convert(
    inputs: Vec<PathBuf>,
    to: String,
    output: Option<PathBuf>,
    threads: Option<usize>,
) -> Result<i32, MeltforgeError> {
    let jobs = parse_convert_args(inputs, &to, output.clone())?;
    let total = jobs.len();

    if jobs.len() > 1 {
        if let Some(ref out) = output {
            if !out.exists() {
                std::fs::create_dir_all(out).map_err(|e| {
                    MeltforgeError::Io(IoError::InvalidOutput(format!(
                        "Failed to create directory '{}': {}",
                        out.display(),
                        e
                    )))
                })?;
            } else if !out.is_dir() {
                return Err(MeltforgeError::Io(IoError::InvalidOutput(format!(
                    "Output '{}' must be a directory when converting multiple files",
                    out.display()
                ))));
            }
        }
    }

    let results = run_jobs(jobs, threads)?;
    let failures = print_results(results);
    let successes = total.saturating_sub(failures);

    println!(
        "\nConversion completed: {} succeeded, {} failed",
        successes, failures
    );

    if failures > 0 { Ok(1) } else { Ok(0) }
}

fn print_results(results: Vec<JobResult>) -> usize {
    let mut failures = 0;
    for r in results {
        println!("input : {}", r.job.input.display());
        println!("to    : {:?}", r.job.format_type);
        match r.result {
            Ok(out_path) => {
                println!("output: {}", out_path.display());
                println!("Conversion successful");
            }
            Err(ref e) => {
                eprintln!("Error: {e}");
                if let MeltforgeError::Io(ioe) = e {
                    match ioe {
                        IoError::AlreadyExists(p) => eprintln!("File already exists: {}", p.display()),
                        IoError::MissingParent(p) => eprintln!("Target directory not found: {}", p.display()),
                        IoError::NotFound(p) => eprintln!("File not found: {}", p.display()),
                        IoError::PermissionDenied(p) => eprintln!("No permission for: {}", p.display()),
                        _ => {}
                    }
                }
                failures += 1;
            }
        }
        println!();
    }
    failures
}
