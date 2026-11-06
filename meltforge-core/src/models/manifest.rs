use std::collections::HashMap;

pub struct Manifest {
    name: String,
    description: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
    excludes: Vec<HashMap<String, String>>,
}