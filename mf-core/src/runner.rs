use std::path::PathBuf;

use rayon::ThreadPoolBuilder;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::convert::convert;
use crate::job::ConvertJob;
use crate::message::Message;
use crate::validation::error::{IoError, MeltforgeError};

pub struct JobResult {
    pub job: ConvertJob,
    pub result: Result<PathBuf, MeltforgeError>,
}

pub fn run_jobs(
    jobs: Vec<ConvertJob>,
    threads: Option<usize>,
) -> Result<Vec<JobResult>, MeltforgeError> {
    let pool = ThreadPoolBuilder::new()
        .num_threads(threads.unwrap_or_else(num_cpus))
        .build()
        .map_err(|error| {
            MeltforgeError::Io(IoError::InvalidOutput(format!(
                "Failed to create thread pool: {}",
                error
            )))
        })?;

    let results = pool.install(|| {
        jobs.into_par_iter()
            .map(|job| {
                let result = convert(job.clone());
                JobResult { job, result }
            })
            .collect::<Vec<_>>()
    });

    Ok(results)
}

pub fn results_to_messages(results: Vec<JobResult>) -> Vec<Message> {
    let mut messages = Vec::new();
    let total = results.len();
    let mut failures = 0;

    for job_result in results {
        messages.push(Message::Info(format!(
            "Converting: {}",
            job_result.job.input.display()
        )));
        match job_result.result {
            Ok(output_path) => {
                messages.push(Message::Success(format!(
                    "Saved: {}",
                    output_path.display()
                )));
            }
            Err(error) => {
                messages.push(Message::Error(format!(
                    "{}: {}",
                    job_result.job.input.display(),
                    error
                )));
                failures += 1;
            }
        }
    }

    let successes = total.saturating_sub(failures);
    if failures == 0 {
        messages.push(Message::Success(format!(
            "All {} conversion(s) succeeded",
            successes
        )));
    } else {
        messages.push(Message::Warning(format!(
            "{} succeeded, {} failed",
            successes, failures
        )));
    }

    messages
}

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}
