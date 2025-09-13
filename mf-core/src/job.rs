use crate::format::FormatType;
use std::path::PathBuf;
pub struct ConvertJob {
    pub input: String,
    pub output: Option<PathBuf>,
    pub format_type: FormatType,
}
