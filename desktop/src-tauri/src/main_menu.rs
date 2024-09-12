use crate::errors::{AppError, AppResult};
use crate::navigation::{AppRoute, NavigationExt};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use tauri::menu::{
    AboutMetadata, Menu, MenuId, MenuItem, PredefinedMenuItem, Submenu, HELP_SUBMENU_ID, WINDOW_SUBMENU_ID,
};
use tauri::{AppHandle, Wry};

const SER_TAG: &str = "MainMenuOption--";

#[derive(Serialize, Deserialize, Debug)]
pub enum MainMenuOption {
    Settings,
    AddNewPodcast,
    SyncFeeds,
    ManageFeeds,
    FindEpisode,
    NavigateLatestEpisodes,
    ManageDownloads,
    PlayPause,
    SkipForward,
    SkipBackward,
    NavigateToEpisode,
    IncreaseVolume,
    DecreaseVolume,
    PlaybackSpeed(i32),
    Help,
}

impl From<MainMenuOption> for MenuId {
    fn from(value: MainMenuOption) -> Self {
        Self(format!("{}{}", SER_TAG, serde_json::to_string(&value).unwrap()))
    }
}

impl TryFrom<String> for MainMenuOption {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.starts_with(SER_TAG) {
            let result = serde_json::from_str::<MainMenuOption>(value.strip_prefix(SER_TAG).unwrap());
            return match result {
                Ok(result) => Ok(result),
                Err(err) => Err(AppError(err.into())),
            };
        }
        Err(anyhow!("incorrect tag").into())
    }
}

