use thiserror::Error;


#[derive(Debug, Error)]
pub enum ResolveError {
    #[error("no plugin available for converting {input_file_type} to {output_file_type}")]
    NoPluginAvailable { input_file_type: String, output_file_type: String },

    #[error("plugin '{name}' is not installed")]
    PluginNotFound { name: String },

    #[error("plugin '{name}' cannot convert {input} to {output}")]
    PluginCannotConvert { name: String, input: String, output: String },
    
    #[error("the requested plugin with the name {plugin_name} was not found")]
    RequestedPluginNotFound{ plugin_name: String },
}