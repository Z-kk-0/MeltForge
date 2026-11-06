use thiserror::Error;
use super::plugin_errors::plugin_error::PluginError;

#[derive(Debug, Error)]
pub enum MeltForgeError {
    #[error(transparent)]
    Plugin(#[from] PluginError)
}