use crate::backend::endpoints;
use crate::backend::models::CreateDeviceRequest;
use crate::config::{Config, ConfigWrapper};
use crate::database::db_connect;
use crate::errors::AppResult;
use crate::frontend_change_tracking::{AppHandleExt, EntityChange};
use crate::models::episode::EpisodeWithProgress;
use crate::models::episode_downloads::EpisodeDownloads;
use crate::models::{episode, podcast};
use crate::models::{Episode, Podcast};
use std::ops::Deref;
use tauri::AppHandle;

#[tauri::command]
pub async fn list_all_podcasts() -> AppResult<Vec<Podcast>> {
    let mut connection = db_connect();
    podcast::list_all(&mut connection)
}

#[tauri::command]
pub fn get_config(config_wrapper: tauri::State<ConfigWrapper>) -> Config {
    config_wrapper.0.lock().unwrap().clone()
}

#[tauri::command]
pub async fn set_config(
    new_config: Config,
    config_wrapper: tauri::State<'_, ConfigWrapper>,
) -> AppResult<()> {
    config_wrapper.update(new_config)?;
    Ok(())
}

#[tauri::command]
pub async fn register_user(config_wrapper: tauri::State<'_, ConfigWrapper>) -> AppResult<()> {
    let response = endpoints::create_user().await?;
    let mut config: Config = config_wrapper.0.lock().unwrap().clone();
    config.user_access_key = response.access_key;
    config_wrapper.update(config)?;
    Ok(())
}

#[tauri::command]
pub async fn set_access_key(
    value: String,
    config_wrapper: tauri::State<'_, ConfigWrapper>,
) -> AppResult<()> {
    let mut config: Config = config_wrapper.0.lock().unwrap().clone();
    config.user_access_key = value;
    config_wrapper.update(config)?;
    Ok(())
}

#[tauri::command]
pub async fn register_device(
    device_name: String,
    config_wrapper: tauri::State<'_, ConfigWrapper>,
) -> AppResult<()> {
    let mut config: Config = config_wrapper.0.lock().unwrap().clone();
    config.device_name = device_name.clone();
    let request = CreateDeviceRequest {
        user_access_key: config.user_access_key.clone(),
        device_name,
    };
    let response = endpoints::create_device(&request).await?;
    config.access_token = response.access_token;
    config_wrapper.update(config)?;
    Ok(())
}

#[tauri::command]
pub async fn import_podcast(url: String, app: AppHandle) -> AppResult<()> {
    let mut conn = db_connect();
    let podcast = podcast::import_podcast_from_url(url, &mut conn).await?;
    app.send_invalidate_cache(EntityChange::Podcast(podcast.id))?;
    Ok(())
}

#[tauri::command]
pub async fn list_podcast_episodes(id: i32) -> AppResult<Vec<EpisodeWithProgress>> {
    let mut conn = db_connect();
    episode::list_for_podcast(id, &mut conn)
}

#[tauri::command]
pub async fn download_episode(
    id: i32,
    progress_indicator: tauri::State<'_, EpisodeDownloads>,
    app: AppHandle,
) -> AppResult<()> {
    let mut conn = db_connect();
    episode::start_download(id, progress_indicator.deref(), &mut conn).await?;
    app.send_invalidate_cache(EntityChange::Episode(id))?;
    Ok(())
}

#[tauri::command]
pub fn get_episode(id: i32) -> AppResult<Episode> {
    let mut conn = db_connect();
    episode::find_one(id, &mut conn)
}
