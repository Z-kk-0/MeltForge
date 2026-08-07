use std::{
    io,
    path::{Path, PathBuf},
};
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

    #[error(transparent)]
    ManifestScanError(#[from] ManifestScanError),

    #[error(transparent)]
    Settings(#[from] SettingsError),
}

impl MeltforgeError {
    pub fn exit_code(&self) -> u8 {
        match self {
            MeltforgeError::Input(_) => 2,
            MeltforgeError::Format(_) => 3,
            MeltforgeError::Conversion(_) => 4,
            MeltforgeError::Io(_) => 5,
            MeltforgeError::ManifestScanError(_) => 6,
            MeltforgeError::Settings(_) => 7,
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
    InvalidArgument(String),
}

#[derive(Debug, Error)]
pub enum FormatError {
    #[error("unsupported input format {0}")]
    UnsupportedInput(String),

    #[error("unsupported output format {0}")]
    UnsupportedOutput(String),
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

    #[error("not found: {0}")]
    NotFound(PathBuf),

    #[error("permission denied: {0}")]
    PermissionDenied(PathBuf),

    #[error("output file already exists: {0}")]
    AlreadyExists(PathBuf),

    #[error("parent directory missing: {0}")]
    MissingParent(PathBuf),

    #[error("Invalid Output: {0}")]
    InvalidOutput(String),
}

#[derive(Debug, Error)]
pub enum ManifestScanError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("TOML parse error in {path:?}: {source}")]
    TomlParse {
        path: PathBuf,
        source: toml::de::Error,
    },

    #[error("Manifest validation error in {path:?}: {message}")]
    Invalid { path: PathBuf, message: String },

}
#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("missing settings file: {0}")]
    MissingSettingsFile(PathBuf),

    #[error("settings directory missing: {0}")]
    MissingSettingsDir(PathBuf),

    #[error("permission denied: {0}")]
    PermissionDenied(PathBuf),

    #[error("failed to read settings file: {0}")]
    ReadError(PathBuf),

    #[error("failed to write settings file: {0}")]
    WriteError(PathBuf),

    #[error("failed to parse settings file {path:?}: {source}")]
    TomlParse {
        path: PathBuf,
        source: toml::de::Error,
    },

    #[error("failed to serialize settings: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("unknown setting key: {0}")]
    UnknownKey(String),

    #[error("invalid value for setting {key}: {message}")]
    InvalidValue { key: String, message: String },
}
