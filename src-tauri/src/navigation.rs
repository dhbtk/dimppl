use crate::errors::AppResult;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum AppRoute {
    Home,
    Podcast { id: i32 },
    Episode { id: i32 },
    Settings,
    Podcasts,
    Downloads,
}

pub trait NavigationExt {
    fn navigate(&self, route: AppRoute) -> AppResult<()>;
}

impl NavigationExt for AppHandle {
    fn navigate(&self, route: AppRoute) -> AppResult<()> {
        self.emit("do-navigation", route)?;
        Ok(())
    }
}
