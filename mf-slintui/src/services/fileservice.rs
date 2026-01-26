use std::path::{Path, PathBuf};
use mf_core::validation::error::{IoError, MeltforgeError};

pub fn get_file_path(path: &Path) -> Result<PathBuf, MeltforgeError> {
    if !path.exists() {
        return Err(MeltforgeError::Io(IoError::NotFound(path.to_path_buf())))
    }
    Ok(path.to_path_buf())
}
