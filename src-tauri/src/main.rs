// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::config::ConfigWrapper;
use crate::database::{database_url, prepare_database};
use crate::directories::ensure_data_dir;

mod commands;
mod config;
mod database;
mod directories;
mod errors;
mod models;
mod schema;
mod state;
mod backend;
mod environment;

fn main() {
    ensure_data_dir();
    prepare_database();
    let db_url = database_url();
    println!("db url: {db_url}");
    tauri::Builder::default()
        .manage(ConfigWrapper::default())
        .invoke_handler(tauri::generate_handler![
            commands::list_all_podcasts,
            commands::get_config,
            commands::set_config,
            commands::register_user,
            commands::set_access_key,
            commands::register_device
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
