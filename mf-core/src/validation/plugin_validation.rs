use std::path::PathBuf;

use serde::Deserialize;

use crate::{plugin::Manifest, validation::error::ManifestScanError};

#[derive(Deserialize)]
pub struct RawManifest {
    name: String,
    version: String,
    capabilities: RawCapabilities,
}
#[derive(Deserialize)]
pub struct RawCapabilities {
    inputs: Vec<String>,
    outputs: Vec<String>,
}

pub fn validate_plugin(
    raw_manifest: RawManifest,
    path: PathBuf,
) -> Result<Manifest, ManifestScanError> {
    if raw_manifest.name.trim().is_empty() {
        return Err(ManifestScanError::Invalid {
            path,
            message: "name must not be empty".into(),
        });
    }
    if raw_manifest.version.trim().is_empty() {
        return Err(ManifestScanError::Invalid {
            path,
            message: "version must not be empty".into(),
        });
    }
    if raw_manifest.capabilities.inputs.is_empty() {
        return Err(ManifestScanError::Invalid {
            path,
            message: "capabilities.inputs must not be empty".into(),
        });
    }
    if raw_manifest.capabilities.outputs.is_empty() {
        return Err(ManifestScanError::Invalid {
            path,
            message: "capabilities.outputs must not be empty".into(),
        });
    }

    Ok(Manifest {
        path,
        name: raw_manifest.name,
        version: raw_manifest.version,
        inputs: raw_manifest.capabilities.inputs,
        outputs: raw_manifest.capabilities.outputs,
    })
}
