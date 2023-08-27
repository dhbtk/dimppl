// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::database::{database_url, prepare_database};
use crate::directories::ensure_data_dir;
use crate::state::AppState;

mod state;
mod database;
mod directories;
mod schema;
mod models;
mod config;
mod errors;

fn main() {
    ensure_data_dir();
    prepare_database();
    let db_url = database_url();
    println!("db url: {db_url}");
    tauri::Builder::default()
        .manage(AppState::new().expect("error initializing app state"))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
