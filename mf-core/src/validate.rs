use std::{
    fs::{self, File},
    io::ErrorKind,
    path::Path,
};

use crate::{
    error::{FormatError, InputError, IoError, MeltforgeError},
    format::FormatType,
    job::ConvertJob,
};

pub fn validate_job(cj: &ConvertJob) -> Result<(), MeltforgeError> {
    // check if paths are readable and
    validate_path(&cj.input)?;
    ensure_readable(&cj.input)?;

    // validate input format and compatibility(will be excluded when plugins are available)
    let input_fmt = validate_input_format(&cj.input)?;
    validate_compatibility(input_fmt, cj.format_type)?;

    // check output if set
    if let Some(out) = &cj.output {
        validate_output_dir(out)?;
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
    // Magic Spell i dont even really know what it does
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase())
        .ok_or_else(|| FormatError::UnsupportedInput("<no extension>".into()))?;

    let fmt = match ext.as_str() {
        "png" => FormatType::PNG,
        "jpg" | "jpeg" => FormatType::JPEG,
        _ => return Err(FormatError::UnsupportedInput(ext)),
    };

    Ok(fmt)
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
        return Err(IoError::WriteError(output_path.to_path_buf()));
    }

    let dir = output_path.parent().unwrap_or(Path::new("."));

    if !dir.exists() {
        return Err(IoError::WriteError(output_path.to_path_buf()));
    }

    let test_path = dir.join(".meltforge_write_test");

    match fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(&test_path)
    {
        Ok(_) => {
            let _ = fs::remove_file(&test_path);
            Ok(())
        }
        Err(_) => Err(IoError::PermissionDenied(dir.to_path_buf())),
    }
}

fn ensure_readable(path: &Path) -> Result<(), IoError> {
    match File::open(path) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
            ErrorKind::PermissionDenied => Err(IoError::PermissionDenied(path.to_path_buf())),
            _ => Err(IoError::ReadError(path.to_path_buf())),
        },
    }
}
