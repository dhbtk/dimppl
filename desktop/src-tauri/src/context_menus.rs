use crate::commands;
use tauri::{AppHandle, Manager};

use crate::errors::AppResult;
use crate::menu_support::ContextMenuOption;
use crate::navigation::{AppRoute, NavigationExt};

pub fn context_menu_event_handler(app_handle: &AppHandle, option: ContextMenuOption) {
    let result = menu_event_handler_inner(app_handle, option);
    if let Err(err) = result {
        tracing::error!("Error handling menu event: {}", err);
    }
}

fn menu_event_handler_inner(app_handle: &AppHandle, option: ContextMenuOption) -> AppResult<()> {
    tracing::info!("context menu item click: {option}");
    match option {
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
