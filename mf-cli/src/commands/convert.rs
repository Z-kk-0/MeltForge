use mf_core::convert::convert;
use mf_core::format::FormatType;
use mf_core::job::ConvertJob;
use mf_core::validation::error::{FormatError, IoError, MeltforgeError};
use rayon::ThreadPoolBuilder;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::path::PathBuf;
use std::usize;

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
                    IoError::AlreadyExists(p) => eprintln!("File already exists: {}", p.display()),
                    IoError::MissingParent(p) => {
                        eprintln!("Target directory not found: {}", p.display())
                    }
                    IoError::PermissionDenied(p) => eprintln!("No permission for: {}", p.display()),
                    _ => {}
                }
            }
            Ok(e.exit_code().into())
        }
    }
}

pub fn run_convert(
    inputs: Vec<PathBuf>,
    to: String,
    output: Option<PathBuf>,
    threads: Option<usize>,
) -> Result<i32, MeltforgeError> {
    let jobs = parse_convert_args(inputs, to, output.clone())?;

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

    let failures = run_multi_convert_job(jobs, threads);
    Ok(if failures > 0 { 1 } else { 0 })
}

pub fn run_multi_convert_job(jobs: Vec<ConvertJob>, threads: Option<usize>) -> usize {
    let pool = ThreadPoolBuilder::new()
        .num_threads(threads.unwrap_or_else(num_cpus))
        .build()
        .expect("build thread pool");

    pool.install(|| {
        jobs.into_par_iter()
            .map(|job| match run_convert_job(job) {
                Ok(0) => 0,
                Ok(_) | Err(_) => 1,
            })
            .sum::<usize>()
    })
}

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}
