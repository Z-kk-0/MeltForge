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

/// Validate a ConvertJob's input and optional output settings.
///
/// Performs these checks:
/// - the input path exists and is a file and can be opened for reading;
/// - the input file's format is detected from its extension and is supported;
/// - the requested output format is compatible with the detected input format;
/// - if an output path is provided, its parent directory exists and the location is writable (and the output file does not already exist).
///
/// Returns `Ok(())` when all validations pass, or `Err(MeltforgeError)` describing the first validation failure.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// use crate::{ConvertJob, FormatType, validate_job};
///
/// let cj = ConvertJob {
///     input: PathBuf::from("tests/fixtures/input.png"),
///     format_type: FormatType::PNG,
///     output: None,
/// };
///
/// // Run validations; result will be `Ok(())` if the example file is present and accessible.
/// let _ = validate_job(&cj);
/// ```
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

/// Checks that a filesystem path exists and refers to a regular file.
///
/// # Examples
///
/// ```
/// use std::fs::File;
/// use std::env;
/// let mut p = env::temp_dir();
/// p.push("meltforge_validate_path_test");
/// let _ = File::create(&p).unwrap();
/// assert!(validate_path(&p).is_ok());
/// let _ = std::fs::remove_file(&p);
/// ```
///
/// # Returns
///
/// `Ok(())` if the path exists and is a file, `Err(InputError::MissingInputFile(_))` otherwise.
fn validate_path(path: &Path) -> Result<(), InputError> {
    if !path.exists() || !path.is_file() {
        return Err(InputError::MissingInputFile(path.to_path_buf()));
    }
    Ok(())
}

/// Determine the input file format from a file path's extension.
///
/// Examines the path's extension case-insensitively and maps recognized extensions to a `FormatType`.
/// Supported extensions: `"png"` → `FormatType::PNG`, `"jpg"`/`"jpeg"` → `FormatType::JPEG`.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// let fmt = crate::validate::validate_input_format(Path::new("image.png")).unwrap();
/// assert_eq!(fmt, crate::FormatType::PNG);
/// ```
///
/// # Returns
///
/// `Ok(FormatType)` with the detected format, or `Err(FormatError)` if the extension is missing or unsupported.
fn validate_input_format(path: &Path) -> Result<FormatType, FormatError> {
    detect_input_format(path)
}

/// Determines the input file format from a file path's extension.
///
/// Returns `FormatType::PNG` for `.png`, `FormatType::JPEG` for `.jpg` or `.jpeg`,
/// or a `FormatError::UnsupportedInput` when the path has no extension or the extension is unsupported.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use mf_core::validate::detect_input_format;
/// use mf_core::FormatType;
///
/// assert_eq!(detect_input_format(Path::new("image.png")).unwrap(), FormatType::PNG);
/// assert_eq!(detect_input_format(Path::new("photo.JPG")).unwrap(), FormatType::JPEG);
/// assert!(detect_input_format(Path::new("file")).is_err()); // no extension
/// ```
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

/// Checks whether conversion between two image formats is supported.
///
/// Supported conversions: PNG ↔ JPEG.
///
/// # Returns
///
/// `Ok(())` if the input→output conversion is supported, `Err(FormatError::UnsupportedOutput(_))` otherwise.
///
/// # Examples
///
/// ```
/// use mf_core::formats::FormatType;
/// use mf_core::validate::validate_compatibility;
///
/// assert!(validate_compatibility(FormatType::PNG, FormatType::JPEG).is_ok());
/// assert!(validate_compatibility(FormatType::JPEG, FormatType::PNG).is_ok());
/// assert!(validate_compatibility(FormatType::PNG, FormatType::PNG).is_err());
/// ```
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

/// Validates that the provided output path is suitable for creating a new file.
///
/// Checks that the path does not already exist, that its parent directory exists, and that a file can be created in that directory.
///
/// # Errors
/// Returns `IoError::WriteError` if the output path already exists or its parent directory does not exist, and `IoError::PermissionDenied` if creating a temporary test file in the target directory fails.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// // Example: `/tmp/out.png` does not exist and `/tmp` is writable
/// let res = validate_output_dir(Path::new("/tmp/out.png"));
/// assert!(res.is_ok());
/// ```
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

/// Verifies that the given path can be opened for reading.
///
/// # Errors
///
/// Returns `IoError::PermissionDenied(path)` if opening fails due to permission denial; returns
/// `IoError::ReadError(path)` for any other I/O failure when attempting to open the file.
///
/// # Examples
///
/// ```
/// use std::fs::File;
/// use std::path::PathBuf;
///
/// let p: PathBuf = std::env::temp_dir().join("meltforge_readable_test");
/// let _ = File::create(&p).unwrap();
/// assert!(ensure_readable(&p).is_ok());
/// std::fs::remove_file(&p).unwrap();
/// ```
fn ensure_readable(path: &Path) -> Result<(), IoError> {
    match File::open(path) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
            ErrorKind::PermissionDenied => Err(IoError::PermissionDenied(path.to_path_buf())),
            _ => Err(IoError::ReadError(path.to_path_buf())),
        },
    }
}