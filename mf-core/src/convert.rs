use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

use image::{ImageError, ImageFormat};

use crate::{
    error::{ConversionError, FormatError, IoError, MeltforgeError},
    format::FormatType,
    job::ConvertJob,
    validate::{detect_input_format, validate_job},
};

pub fn convert(cj: ConvertJob) -> Result<PathBuf, MeltforgeError> {
    validate_job(&cj)?; // Validate

    let mut output_path = cj
        .output
        .clone()
        .unwrap_or_else(|| derive_output_path(&cj.input, cj.format_type));

    if let Some(parent) = output_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| map_io_write(e, parent.to_path_buf()))?;
        }
    }
    let input_fmt = detect_input_format(&cj.input).map_err(|e| MeltforgeError::from(e))?;
    match (input_fmt, cj.format_type) {
        (FormatType::PNG, FormatType::JPEG) => convert_png_jpg(&cj.input, &output_path)?,
        (FormatType::JPEG, FormatType::PNG) => convert_jpg_png(&cj.input, &output_path)?,
        _ => {
            return Err(FormatError::UnsupportedOutput(format!(
                "{:?} → {:?} not supported yet",
                input_fmt, cj.format_type
            ))
            .into());
        }
    } // Convert currently only png  to jpg will later be replaced with the plugin function

    Ok(output_path) // Respond
}

fn convert_png_jpg(input: &Path, output: &Path) -> Result<(), MeltforgeError> {
    let img = image::open(input)
        .map_err(|e| ConversionError::ExecutionFailed(format!("open {}: {e}", input.display())))?;

    img.save_with_format(output, ImageFormat::Jpeg)
        .map_err(|e| {
            ConversionError::OutputWriteFailed(format!("save {}: {e}", output.display()))
        })?;

    Ok(())
}

fn convert_jpg_png(input: &Path, output: &Path) -> Result<(), MeltforgeError> {
    let img = image::open(input)
        .map_err(|e| ConversionError::ExecutionFailed(format!("open {}: {e}", input.display())))?;

    img.save_with_format(output, ImageFormat::Png)
        .map_err(|e| {
            ConversionError::OutputWriteFailed(format!("save {}: {e}", output.display()))
        })?;

    Ok(())
}

fn derive_output_path(input: &Path, to: FormatType) -> PathBuf {
    let mut p = input.to_path_buf();
    let ext = match to {
        FormatType::JPEG => "jpg",
        FormatType::PNG => "png",
    };
    p.set_extension(ext);
    p
}

fn map_io_write(e: io::Error, p: PathBuf) -> MeltforgeError {
    match e.kind() {
        io::ErrorKind::AlreadyExists => IoError::AlreadyExists(p).into(),
        io::ErrorKind::NotFound => IoError::MissingParent(p).into(),
        io::ErrorKind::PermissionDenied => IoError::PermissionDenied(p).into(),
        _ => IoError::WriteFailed(p).into(),
    }
}
