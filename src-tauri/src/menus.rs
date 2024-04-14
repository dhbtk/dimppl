use crate::commands;
use serde::{Deserialize, Serialize};
use tauri::menu::{IsMenuItem, Menu, MenuBuilder, MenuEvent, MenuItem, MenuItemKind, PredefinedMenuItem};
use tauri::{AppHandle, Manager, Wry};

use crate::database::db_connect;
use crate::errors::AppResult;
use crate::models::episode;
use crate::models::episode_downloads::EpisodeDownloads;
use crate::navigation::{AppRoute, NavigationExt};
use crate::player::Player;

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub enum ContextMenuOption {
    // For episodes:
    PlayEpisode { id: i32 },
    PauseEpisode { id: i32 },
    StartEpisodeDownload { id: i32 },
    RemoveEpisodeDownload { id: i32 },
    MarkAsCompleted { id: i32 },
    MarkAsNotCompleted { id: i32 },
    ShowFileInFolder { id: i32 },
    GoToEpisode { id: i32 },
    GoToPodcast { episode_id: i32 },
    Separator,
}

impl ContextMenuOption {
    pub fn label(&self) -> String {
        let str = match self {
            ContextMenuOption::PlayEpisode { .. } => "Reproduzir episódio",
            ContextMenuOption::PauseEpisode { .. } => "Pausar episódio",
            ContextMenuOption::StartEpisodeDownload { .. } => "Iniciar download",
            ContextMenuOption::RemoveEpisodeDownload { .. } => "Excluir arquivo local",
            ContextMenuOption::MarkAsCompleted { .. } => "Marcar como reproduzido",
            ContextMenuOption::MarkAsNotCompleted { .. } => "Marcar como não reproduzido",
            ContextMenuOption::ShowFileInFolder { .. } => "Mostrar arquivo na pasta",
            ContextMenuOption::GoToEpisode { .. } => "Ver detalhes do episódio",
            ContextMenuOption::GoToPodcast { .. } => "Ver podcast",
            ContextMenuOption::Separator => "",
        };
        str.into()
    }

    pub fn menu_item(&self, app_handle: &AppHandle) -> Option<MenuItemKind<Wry>> {
        if *self == ContextMenuOption::Separator {
            return PredefinedMenuItem::separator(app_handle).ok().map(|i| i.kind());
        }
        MenuItem::with_id(
            app_handle,
            serde_json::to_string(self).unwrap(),
            self.label(),
            true,
            None::<&str>,
        )
        .ok()
        .map(|i| i.kind())
    }

    pub fn make_menu_items(vec: Vec<Self>, app_handle: &AppHandle) -> Vec<MenuItemKind<Wry>> {
        vec.iter().filter_map(|option| option.menu_item(app_handle)).collect()
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub enum ContextMenuType {
    PodcastEpisode { id: i32 },
}

impl ContextMenuType {
    pub fn show_context_menu(
        self,
        app: AppHandle,
        player: &Player,
        downloads: EpisodeDownloads,
    ) -> AppResult<Menu<Wry>> {
        let mut conn = db_connect();
        let menu = match self {
            ContextMenuType::PodcastEpisode { id } => {
                let episode_data = episode::find_one_full(id, &mut conn)?;
                let mut options: Vec<ContextMenuOption> = Vec::new();
                if let Some(status) = player.latest_status() {
                    if !status.is_paused && status.episode.map(|e| e.id) == Some(id) {
                        options.push(ContextMenuOption::PauseEpisode { id });
                    } else {
                        options.push(ContextMenuOption::PlayEpisode { id });
                    }
                } else {
                    options.push(ContextMenuOption::PlayEpisode { id });
                }
                options.push(ContextMenuOption::GoToEpisode { id });
                options.push(ContextMenuOption::GoToPodcast { episode_id: id });
                if episode_data.progress.completed {
                    options.push(ContextMenuOption::MarkAsNotCompleted { id });
                } else {
                    options.push(ContextMenuOption::MarkAsCompleted { id });
                }
                options.push(ContextMenuOption::Separator);
                if !downloads.in_progress().contains_key(&id) {
                    if episode_data.episode.content_local_path.is_empty() {
                        options.push(ContextMenuOption::StartEpisodeDownload { id });
                    } else {
                        options.push(ContextMenuOption::RemoveEpisodeDownload { id });
                        options.push(ContextMenuOption::ShowFileInFolder { id });
                    }
                }
                let rendered_options = ContextMenuOption::make_menu_items(options, &app);
                let mut builder = MenuBuilder::new(&app);
                for option in &rendered_options {
                    builder = builder.item(option);
                }
                builder.build()?
            }
        };
        Ok(menu)
    }
}

pub fn menu_event_handler(app_handle: &AppHandle, event: MenuEvent) {
    let result = menu_event_handler_inner(app_handle, event);
    if let Err(err) = result {
        tracing::error!("Error handling menu event: {}", err);
    }
}

fn menu_event_handler_inner(app_handle: &AppHandle, event: MenuEvent) -> AppResult<()> {
    tracing::info!("menu item click: {}", event.id.0);
    let maybe_option: ContextMenuOption = serde_json::from_str(&event.id.0)?;
    match maybe_option {
        ContextMenuOption::PlayEpisode { id } => {
            commands::play_episode(id, app_handle.state())?;
        }
        ContextMenuOption::PauseEpisode { .. } => {
            commands::player_action("pause".into(), app_handle.state())?;
        }
        ContextMenuOption::StartEpisodeDownload { id } => {
            let cloned = app_handle.clone();
            tokio::spawn(async move { commands::download_episode(id, cloned.state(), cloned.clone()).await });
        }
        ContextMenuOption::RemoveEpisodeDownload { id } => {
            commands::erase_episode_download(id, app_handle.clone())?;
        }
        ContextMenuOption::MarkAsCompleted { id } => {
            commands::mark_episode_complete(id, app_handle.clone())?;
        }
        ContextMenuOption::MarkAsNotCompleted { id } => {
            commands::mark_episode_not_complete(id, app_handle.clone())?;
        }
        ContextMenuOption::ShowFileInFolder { id } => {
            commands::show_episode_file_in_folder(id)?;
        }
        ContextMenuOption::GoToEpisode { id } => {
            app_handle.navigate(AppRoute::Episode { id })?;
        }
        ContextMenuOption::GoToPodcast { .. } => {}
        ContextMenuOption::Separator => {}
    }
    Ok(())
}
