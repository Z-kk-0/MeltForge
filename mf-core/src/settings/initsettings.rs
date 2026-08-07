use std::{fs, io::ErrorKind, path::PathBuf};
use toml::Table;


use crate::{
    settings::settingschema::{SettingSchema, SettingValue::Path, SettingsFile}, validation::error::{MeltforgeError, SettingsError},
};

pub fn init_settings() -> Result<(), MeltforgeError> {

    // create setting list with default save path
    let settingslist: Vec<SettingSchema> = vec![SettingSchema {
        key: "default_save_path".to_string(),
        value: Path(None),
    }];
    // create settingsfile struct for toml
    let settingsfile = SettingsFile {
        settings: settingslist
    };
    
    let toml_string = toml::to_string(&settingsfile).map_err(SettingsError::from)?;

    let path = PathBuf::from("settings.toml");
    fs::write(&path, toml_string).map_err(|error| map_io_write(error, path))?;

    Ok(())
}

fn map_io_write(io_error: std::io::Error, path: PathBuf) -> MeltforgeError {
    match io_error.kind() {
        ErrorKind::NotFound => SettingsError::MissingSettingsDir(path).into(),
        ErrorKind::PermissionDenied => SettingsError::PermissionDenied(path).into(),
        _ => SettingsError::WriteError(path).into(),
    }
}

pub fn map_io_read(io_error: std::io::Error, path: PathBuf) -> MeltforgeError {
    match io_error.kind() {
        ErrorKind::NotFound => SettingsError::MissingSettingsFile(path).into(),
        ErrorKind::PermissionDenied => SettingsError::PermissionDenied(path).into(),
        _ => SettingsError::ReadError(path).into(),
    }
}
