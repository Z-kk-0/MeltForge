use mf_core::job::ConvertJob;
use mf_core::runner::{run_jobs, JobResult};
use mf_core::validation::error::MeltforgeError;

pub enum ConvertMessage {
    Info(String),
    Success(String),
    Warning(String),
    Error(String),
}

pub fn run_convert(
    jobs: Vec<ConvertJob>,
    threads: Option<usize>,
) -> Result<Vec<ConvertMessage>, MeltforgeError> {
    let results = run_jobs(jobs, threads)?;
    Ok(results_to_messages(results))
}

fn results_to_messages(results: Vec<JobResult>) -> Vec<ConvertMessage> {
    let mut messages = Vec::new();
    let total = results.len();
    let mut failures = 0;

    for r in results {
        messages.push(ConvertMessage::Info(format!(
            "Converting: {}",
            r.job.input.display()
        )));
        match r.result {
            Ok(out_path) => {
                messages.push(ConvertMessage::Success(format!(
                    "Saved: {}",
                    out_path.display()
                )));
            }
            Err(e) => {
                messages.push(ConvertMessage::Error(format!(
                    "{}: {}",
                    r.job.input.display(),
                    e
                )));
                failures += 1;
            }
        }
    }

    let successes = total.saturating_sub(failures);
    if failures == 0 {
        messages.push(ConvertMessage::Success(format!(
            "All {} conversion(s) succeeded",
            successes
        )));
    } else {
        messages.push(ConvertMessage::Warning(format!(
            "{} succeeded, {} failed",
            successes, failures
        )));
    }

    messages
}
