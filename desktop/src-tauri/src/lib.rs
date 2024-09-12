extern crate core;

use std::fs;

use std::path::PathBuf;
use std::sync::Arc;
use tauri::http::Response;

use crate::config::ConfigWrapper;
#[cfg(not(target_os = "ios"))]
use crate::context_menus::context_menu_event_handler;
use crate::database::{database_path, db_connect, prepare_database};
use crate::directories::ensure_data_dir;
#[cfg(not(target_os = "ios"))]
use crate::main_menu::{build_main_menu, main_menu_event_handler, MainMenuOption};
use crate::models::episode_downloads::EpisodeDownloads;
use crate::models::podcast;
use crate::player::Player;
#[cfg(not(target_os = "ios"))]
use menu_support::ContextMenuOption;
use tauri::Manager;
use tracing::Level;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

mod backend;
mod commands;
mod config;
#[cfg(not(target_os = "ios"))]
mod context_menus;
mod database;
mod directories;
mod environment;
mod errors;
mod extensions;
mod frontend_change_tracking;
#[cfg(not(target_os = "ios"))]
mod main_menu;
mod menu_support;
mod models;
mod navigation;
mod player;
mod schema;
mod show_file_in_folder;

#[allow(deprecated)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("starting run function");
    // let collector = tracing_subscriber::registry()
    //     .with(OsLogger::new("com.dianahorbatiuk.dimppl", "default"));
    // tracing::subscriber::set_global_default(collector).expect("failed to set global subscriber");
    #[cfg(not(target_os = "ios"))]
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .parse_lossy("info,app=debug"),
        )
        .with_target(false)
        .pretty()
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .manage(ConfigWrapper::default())
        .register_uri_scheme_protocol("localimages", move |_app, request| {
            let mut conn = db_connect();
            let uri_str = request.uri().to_string();
            if uri_str.starts_with("localimages://podcast/") {
                let podcast_id: i32 = uri_str.strip_prefix("localimages://podcast/").unwrap().parse().unwrap();
                let podcast = podcast::find_one(podcast_id, &mut conn).unwrap();
                let path = PathBuf::from(podcast.local_image_path);
                if path.exists() {
                    return Response::builder().status(200).body(fs::read(path).unwrap()).unwrap();
                }
            }
            Response::builder().status(404).body(Vec::new()).unwrap()
        })
        .setup(|app| {
            ensure_data_dir();
            prepare_database();
            let db_url = database_path();
            tracing::info!("db url: {db_url}");
            app.manage(EpisodeDownloads::new(app.handle().clone()));
            let player = Arc::new(Player::new(app.handle().clone()));
            let config_wrapper = app.state::<ConfigWrapper>();
            let config = config_wrapper.0.lock().unwrap();
            player.set_volume(config.volume);
            player.set_playback_speed(config.playback_speed);
            app.manage(player);
            #[cfg(not(target_os = "ios"))]
            app.on_menu_event(move |handle, event| {
                let payload = event.id.0;
                if let Ok(context_menu_event) = ContextMenuOption::try_from(payload.clone()) {
                    context_menu_event_handler(handle, context_menu_event);
                } else if let Ok(main_menu_event) = MainMenuOption::try_from(payload.clone()) {
                    main_menu_event_handler(handle, main_menu_event);
                } else {
                    tracing::info!("option not recognized: {payload}");
                }
            });
            #[cfg(not(target_os = "ios"))]
            app.set_menu(build_main_menu(app.handle()).unwrap()).unwrap();
            #[cfg(debug_assertions)]
            {
                #[cfg(not(target_os = "ios"))]
                app.get_webview_window("main").unwrap().open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_all_podcasts,
            commands::sync_podcasts,
            commands::find_last_played,
            commands::list_listen_history,
            commands::list_latest_episodes,
            commands::get_config,
            commands::set_config,
            commands::register_user,
            commands::set_access_key,
            commands::register_device,
            commands::import_podcast,
            commands::list_podcast_episodes,
            commands::download_episode,
            commands::get_episode,
            commands::get_episode_full,
            commands::play_episode,
            commands::player_action,
            commands::find_progress_for_episode,
            commands::set_volume,
            commands::seek,
            commands::set_up_media_controls,
            commands::show_context_menu,
            commands::mark_episode_complete,
            commands::mark_episode_not_complete,
            commands::show_episode_file_in_folder,
            commands::erase_episode_download,
            commands::list_all_downloads
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