#[cfg(not(target_os = "ios"))]
pub fn build_main_menu(app_handle: &AppHandle) -> AppResult<Menu<Wry>> {
    let pkg_info = app_handle.package_info();
    let config = app_handle.config();
    let about_metadata = AboutMetadata {
        name: Some(pkg_info.name.clone()),
        version: Some(pkg_info.version.to_string()),
        copyright: config.bundle.copyright.clone(),
        authors: config.bundle.publisher.clone().map(|p| vec![p]),
        ..Default::default()
    };

    let menu = Menu::with_items(
        app_handle,
        &[
            &Submenu::with_items(
                app_handle,
                pkg_info.name.clone(),
                true,
                &[
                    &PredefinedMenuItem::about(app_handle, None, Some(about_metadata))?,
                    &PredefinedMenuItem::separator(app_handle)?,
                    &MenuItem::with_id(app_handle, MainMenuOption::Settings, "Ajustes", true, Some("Cmd+,"))?,
                    &PredefinedMenuItem::services(app_handle, None)?,
                    &PredefinedMenuItem::separator(app_handle)?,
                    &PredefinedMenuItem::hide(app_handle, None)?,
                    &PredefinedMenuItem::hide_others(app_handle, None)?,
                    &PredefinedMenuItem::separator(app_handle)?,
                    &PredefinedMenuItem::quit(app_handle, None)?,
                ],
            )?,
            &Submenu::with_items(
                app_handle,
                "Podcasts",
                true,
                &[
                    &MenuItem::with_id(
                        app_handle,
                        MainMenuOption::AddNewPodcast,
                        "Adicionar Podcast...",
                        true,
                        Some("CmdOrCtrl+N"),
                    )?,
                    &MenuItem::with_id(
                        app_handle,
                        MainMenuOption::SyncFeeds,
                        "Sincronizar",
                        true,
                        Some("CmdOrCtrl+Shift+R"),
                    )?,
                    &PredefinedMenuItem::separator(app_handle)?,
                    &MenuItem::with_id(app_handle, MainMenuOption::ManageFeeds, "Gerenciar", true, None::<&str>)?,
                    #[cfg(not(target_os = "macos"))]
                    &PredefinedMenuItem::separator(app_handle)?,
                    #[cfg(not(target_os = "macos"))]
                    &MenuItem::with_id(
                        app_handle,
                        MainMenuOption::Settings,
                        "Configurações",
                        true,
                        Some("CmdOrCtrl-,"),
                    )?,
                    #[cfg(not(target_os = "macos"))]
                    &PredefinedMenuItem::separator(app_handle)?,
                    #[cfg(not(target_os = "macos"))]
                    &PredefinedMenuItem::quit(app_handle, None)?,
                ],
            )?,
            &Submenu::with_items(
                app_handle,
                "Editar",
                true,
                &[
                    &PredefinedMenuItem::undo(app_handle, None)?,
                    &PredefinedMenuItem::redo(app_handle, None)?,
                    &PredefinedMenuItem::separator(app_handle)?,
                    &PredefinedMenuItem::cut(app_handle, None)?,
                    &PredefinedMenuItem::copy(app_handle, None)?,
                    &PredefinedMenuItem::paste(app_handle, None)?,
                    &PredefinedMenuItem::select_all(app_handle, None)?,
                ],
            )?,
            &Submenu::with_items(
                app_handle,
                "Episódios",
                true,
                &[
                    &MenuItem::with_id(
                        app_handle,
                        MainMenuOption::FindEpisode,
                        "Buscar",
                        true,
                        Some("CmdOrCtrl-F"),
                    )?,
                    &MenuItem::with_id(
                        app_handle,
                        MainMenuOption::NavigateLatestEpisodes,
                        "Episódios Recentes",
                        true,
                        None::<&str>,
                    )?,
                    &MenuItem::with_id(
                        app_handle,
                        MainMenuOption::ManageDownloads,
                        "Gerenciar Downloads",
                        true,
                        None::<&str>,
                    )?,
                ],
            )?,
            &Submenu::with_items(
                app_handle,
                "Controles",
                true,
                &[
                    &MenuItem::with_id(
                        app_handle,
                        MainMenuOption::PlayPause,
                        "Não Reproduzindo",
                        false,
                        Some("Space"),
                    )?,
                    &MenuItem::with_id(
                        app_handle,
                        MainMenuOption::SkipForward,
                        "Avançar 30 seg",
                        false,
                        Some("CmdOrCtrl+Right"),
                    )?,
                    &MenuItem::with_id(
                        app_handle,
                        MainMenuOption::SkipBackward,
                        "Retroceder 15 seg",
                        false,
                        Some("CmdOrCtrl+Left"),
                    )?,
                    &MenuItem::with_id(
                        app_handle,
                        MainMenuOption::NavigateToEpisode,
                        "Ir para Episódio",
                        false,
                        Some("CmdOrCtrl+L"),
                    )?,
                    &PredefinedMenuItem::separator(app_handle)?,
                    &MenuItem::with_id(
                        app_handle,
                        MainMenuOption::IncreaseVolume,
                        "Aumentar Volume",
                        true,
                        Some("CmdOrCtrl+="),
                    )?,
                    &MenuItem::with_id(
                        app_handle,
                        MainMenuOption::DecreaseVolume,
                        "Diminuir Volume",
                        true,
                        Some("CmdOrCtrl+-"),
                    )?,
                    &PredefinedMenuItem::separator(app_handle)?,
                    &Submenu::with_items(
                        app_handle,
                        "Velocidade de Reprodução",
                        true,
                        &[
                            &MenuItem::with_id(
                                app_handle,
                                MainMenuOption::PlaybackSpeed(0),
                                "0,75x",
                                true,
                                Some("Option-1"),
                            )?,
                            &MenuItem::with_id(
                                app_handle,
                                MainMenuOption::PlaybackSpeed(1),
                                "1,0x",
                                true,
                                Some("Option-2"),
                            )?,
                            &MenuItem::with_id(
                                app_handle,
                                MainMenuOption::PlaybackSpeed(2),
                                "1,25x",
                                true,
                                Some("Option-3"),
                            )?,
                            &MenuItem::with_id(
                                app_handle,
                                MainMenuOption::PlaybackSpeed(3),
                                "1,5x",
                                true,
                                Some("Option-4"),
                            )?,
                            &MenuItem::with_id(
                                app_handle,
                                MainMenuOption::PlaybackSpeed(4),
                                "1,75x",
                                true,
                                Some("Option-5"),
                            )?,
                            &MenuItem::with_id(
                                app_handle,
                                MainMenuOption::PlaybackSpeed(5),
                                "2,0x",
                                true,
                                Some("Option-6"),
                            )?,
                        ],
                    )?,
                ],
            )?,
            &Submenu::with_id_and_items(
                app_handle,
                WINDOW_SUBMENU_ID,
                "Janela",
                true,
                &[
                    &PredefinedMenuItem::minimize(app_handle, None)?,
                    &PredefinedMenuItem::maximize(app_handle, None)?,
                    #[cfg(target_os = "macos")]
                    &PredefinedMenuItem::separator(app_handle)?,
                    &PredefinedMenuItem::close_window(app_handle, None)?,
                ],
            )?,
            &Submenu::with_id_and_items(
                app_handle,
                HELP_SUBMENU_ID,
                "Ajuda",
                true,
                &[
                    #[cfg(not(target_os = "macos"))]
                    &PredefinedMenuItem::about(app_handle, None, Some(about_metadata))?,
                ],
            )?,
        ],
    )?;
    Ok(menu)
}

pub fn main_menu_event_handler(app_handle: &AppHandle, option: MainMenuOption) {
    let result = event_handler_inner(app_handle, option);
    if let Err(err) = result {
        tracing::error!("Error handling main menu event: {}", err);
    }
}

fn event_handler_inner(app_handle: &AppHandle, option: MainMenuOption) -> AppResult<()> {
    match option {
        MainMenuOption::Settings => {
            app_handle.navigate(AppRoute::Settings)?;
        }
        MainMenuOption::AddNewPodcast => {}
        MainMenuOption::SyncFeeds => {}
        MainMenuOption::ManageFeeds => {
            app_handle.navigate(AppRoute::Podcasts)?;
        }
        MainMenuOption::FindEpisode => {}
        MainMenuOption::NavigateLatestEpisodes => {}
        MainMenuOption::ManageDownloads => {
            app_handle.navigate(AppRoute::Downloads)?;
        }
        MainMenuOption::PlayPause => {}
        MainMenuOption::SkipForward => {}
        MainMenuOption::SkipBackward => {}
        MainMenuOption::NavigateToEpisode => {}
        MainMenuOption::IncreaseVolume => {}
        MainMenuOption::DecreaseVolume => {}
        MainMenuOption::PlaybackSpeed(_) => {}
        MainMenuOption::Help => {}
    }
    Ok(())
}
