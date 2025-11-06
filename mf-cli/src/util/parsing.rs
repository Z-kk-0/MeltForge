use std::path::PathBuf;

use mf_core::format::FormatType;
use mf_core::job::ConvertJob;
use mf_core::validation::error::{FormatError, MeltforgeError};

pub fn parse_format(to: &str) -> Result<FormatType, MeltforgeError> {
    match to.to_lowercase().as_str() {
        "jpg" | "jpeg" => Ok(FormatType::JPEG),
        "png"          => Ok(FormatType::PNG),
        _ => Err(MeltforgeError::Format(FormatError::UnsupportedOutput(
            to.to_string(),
        ))),
    }
}
pub fn parse_convert_args(
    inputs: Vec<PathBuf>,
    to: String,
    output: Option<PathBuf>,
) -> Result<Vec<ConvertJob>, MeltforgeError> {
    let format_type = crate::util::parsing::parse_format(&to)?;
    let jobs = inputs
        .into_iter()
        .map(|input| ConvertJob {
            input,                 
            output: output.clone(),
            format_type,
        })
        .collect();
    Ok(jobs)
}