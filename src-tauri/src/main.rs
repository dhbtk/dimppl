// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

use std::path::PathBuf;
use std::sync::Arc;
use tauri::http::Response;

use crate::config::ConfigWrapper;
use crate::database::{database_path, db_connect, prepare_database};
use crate::directories::ensure_data_dir;
use crate::models::episode_downloads::EpisodeDownloads;
use crate::models::podcast;
use crate::player::Player;
use tauri::Manager;
use tracing::Level;

mod backend;
mod commands;
mod config;
mod database;
mod directories;
mod environment;
mod errors;
mod extensions;
mod frontend_change_tracking;
mod models;
mod player;
mod schema;

#[tokio::main]
#[allow(deprecated)]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_target(false)
        .pretty()
        .init();

    tauri::async_runtime::set(tokio::runtime::Handle::current());
    ensure_data_dir();
    prepare_database();
    let db_url = database_path();
    tracing::info!("db url: {db_url}");
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_window::init())
        .manage(ConfigWrapper::default())
        .register_uri_scheme_protocol("localimages", move |_app, request| {
            let mut conn = db_connect();
            let uri_str = request.uri().to_string();
            if uri_str.starts_with("localimages://podcast/") {
                let podcast_id: i32 = uri_str
                    .strip_prefix("localimages://podcast/")
                    .unwrap()
                    .parse()
                    .unwrap();
                let podcast = podcast::find_one(podcast_id, &mut conn).unwrap();
                let path = PathBuf::from(podcast.local_image_path);
                if path.exists() {
                    return Response::builder()
                        .status(200)
                        .body(fs::read(path).unwrap())
                        .unwrap();
                }
            }
            Response::builder().status(404).body(Vec::new()).unwrap()
        })
        .setup(|app| {
            app.manage(EpisodeDownloads::new(app.handle().clone()));
            let player = Arc::new(Player::new(app.handle().clone()));
            let config_wrapper = app.state::<ConfigWrapper>();
            let config = config_wrapper.0.lock().unwrap();
            player.set_volume(config.volume);
            player.set_playback_speed(config.playback_speed);
            app.manage(player);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_all_podcasts,
            commands::sync_podcasts,
            commands::get_config,
            commands::set_config,
            commands::register_user,
            commands::set_access_key,
            commands::register_device,
            commands::import_podcast,
            commands::list_podcast_episodes,
            commands::download_episode,
            commands::get_episode,
            commands::play_episode,
            commands::player_action,
            commands::find_progress_for_episode,
            commands::set_volume,
            commands::seek
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
