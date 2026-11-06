use std::{collections::HashMap, path::PathBuf};

pub struct Plugin {
    name: String,
    description: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
    excludes: Vec<HashMap<String, String>>,
    version: i32,
    path: PathBuf
}