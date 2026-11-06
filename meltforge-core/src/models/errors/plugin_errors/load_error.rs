use std::path::PathBuf;
use thiserror::Error;

use super::manifest_error::ManifestError;

#[derive(Debug, Error)]
pub enum LoadError {
    #[error(transparent)]
    Manifest(#[from] ManifestError),
    
    #[error("the plugins directory in {path} was not found")]
    PluginsDirNotFound { path: PathBuf },


    #[error("plugin at {path} has no plugin.lua")]
    ScriptMissing { path: PathBuf },
}