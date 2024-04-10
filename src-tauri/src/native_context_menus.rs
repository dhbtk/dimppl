use serde::{Deserialize, Serialize};
use tauri::menu::{Menu, MenuBuilder};
use tauri::{AppHandle, Wry};

use crate::errors::AppResult;

#[derive(Serialize, Deserialize)]
pub enum ContextMenuOption {
    PodcastEpisode { id: i32 },
}

impl ContextMenuOption {
    pub fn show_context_menu(&self, app: AppHandle) -> AppResult<Menu<Wry>> {
        let menu = match self {
            ContextMenuOption::PodcastEpisode { id: _ } => MenuBuilder::new(&app)
                .text("opt1", "Opção 1")
                .text("opt2", "Opção 2")
                .build()?,
        };
        Ok(menu)
    }
}
