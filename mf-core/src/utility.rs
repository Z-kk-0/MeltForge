use std::{fs, io, path::{Path, PathBuf}};

pub fn ensure_and_scan_dir<P: AsRef<Path>>(dir: P) -> io::Result<Vec<PathBuf>> {
    let dir = dir.as_ref();

    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }

    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
    let entry = entry?;
    let path = entry.path();
    if path.is_file() {
        files.push(path);
    }
}
    Ok(files)
} 