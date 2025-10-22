use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MeltforgeError {
    Input = 0,
    Format = 1,
    Conversion = 2,
    Io = 3
}

#[derive(Debug, Error)]
pub enum InputError {
    #[error("Missing input file: {0}")]
    MissingInputFile(PathBuf),
    #[error("Missing target format (--to)")]
    MissingTargetFormat,
    #[error("Invalid argument: {0} ")]
    InvalidArgument(String)
}       

#[derive(Debug, Error)]
pub enum FormatError {
    #[error("Unsupported Input type: {0}")]
    UnsupportedInput(String)
    #[error("Unsupported Output type: {0}")]
    UnsupportedOutput(String)
}