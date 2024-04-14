use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, RwLock};

use crate::models::Episode;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

#[derive(Clone)]
pub struct EpisodeDownloads {
    in_progress: Arc<RwLock<HashMap<i32, EpisodeDownloadProgress>>>,
    episode_instance_cache: Arc<RwLock<Vec<Episode>>>,
    app_handle: AppHandle,
}

impl EpisodeDownloads {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            in_progress: Arc::new(RwLock::new(HashMap::new())),
            episode_instance_cache: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn in_progress(&self) -> HashMap<i32, EpisodeDownloadProgress> {
        let map = self.in_progress.read().unwrap();
        return map.deref().clone();
    }

    pub fn set_progress(&self, episode: &Episode, progress: EpisodeDownloadProgress) {
        let mut map = self.in_progress.write().unwrap();
        if !map.contains_key(&episode.id) {
            let mut cache = self.episode_instance_cache.write().unwrap();
            cache.push(episode.clone());
        }
        map.insert(episode.id, progress);
        self.broadcast_change(map.deref());
    }

    pub fn mark_done(&self, id: i32) {
        let mut map = self.in_progress.write().unwrap();
        map.remove(&id);
        self.broadcast_change(map.deref());
    }

    fn broadcast_change(&self, map: &HashMap<i32, EpisodeDownloadProgress>) {
        let episodes = self.episode_instance_cache.read().unwrap();
        let mut report: Vec<EpisodeDownloadProgressReport> = Vec::new();
        for episode in episodes.iter() {
            let Some(progress) = map.get(&episode.id) else {
                continue;
            };
            let EpisodeDownloadProgress {
                downloaded_bytes,
                total_bytes,
            } = progress;
            report.push(EpisodeDownloadProgressReport {
                episode: episode.clone(),
                downloaded_bytes: *downloaded_bytes,
                total_bytes: *total_bytes,
            });
        }
        let _ = self.app_handle.emit("episode-downloads", report);
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

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeDownloadProgressReport {
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub episode: Episode,
}
