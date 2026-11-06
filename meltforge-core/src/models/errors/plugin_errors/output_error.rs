use thiserror::Error;

#[derive(Debug, Error)]
pub enum OutputError {
    #[error("plugin '{plugin}' returned {got}, expected {expected}")]
    InvalidReturnType { plugin: String, expected: String, got: String },

    #[error("plugin '{plugin}' returned empty output")]
    EmptyOutput { plugin: String },
}
