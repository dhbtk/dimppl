use crate::errors::AppResult;
use std::process::Command;

pub fn show_file_in_folder(path: &str) -> AppResult<()> {
    if cfg!(target_os = "macos") {
        tracing::debug!("opening {path}");
        Command::new("open").args(["-R", path]).spawn()?;
    }
    Ok(())
}
