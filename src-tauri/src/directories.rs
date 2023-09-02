use directories::ProjectDirs;
use std::fs;
use std::path::{Path, PathBuf};

pub fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("com", "dhbtk", "Dimppl").expect("Could not locate user home folder")
}

pub fn ensure_data_dir() {
    let directories = project_dirs();
    let dir = directories.data_dir();
    if !dir.exists() {
        fs::create_dir_all(dir).expect("Could not create data folder");
    }
}

pub fn images_dir() -> PathBuf {
    let path = project_dirs().data_dir().join("downloadedImages");
    if !path.exists() {
        fs::create_dir_all(&path).expect("could not create images dir");
    }
    path
}
