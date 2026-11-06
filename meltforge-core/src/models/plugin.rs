use std::{path::PathBuf};

use uuid::Uuid;

use crate::models::manifest::Manifest;

pub struct Plugin {
    id: Uuid,
    manifest: Manifest,
    path: PathBuf
}