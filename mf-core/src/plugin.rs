use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::message::Message;
use crate::validation::{
    error::ManifestScanError,
    plugin_validation::{RawManifest, validate_plugin},
};

#[derive(Debug, Clone)]
pub struct Manifest {
    pub path: PathBuf,
    pub name: String,
    pub version: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

pub fn scan_plugins(
    root: impl AsRef<Path>,
) -> Result<(Vec<Manifest>, Vec<Message>), ManifestScanError> {
    let root = root.as_ref();
    if !root.exists() {
        fs::create_dir_all(root)?;
    }

    let mut found = Vec::new();
    let mut messages = Vec::new();
    let mut errors = Vec::new();

    for entry in fs::read_dir(root)? {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                messages.push(Message::Warning(format!(
                    "Failed to read directory entry: {error}"
                )));
                continue;
            }
        };
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let manifest_path = path.join("plugin.toml");
        if !manifest_path.exists() {
            continue;
        }

        match load_single_plugin(&manifest_path) {
            Ok(manifest) => {
                messages.push(Message::Info(format!(
                    "Loaded plugin: {} v{} at {}",
                    manifest.name,
                    manifest.version,
                    manifest.path.display()
                )));
                found.push(manifest);
            }
            Err(error) => {
                messages.push(Message::Warning(format!(
                    "Failed to load plugin at {}: {error}",
                    manifest_path.display()
                )));
                errors.push(error);
            }
        }
    }

    if errors.is_empty() {
        messages.push(Message::Success(format!("{} plugin(s) loaded", found.len())));
    } else {
        messages.push(Message::Warning(format!(
            "{} plugin(s) loaded, {} failed",
            found.len(),
            errors.len()
        )));
    }

    Ok((found, messages))
}

fn load_single_plugin(manifest_path: &Path) -> Result<Manifest, ManifestScanError> {
    let content = fs::read_to_string(manifest_path)?;
    let raw_manifest: RawManifest =
        toml::from_str(&content).map_err(|error| ManifestScanError::TomlParse {
            path: manifest_path.to_path_buf(),
            source: error,
        })?;
    validate_plugin(raw_manifest, manifest_path.to_path_buf())
}
