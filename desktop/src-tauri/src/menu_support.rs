use crate::database::db_connect;
use crate::errors::{AppError, AppResult};
use crate::models::episode;
use crate::models::episode_downloads::EpisodeDownloads;
use crate::player::Player;
use anyhow::anyhow;
use derive_more::Display;
use serde::{Deserialize, Serialize};
#[cfg(not(target_os = "ios"))]
use tauri::menu::{IsMenuItem, MenuItem, MenuItemKind, PredefinedMenuItem};
#[cfg(not(target_os = "ios"))]
use tauri::menu::{Menu, MenuBuilder};
use tauri::{AppHandle, Wry};

const SER_TAG: &str = "ContextMenuOption--";

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub enum ContextMenuType {
    PodcastEpisode { id: i32 },
}

impl ContextMenuType {
    #[cfg(not(target_os = "ios"))]
    pub async fn show_context_menu(
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
                if !downloads.in_progress().await.contains_key(&id) {
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

#[derive(Serialize, Deserialize, Eq, PartialEq, Display, Debug)]
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

    #[cfg(not(target_os = "ios"))]
    pub fn menu_item(&self, app_handle: &AppHandle) -> Option<MenuItemKind<Wry>> {
        if *self == ContextMenuOption::Separator {
            return PredefinedMenuItem::separator(app_handle).ok().map(|i| i.kind());
        }
        MenuItem::with_id(
            app_handle,
            format!("{}{}", SER_TAG, serde_json::to_string(self).unwrap()),
            self.label(),
            true,
            None::<&str>,
        )
        .ok()
        .map(|i| i.kind())
    }

    #[cfg(not(target_os = "ios"))]
    pub fn make_menu_items(vec: Vec<Self>, app_handle: &AppHandle) -> Vec<MenuItemKind<Wry>> {
        vec.iter().filter_map(|option| option.menu_item(app_handle)).collect()
    }
}

impl TryFrom<String> for ContextMenuOption {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.starts_with(SER_TAG) {
            let result = serde_json::from_str::<ContextMenuOption>(value.strip_prefix(SER_TAG).unwrap());
            return match result {
                Ok(result) => Ok(result),
                Err(err) => Err(AppError(err.into())),
            };
        }
        Err(anyhow!("incorrect tag").into())
    }
}
