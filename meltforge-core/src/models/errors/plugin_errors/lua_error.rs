use thiserror::Error;

#[derive(Debug, Error)]
pub enum LuaError {
    #[error("failed to load plugin.lua of '{plugin}'")]
    ScriptLoad { plugin: String, #[source] source: mlua::Error },

    #[error("plugin '{plugin}' declares input '{function}' but has no {function}() function")]
    MissingInputFunction { plugin: String, function: String },

    #[error("plugin '{plugin}' has no handling for {input} -> {output}")]
    UnsupportedTarget { plugin: String, input: String, output: String },

    #[error("plugin '{plugin}' failed during conversion")]
    Runtime { plugin: String, #[source] source: mlua::Error },
}