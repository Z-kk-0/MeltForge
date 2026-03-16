use std::{
    fs,
    io::ErrorKind,
    path::Path,
};

use crate::{
    format::FormatType,
    job::ConvertJob,
    validation::error::{FormatError, InputError, IoError, MeltforgeError},
};

pub fn validate_job(job: &ConvertJob) -> Result<(), MeltforgeError> {
    validate_path(&job.input)?;
    ensure_readable(&job.input)?;

    // validate input format and compatibility (will be excluded when plugins are available)
    let input_format = validate_input_format(&job.input)?;
    validate_compatibility(input_format, job.format_type)?;

    if let Some(output) = &job.output {
        validate_output_dir(output)?;
    }

    Ok(())
}

fn validate_path(path: &Path) -> Result<(), InputError> {
    if !path.exists() || !path.is_file() {
        return Err(InputError::MissingInputFile(path.to_path_buf()));
    }
    Ok(())
}

fn validate_input_format(path: &Path) -> Result<FormatType, FormatError> {
    detect_input_format(path)
}

pub fn detect_input_format(path: &Path) -> Result<FormatType, FormatError> {
    let extension = path
        .extension()
        .and_then(|os_str| os_str.to_str())
        .map(|ext_str| ext_str.to_lowercase())
        .ok_or_else(|| FormatError::UnsupportedInput("<no extension>".into()))?;

    let format = match extension.as_str() {
        "png" => FormatType::PNG,
        "jpg" | "jpeg" => FormatType::JPEG,
        _ => return Err(FormatError::UnsupportedInput(extension)),
    };

    Ok(format)
}

pub fn validate_compatibility(input: FormatType, output: FormatType) -> Result<(), FormatError> {
    match (input, output) {
        (FormatType::PNG, FormatType::JPEG) => Ok(()),
        (FormatType::JPEG, FormatType::PNG) => Ok(()),
        _ => Err(FormatError::UnsupportedOutput(format!(
            "{:?} → {:?} not supported yet",
            input, output
        ))),
    }
}

fn validate_output_dir(output_path: &Path) -> Result<(), IoError> {
    if output_path.exists() {
        return Err(IoError::AlreadyExists(output_path.to_path_buf()).into());
    }
    // defaulting to used directory for user-friendly experience
    let parent_dir = output_path.parent().unwrap_or(Path::new("."));

    if !parent_dir.exists() {
        return Err(IoError::MissingParent(output_path.to_path_buf()));
    }

    let test_path = parent_dir.join(".meltforge_write_test");

    match fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(&test_path)
    {
        Ok(_) => {
            let _ = fs::remove_file(&test_path);
            Ok(())
        }
        Err(_) => Err(IoError::PermissionDenied(parent_dir.to_path_buf())),
    }
}

fn ensure_readable(path: &Path) -> Result<(), IoError> {
    match fs::File::open(path) {
        Ok(_) => Ok(()),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => Err(IoError::NotFound(path.to_path_buf())),
            ErrorKind::PermissionDenied => Err(IoError::PermissionDenied(path.to_path_buf())),
            _ => Err(IoError::ReadError(path.to_path_buf())),
        },
    }
}
