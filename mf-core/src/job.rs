use crate::format::FormatType;
use std::path::PathBuf;
pub struct ConvertJob {
    pub input: PathBuf,
    pub output: Option<PathBuf>,
    pub format_type: FormatType,
}
