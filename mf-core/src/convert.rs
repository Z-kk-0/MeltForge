use std::{fs::File, path::PathBuf};

use crate::{error::MeltforgeError, job::ConvertJob, validate::validate_job};

pub fn convert(cj: ConvertJob) -> Result<PathBuf, MeltforgeError> {
    validate_job(&cj); // Validate
    let out = convert_png_jpg(&cj)?; // Convert currently only png  to jpg will later be replaced with the plugin function
    Ok(out) // Respond
}
