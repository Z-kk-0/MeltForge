use std::{fs, path::{Path, PathBuf}};

use serde::Deserialize;

use crate::validation::{error::ManifestScanError, plugin_validation::{RawManifest, validate_plugin}};

#[derive(Debug, Clone)]
pub struct Manifest {
    pub path: PathBuf,
    pub name: String,
    pub version: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

pub fn scan_plugins(root: impl AsRef<Path>) -> Result<Vec<Manifest>, ManifestScanError> {
    let root = root.as_ref();
    if !root.exists() {
        fs::create_dir_all(root)?;
    }

    let mut found = Vec::new();

    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let manifest_path = if path.join("plugin.toml").exists() {
            path.join("plugin.toml")
        } else {
            continue;
        };

        let content = fs::read_to_string(&manifest_path)?;
        let raw: RawManifest =
            toml::from_str(&content).map_err(|e| ManifestScanError::TomlParse {
                path: manifest_path.clone(),
                source: e,
            })?;
        let manifest = validate_plugin(raw, manifest_path)?;
        found.push(manifest);
    }

    Ok(found)
}
