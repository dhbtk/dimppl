use crate::backend::endpoints;
use crate::backend::endpoints::sync_remote_podcasts;
use crate::backend::models::CreateDeviceRequest;
use crate::config::{Config, ConfigWrapper};
use crate::context_menus::ContextMenuType;
use crate::database::db_connect;
use crate::errors::AppResult;
use crate::frontend_change_tracking::{AppHandleExt, EntityChange};
use crate::models::episode::{EpisodeWithFileSize, EpisodeWithPodcast, EpisodeWithProgress};
use crate::models::episode_downloads::EpisodeDownloads;
use crate::models::podcast::{build_backend_sync_request, store_backend_sync_response};
use crate::models::{episode, podcast, EpisodeProgress};
use crate::models::{Episode, Podcast};
use crate::player::Player;
use crate::show_file_in_folder::show_file_in_folder;
use std::ops::Deref;
use std::sync::Arc;
use tauri::{AppHandle, Manager, Window};
use uuid::Uuid;

#[tauri::command]
pub async fn list_all_podcasts() -> AppResult<Vec<Podcast>> {
    let mut connection = db_connect();
    podcast::list_all(&mut connection)
}

#[tauri::command]
pub async fn sync_podcasts(app: AppHandle, config_wrapper: tauri::State<'_, ConfigWrapper>) -> AppResult<()> {
    let config = config_wrapper.0.lock().unwrap().clone();
    let _ = app.emit("sync-podcasts-start", ());
    tokio::spawn(async move {
        let mut connection = db_connect();

        podcast::sync_podcasts(&mut connection, &app).await.unwrap();

        let sync_state_request = build_backend_sync_request(&mut connection).unwrap();
        let backend_sync_result = sync_remote_podcasts(&config.access_token, &sync_state_request)
            .await
            .unwrap();
        store_backend_sync_response(&mut connection, backend_sync_result)
            .await
            .unwrap();

        app.send_invalidate_cache(EntityChange::AllPodcasts).unwrap();
        app.send_invalidate_cache(EntityChange::AllEpisodes).unwrap();
        let podcasts = podcast::list_all(&mut connection).unwrap();
        for podcast in &podcasts {
            let _ = app.emit("sync-podcast-stop", podcast.id);
            app.send_invalidate_cache(EntityChange::Podcast(podcast.id)).unwrap();
            app.send_invalidate_cache(EntityChange::PodcastEpisodes(podcast.id))
                .unwrap();
        }
        let _ = app.emit("sync-podcasts-done", ());
    });

    Ok(())
}

#[tauri::command]
pub fn find_last_played() -> Option<EpisodeWithPodcast> {
    let mut connection = db_connect();
    episode::find_last_played(&mut connection)
}

#[tauri::command]
pub fn list_listen_history() -> AppResult<Vec<EpisodeWithPodcast>> {
    let mut connection = db_connect();
    episode::list_listen_history(&mut connection)
}

#[tauri::command]
pub fn list_latest_episodes() -> AppResult<Vec<EpisodeWithPodcast>> {
    let mut connection = db_connect();
    episode::list_latest_episodes(&mut connection)
}

#[tauri::command]
pub fn get_config(config_wrapper: tauri::State<ConfigWrapper>) -> Config {
    config_wrapper.0.lock().unwrap().clone()
}

