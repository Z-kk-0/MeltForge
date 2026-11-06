use thiserror::Error;

use super::{load_error::LoadError, lua_error::LuaError, output_error::OutputError, resolve_error::ResolveError};

#[derive(Debug, Error)]
pub enum PluginError {
    #[error(transparent)]
    Load(#[from] LoadError),

    #[error(transparent)]
    Resolve(#[from] ResolveError),

    #[error(transparent)]
    Lua(#[from] LuaError),

    #[error(transparent)]
    Output(#[from] OutputError),
}
