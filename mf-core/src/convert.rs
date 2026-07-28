use std::{
    fs,
    path::{Path, PathBuf},
};

use image::ImageFormat;

use crate::{
    format::FormatType,
    job::ConvertJob,
    validation::error::{ConversionError, FormatError, IoError, MeltforgeError},
    validation::job_validation::{detect_input_format, validate_job},
};

pub fn convert(job: ConvertJob) -> Result<PathBuf, MeltforgeError> {
    validate_job(&job)?;

    let output_path = job
        .output
        .clone()
        .unwrap_or_else(|| derive_output_path(&job.input, job.format_type));

    let parent = match output_path.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => Some(parent),
        _ => None,
    };
    if let Some(parent) = parent {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|io_error| map_io_write(io_error, parent.to_path_buf()))?;
        }
    }

    let input_format = detect_input_format(&job.input).map_err(MeltforgeError::from)?;
    match (input_format, job.format_type) {
        (FormatType::PNG, FormatType::JPEG) => convert_png_jpg(&job.input, &output_path)?,
        (FormatType::JPEG, FormatType::PNG) => convert_jpg_png(&job.input, &output_path)?,
        _ => {
            return Err(FormatError::UnsupportedOutput(format!(
                "{:?} → {:?} not supported yet",
                input_format, job.format_type
            ))
            .into());
        }
    } // Convert currently only png to jpg, will later be replaced with the plugin function

    Ok(output_path)
}

fn convert_png_jpg(input: &Path, output: &Path) -> Result<(), MeltforgeError> {
    let image = image::open(input).map_err(|error| {
        ConversionError::ExecutionFailed(format!("open {}: {error}", input.display()))
    })?;

    image
        .save_with_format(output, ImageFormat::Jpeg)
        .map_err(|error| {
            ConversionError::OutputWriteFailed(format!("save {}: {error}", output.display()))
        })?;

    Ok(())
}

fn convert_jpg_png(input: &Path, output: &Path) -> Result<(), MeltforgeError> {
    let image = image::open(input).map_err(|error| {
        ConversionError::ExecutionFailed(format!("open {}: {error}", input.display()))
    })?;

    image
        .save_with_format(output, ImageFormat::Png)
        .map_err(|error| {
            ConversionError::OutputWriteFailed(format!("save {}: {error}", output.display()))
        })?;

    Ok(())
}

fn derive_output_path(input: &Path, to: FormatType) -> PathBuf {
    let mut path = input.to_path_buf();
    let extension = match to {
        FormatType::JPEG => "jpg",
        FormatType::PNG => "png",
    };
    path.set_extension(extension);
    path
}

fn map_io_write(io_error: std::io::Error, path: PathBuf) -> MeltforgeError {
    match io_error.kind() {
        std::io::ErrorKind::AlreadyExists => IoError::AlreadyExists(path).into(),
        std::io::ErrorKind::NotFound => IoError::MissingParent(path).into(),
        std::io::ErrorKind::PermissionDenied => IoError::PermissionDenied(path).into(),
        _ => IoError::WriteError(path).into(),
    }
}
