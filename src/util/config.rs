use std::path::{Path, PathBuf};

use once_cell::sync::OnceCell;

static SCENE_ROOT_DIR: OnceCell<PathBuf> = OnceCell::new();

pub fn set_scene_root_dir(scene_file: &impl AsRef<Path>)
{ SCENE_ROOT_DIR.set(scene_file.as_ref().parent().unwrap().to_path_buf()).unwrap(); }

pub fn relative_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let mut abs_path = SCENE_ROOT_DIR.get().unwrap().clone();
    abs_path.push(path); abs_path
}
