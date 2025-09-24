use std::fs::File;

use crate::job::ConvertJob;

pub fn convert(cj: ConvertJob) -> Result<PathBuf, MeltforgeError> {
    validate_job(&cj)?; // Validate
    let out = convert_png_jpg(&cj)?; // Convert currently only png  to jpg will later be replaced with the plugin function
    Ok(out) // Respond
}
