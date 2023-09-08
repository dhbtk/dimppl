// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

use std::path::PathBuf;

use crate::config::ConfigWrapper;
use crate::database::{database_path, db_connect, prepare_database};
use crate::directories::ensure_data_dir;
use crate::models::episode_downloads::EpisodeDownloads;
use crate::models::podcast;
use tauri::http::ResponseBuilder;
use tauri::Manager;

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
mod schema;

fn main() {
    ensure_data_dir();
    prepare_database();
    let db_url = database_path();
    println!("db url: {db_url}");
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_window::init())
        .manage(ConfigWrapper::default())
        .register_uri_scheme_protocol("localimages", move |_app, request| {
            let mut conn = db_connect();
            if request.uri().starts_with("localimages://podcast/") {
                let podcast_id: i32 = request
                    .uri()
                    .strip_prefix("localimages://podcast/")
                    .unwrap()
                    .parse()?;
                let podcast = podcast::find_one(podcast_id, &mut conn)?;
                let path = PathBuf::from(podcast.local_image_path);
                if path.exists() {
                    return ResponseBuilder::new().status(200).body(fs::read(path)?);
                }
            }
            return ResponseBuilder::new().status(404).body(Vec::new());
        })
        .setup(|app| {
            app.manage(EpisodeDownloads::new(app.handle().clone()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_all_podcasts,
            commands::get_config,
            commands::set_config,
            commands::register_user,
            commands::set_access_key,
            commands::register_device,
            commands::import_podcast,
            commands::list_podcast_episodes,
            commands::download_episode,
            commands::get_episode
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
