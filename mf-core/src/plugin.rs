use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::Deserialize;

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

pub fn scan_plugins(root: impl AsRef<Path>) -> Result<Vec<Manifest>, ManifestScanError> {
    let root = root.as_ref();
    if !root.exists() {
        fs::create_dir_all(root)?;
    }

    let mut found = Vec::new();
    let mut errors = Vec::new();

    for entry in fs::read_dir(root)? {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                eprintln!("Warning: Failed to read directory entry: {err}");
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
            Ok (manifest) => found.push(manifest),
            Err(err) => {
                eprintln!("Warning: Failed to load plugin at {}: {err}", manifest_path.display());
                errors.push(err);
            }
        }
    }

    if !errors.is_empty() {
        eprintln!("Loaded {} plugins with {} errors", found.len(), errors.len());
    }

    Ok(found)
}

fn load_single_plugin(manifest_path: &Path) -> Result<Manifest, ManifestScanError> {
    let content = fs::read_to_string(&manifest_path)?;
    let mut raw: RawManifest =
            toml::from_str(&content).map_err(|e| ManifestScanError::TomlParse {
                path: manifest_path.to_path_buf(),
                source: e,
            })?;
    validate_plugin(raw, manifest_path.to_path_buf())
}