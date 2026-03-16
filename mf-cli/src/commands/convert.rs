use mf_core::runner::{results_to_messages, run_jobs};
use mf_core::validation::error::{IoError, MeltforgeError};
use std::path::PathBuf;

use crate::util::messages::print_messages;
use crate::util::parsing::parse_convert_args;

pub fn run_convert(
    inputs: Vec<PathBuf>,
    to: String,
    output: Option<PathBuf>,
    threads: Option<usize>,
) -> Result<i32, MeltforgeError> {
    let jobs = parse_convert_args(inputs, &to, output.clone())?;

    if jobs.len() > 1 {
        if let Some(ref output_dir) = output {
            if !output_dir.exists() {
                std::fs::create_dir_all(output_dir).map_err(|error| {
                    MeltforgeError::Io(IoError::InvalidOutput(format!(
                        "Failed to create directory '{}': {}",
                        output_dir.display(),
                        error
                    )))
                })?;
            } else if !output_dir.is_dir() {
                return Err(MeltforgeError::Io(IoError::InvalidOutput(format!(
                    "Output '{}' must be a directory when converting multiple files",
                    output_dir.display()
                ))));
            }
        }
    }

    let results = run_jobs(jobs, threads)?;
    let messages = results_to_messages(results);
    let has_error = messages.iter().any(|message| message.is_error());
    print_messages(messages);

    if has_error { Ok(1) } else { Ok(0) }
}
