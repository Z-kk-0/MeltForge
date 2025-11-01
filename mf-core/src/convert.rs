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

/// Convert an image file between PNG and JPEG and write the result to disk.
///
/// Validates the provided `ConvertJob`, determines the output path (uses `cj.output` if set,
/// otherwise derives a path by changing the input file extension to the target format), ensures
/// the output directory exists (creates parents if necessary), detects the input image format,
/// and performs the conversion for the supported pairs: PNG → JPEG and JPEG → PNG. If the input
/// format and requested output format are not one of the supported pairs, an error is returned.
///
/// # Errors
///
/// Returns a `MeltforgeError` when validation fails, when creating the output directory fails,
/// when input format detection fails, when the conversion or output write fails, or when the
/// requested conversion pair is unsupported.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// // Construct a ConvertJob converting "photo.png" to JPEG, letting the function derive the output path.
/// let cj = ConvertJob { input: PathBuf::from("photo.png"), output: None, format_type: FormatType::JPEG };
/// let out_path = convert(cj).unwrap();
/// assert_eq!(out_path.extension().and_then(|s| s.to_str()), Some("jpg"));
/// ```
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

/// Converts a PNG image at `input` to JPEG format and writes it to `output`.
///
/// # Returns
///
/// `Ok(())` on success, `Err(MeltforgeError)` if opening, conversion, or writing fails.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// // Converts "image.png" to "image.jpg"
/// convert_png_jpg(Path::new("image.png"), Path::new("image.jpg")).unwrap();
/// ```
fn convert_png_jpg(input: &Path, output: &Path) -> Result<(), MeltforgeError> {
    let img = image::open(input)
        .map_err(|e| ConversionError::ExecutionFailed(format!("open {}: {e}", input.display())))?;

    img.save_with_format(output, ImageFormat::Jpeg)
        .map_err(|e| {
            ConversionError::OutputWriteFailed(format!("save {}: {e}", output.display()))
        })?;

    Ok(())
}

/// Converts a JPEG image file to PNG and writes the result to `output`.
///
/// Attempts to open `input` as an image and save it in PNG format to `output`.
/// Image read or write failures are returned as a `MeltforgeError` mapped from conversion errors.
///
/// # Returns
///
/// `Ok(())` on success, otherwise a `MeltforgeError` describing the failure.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// // Converts "photo.jpg" to "photo.png"
/// let _ = convert_jpg_png(Path::new("photo.jpg"), Path::new("photo.png"));
/// ```
fn convert_jpg_png(input: &Path, output: &Path) -> Result<(), MeltforgeError> {
    let img = image::open(input)
        .map_err(|e| ConversionError::ExecutionFailed(format!("open {}: {e}", input.display())))?;

    img.save_with_format(output, ImageFormat::Png)
        .map_err(|e| {
            ConversionError::OutputWriteFailed(format!("save {}: {e}", output.display()))
        })?;

    Ok(())
}

/// Create an output path by changing the input file's extension to match the target image format.
///
/// The returned path is a copy of `input` with its file extension replaced according to `to`:
/// `FormatType::JPEG` -> `"jpg"`, `FormatType::PNG` -> `"png"`.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// let out = derive_output_path(Path::new("images/photo.png"), FormatType::JPEG);
/// assert_eq!(out.to_str().unwrap(), "images/photo.jpg");
/// ```
fn derive_output_path(input: &Path, to: FormatType) -> PathBuf {
    let mut p = input.to_path_buf();
    let ext = match to {
        FormatType::JPEG => "jpg",
        FormatType::PNG => "png",
    };
    p.set_extension(ext);
    p
}

/// Maps an I/O error that occurred while writing to a filesystem path into a `MeltforgeError`.
///
/// The returned error distinguishes permission-denied failures from other write failures by
/// wrapping the provided path in the appropriate `IoError` variant.
///
/// # Examples
///
/// ```
/// use std::io;
/// use std::path::PathBuf;
///
/// let err = io::Error::from(io::ErrorKind::PermissionDenied);
/// let path = PathBuf::from("output/result.jpg");
/// let _ = map_io_write(err, path);
/// ```
fn map_io_write(e: io::Error, p: PathBuf) -> MeltforgeError {
    match e.kind() {
        io::ErrorKind::PermissionDenied => IoError::PermissionDenied(p).into(),
        _ => IoError::WriteError(p).into(),
    }
}