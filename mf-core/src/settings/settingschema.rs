use std::path::PathBuf;

use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub enum SettingValue {
    Boolean(bool),
    Enum { options: Vec<String>, selected: String },
    NumericRange { max_number: f64, min_number: f64, steps: f64, selected_number: f64 },
    Path(Option<PathBuf>),
    KeyValueList(Vec<(String, SettingValue)>),
    SubSettings { condition: String, settings: Vec<SettingSchema> },
}

#[derive(Deserialize, Serialize)]
pub struct SettingSchema {
    pub key: String,
    pub value: SettingValue
}
 #[derive(Serialize, Deserialize)]
pub struct SettingsFile {
    pub settings: Vec<SettingSchema>,
}