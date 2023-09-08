use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tokio::sync::RwLock;

pub struct EpisodeDownloads {
    in_progress: Arc<RwLock<HashMap<i32, EpisodeDownloadProgress>>>,
    app_handle: AppHandle,
}

impl EpisodeDownloads {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            in_progress: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    #[allow(dead_code)]
    pub async fn in_progress(&self) -> HashMap<i32, EpisodeDownloadProgress> {
        let map = self.in_progress.read().await;
        return map.deref().clone();
    }

    pub async fn set_progress(&self, id: i32, progress: EpisodeDownloadProgress) {
        let mut map = self.in_progress.write().await;
        map.insert(id, progress);
        self.broadcast_change(map.deref());
    }

    pub async fn mark_done(&self, id: i32) {
        let mut map = self.in_progress.write().await;
        map.remove(&id);
        self.broadcast_change(map.deref());
    }

    fn broadcast_change(&self, map: &HashMap<i32, EpisodeDownloadProgress>) {
        let _ = self.app_handle.emit_all("EpisodeDownloads", map);
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeDownloadProgress {
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
}

impl EpisodeDownloadProgress {
    pub fn new(downloaded_bytes: u64, total_bytes: u64) -> Self {
        Self {
            downloaded_bytes,
            total_bytes,
        }
    }
}
