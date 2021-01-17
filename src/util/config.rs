use std::path::{Path, PathBuf};

pub fn relative_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let mut abs_path = crate::CONFIG_DIR.read().unwrap().clone();
    abs_path.push(path); abs_path
}
