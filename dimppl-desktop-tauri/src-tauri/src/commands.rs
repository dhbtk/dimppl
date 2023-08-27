use crate::config::{Config, ConfigWrapper};
use crate::database::db_connect;
use crate::errors::AppResult;
use crate::models::Podcast;
use crate::models::podcast;
use crate::state::AppState;

#[tauri::command]
pub async fn list_all_podcasts() -> AppResult<Vec<Podcast>> {
    let mut connection = db_connect();
    podcast::list_all(&mut connection)
}

#[tauri::command]
pub fn get_config(
    config_wrapper: tauri::State<ConfigWrapper>
) -> Config {
    println!("get_config called");
    config_wrapper.0.lock().unwrap().clone()
}

#[tauri::command]
pub async fn set_config(
    new_config: Config,
    config_wrapper: tauri::State<'_, ConfigWrapper>
) -> AppResult<()> {
    new_config.save()?;
    *config_wrapper.0.lock().unwrap() = new_config.clone();
    Ok(())
}
