use mf_core::job::ConvertJob;
use mf_core::message::Message;
use mf_core::runner::{results_to_messages, run_jobs};
use mf_core::validation::error::MeltforgeError;

pub fn run_convert(
    jobs: Vec<ConvertJob>,
    threads: Option<usize>,
) -> Result<Vec<Message>, MeltforgeError> {
    let results = run_jobs(jobs, threads)?;
    Ok(results_to_messages(results))
}