#[tauri::command]
pub async fn set_config(new_config: Config, config_wrapper: tauri::State<'_, ConfigWrapper>) -> AppResult<()> {
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
pub async fn set_access_key(value: String, config_wrapper: tauri::State<'_, ConfigWrapper>) -> AppResult<()> {
    let mut config: Config = config_wrapper.0.lock().unwrap().clone();
    config.user_access_key = value;
    config_wrapper.update(config)?;
    Ok(())
}

#[tauri::command]
pub async fn register_device(device_name: String, config_wrapper: tauri::State<'_, ConfigWrapper>) -> AppResult<()> {
    let mut config: Config = config_wrapper.0.lock().unwrap().clone();
    config.device_name.clone_from(&device_name);
    let request = CreateDeviceRequest {
        user_access_key: config.user_access_key.clone(),
        device_name,
    };
    let response = endpoints::create_device(&request).await?;
    config.access_token = response.access_token;
    config_wrapper.update(config)?;
    Ok(())
}

async fn do_import_podcast(url: String, app: AppHandle) -> AppResult<()> {
    let mut conn = db_connect();
    let podcast = podcast::import_podcast_from_url(url, &mut conn).await?;
    app.send_invalidate_cache(EntityChange::Podcast(podcast.id))?;
    Ok(())
}

#[tauri::command]
pub async fn import_podcast(url: String, app: AppHandle) -> AppResult<String> {
    let import_id = Uuid::new_v4().to_string();
    let import_id_clone = import_id.clone();
    tokio::spawn(async move {
        let result = do_import_podcast(url, app.clone()).await;
        match result {
            Ok(_) => {
                let _ = app.emit("import-podcast-done", import_id_clone.clone());
            }
            Err(e) => {
                let _ = app.emit("import-podcast-error", (import_id_clone.clone(), e.to_string()));
            }
        }
    });

    Ok(import_id)
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
    tokio::spawn(do_download_episode(id, progress_indicator.deref().clone(), app));
    Ok(())
}

async fn do_download_episode(id: i32, progress_indicator: EpisodeDownloads, app: AppHandle) -> AppResult<()> {
    let mut conn = db_connect();
    tracing::debug!("start_download");
    episode::start_download(id, &progress_indicator, &mut conn).await?;
    tracing::debug!("start_download finished, now invalidate_cache");
    app.send_invalidate_cache(EntityChange::Episode(id))?;
    app.send_invalidate_cache(EntityChange::AllDownloads)?;
    tracing::debug!("ok");
    Ok(())
}

#[tauri::command]
pub fn get_episode(id: i32) -> AppResult<Episode> {
    let mut conn = db_connect();
    episode::find_one(id, &mut conn)
}

#[tauri::command]
pub fn get_episode_full(id: i32) -> AppResult<EpisodeWithPodcast> {
    let mut conn = db_connect();
    episode::find_one_full(id, &mut conn)
}

#[tauri::command]
pub fn play_episode(id: i32, player: tauri::State<'_, Arc<Player>>) -> AppResult<()> {
    let player = player.deref().clone();
    let mut conn = db_connect();
    let episode = episode::find_one(id, &mut conn)?;
    let progress = episode::find_one_progress(id, &mut conn)?;
    let start_seconds = if progress.completed {
        0
    } else {
        progress.listened_seconds as u64
    };
    std::thread::spawn(move || {
        let _ = player.play_episode(episode, start_seconds);
    });
    Ok(())
}

#[tauri::command]
pub fn player_action(action: String, player: tauri::State<'_, Arc<Player>>) -> AppResult<()> {
    let player = player.deref().clone();
    std::thread::spawn(move || {
        match action.as_str() {
            "play" => player.play(),
            "pause" => player.pause(),
            "skip_forwards" => player.skip_forwards(),
            "skip_backwards" => player.skip_backwards(),
            _ => {}
        };
    });
    Ok(())
}

#[tauri::command]
pub async fn find_progress_for_episode(episode_id: i32) -> AppResult<EpisodeProgress> {
    let mut conn = db_connect();
    episode::find_one_progress(episode_id, &mut conn)
}

#[tauri::command]
pub async fn set_volume(
    volume: f32,
    config_wrapper: tauri::State<'_, ConfigWrapper>,
    player: tauri::State<'_, Arc<Player>>,
) -> AppResult<()> {
    let mut config = config_wrapper.0.lock().unwrap().clone();
    config.volume = volume;
    config_wrapper.update(config)?;
    player.set_volume(volume);
    Ok(())
}

#[tauri::command]
pub async fn seek(to: i64, player: tauri::State<'_, Arc<Player>>) -> AppResult<()> {
    player.seek_to(to);
    Ok(())
}

#[tauri::command]
pub async fn set_up_media_controls(app: AppHandle, player: tauri::State<'_, Arc<Player>>) -> AppResult<()> {
    #[allow(unused)]
    if let Some(window) = app.get_window("main") {
        #[cfg(target_os = "windows")]
        let handle = Some(window.hwnd().unwrap().0 as *mut _);
        #[cfg(not(target_os = "windows"))]
        let handle = None;
        tracing::debug!("setting up media controls");
        player.set_up_media_controls(handle);
    }
    Ok(())
}

#[tauri::command]
pub async fn show_context_menu(
    app: AppHandle,
    window: Window,
    menu_option: ContextMenuType,
    player: tauri::State<'_, Arc<Player>>,
    progress_indicator: tauri::State<'_, EpisodeDownloads>,
) -> AppResult<()> {
    let player = player.deref().clone();
    let menu = menu_option.show_context_menu(app, player.as_ref(), progress_indicator.deref().clone())?;
    window.popup_menu(&menu)?;
    Ok(())
}

#[tauri::command]
pub fn mark_episode_complete(id: i32, app: AppHandle) -> AppResult<()> {
    let mut connection = db_connect();
    let progress_id = episode::mark_as_complete(id, &mut connection)?;
    app.send_invalidate_cache(EntityChange::Episode(id))?;
    app.send_invalidate_cache(EntityChange::EpisodeProgress(progress_id))?;
    Ok(())
}

#[tauri::command]
pub fn mark_episode_not_complete(id: i32, app: AppHandle) -> AppResult<()> {
    let mut connection = db_connect();
    let progress_id = episode::mark_as_not_complete(id, &mut connection)?;
    app.send_invalidate_cache(EntityChange::Episode(id))?;
    app.send_invalidate_cache(EntityChange::EpisodeProgress(progress_id))?;
    Ok(())
}

#[tauri::command]
pub fn show_episode_file_in_folder(id: i32) -> AppResult<()> {
    let mut connection = db_connect();
    let episode = episode::find_one(id, &mut connection)?;
    if !episode.content_local_path.is_empty() {
        show_file_in_folder(&episode.content_local_path)?;
    }
    Ok(())
}

#[tauri::command]
pub fn erase_episode_download(id: i32, app: AppHandle) -> AppResult<()> {
    let mut connection = db_connect();
    episode::erase_downloaded_file(id, &mut connection)?;
    app.send_invalidate_cache(EntityChange::Episode(id))?;
    app.send_invalidate_cache(EntityChange::AllDownloads)?;
    Ok(())
}

#[tauri::command]
pub fn list_all_downloads() -> AppResult<Vec<EpisodeWithFileSize>> {
    let mut connection = db_connect();
    let downloads = episode::find_all_downloaded(&mut connection)?;
    Ok(downloads)
}
