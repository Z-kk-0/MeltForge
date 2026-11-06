use thiserror::Error;



#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("plugin '{plugin}' declares no inputs or outputs")]
    EmptyInputsOrOutputs { plugin: String },

    #[error("plugin '{plugin}' excludes {input} -> {output}, which it doesn't declare")]
    InvalidExclude { plugin: String, input: String, output: String },
}
