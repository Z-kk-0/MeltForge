use std::path::PathBuf;

use rayon::ThreadPoolBuilder;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::convert::convert;
use crate::job::ConvertJob;
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
        .map_err(|e| {
            MeltforgeError::Io(IoError::InvalidOutput(format!(
                "Failed to create thread pool: {}",
                e
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

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}
