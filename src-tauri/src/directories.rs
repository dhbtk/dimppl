use directories::ProjectDirs;
use std::fs;

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
