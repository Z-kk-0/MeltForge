use std::path::PathBuf;

use serde::Deserialize;

use crate::{plugin::Manifest, validation::error::ManifestScanError};

#[derive(Deserialize)]
pub struct RawManifest {
    name: String,
    version: String,
    capabilities: RawCapabilites,
}
#[derive(Deserialize)]
pub struct RawCapabilites {
    inputs: Vec<String>,
    outputs: Vec<String>,
}

pub fn validate_plugin(raw: RawManifest, path: PathBuf) -> Result<Manifest, ManifestScanError> {
    if raw.name.trim().is_empty() {
        return Err(ManifestScanError::Invalid {
            path,
            msg: "name must not be empty".into(),
        });
    }
    if raw.version.trim().is_empty() {
        return Err(ManifestScanError::Invalid {
            path,
            msg: "version must not be empty".into(),
        });
    }
    if raw.capabilities.inputs.is_empty() {
        return Err(ManifestScanError::Invalid {
            path,
            msg: "capabilities.inputs must not be empty".into(),
        });
    }
    if raw.capabilities.outputs.is_empty() {
        return Err(ManifestScanError::Invalid {
            path,
            msg: "capabilities.outputs must not be empty".into(),
        });
    }

    Ok(Manifest {
        path,
        name: raw.name,
        version: raw.version,
        inputs: raw.capabilities.inputs,
        outputs: raw.capabilities.outputs,
    })
}
