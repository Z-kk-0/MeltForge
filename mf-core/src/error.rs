use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MeltforgeError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    Format(#[from] FormatError),

    #[error(transparent)]
    Conversion(#[from] ConversionError),

    #[error(transparent)]
    Io(#[from] IoError),


}

impl MeltforgeError {
    pub fn exit_code(&self) -> u8 {
        match self {
            MeltforgeError::Input(_)      => 2,
            MeltforgeError::Format(_)     => 3, 
            MeltforgeError::Conversion(_) => 4, 
            MeltforgeError::Io(_)         => 5,
        }
    }
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
    #[error("unsupported input format {0}")]
    UnsupportedInput(String),

    #[error("unsupported output format {0}")]
    UnsupportedOutput(String)
}

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("plugin load failed: {0}")]
    PluginLoadFailed(String),

    #[error("execution failed: {0}")]
    ExecutionFailed(String),

    #[error("output write failed: {0}")]
    OutputWriteFailed(String),
}

#[derive(Debug, Error)]
pub enum IoError {
    #[error("read error: {0}")]
    ReadError(PathBuf),

    #[error("write error: {0}")]
    WriteError(PathBuf),

    #[error("permission denied: {0}")]
    PermissionDenied(PathBuf),
}