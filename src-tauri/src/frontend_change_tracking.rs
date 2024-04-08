use crate::errors::AppResult;
use tauri::{AppHandle, Manager};

pub enum EntityChange {
    AllPodcasts,
    Podcast(i32),
    PodcastEpisodes(i32),
    Episode(i32),
    EpisodeProgress(i32),
}

impl From<EntityChange> for String {
    fn from(value: EntityChange) -> Self {
        match value {
            EntityChange::AllPodcasts => String::from("allPodcasts"),
            EntityChange::Podcast(id) => format!("podcast-{id}"),
            EntityChange::PodcastEpisodes(id) => format!("podcastEpisodes-{id}"),
            EntityChange::Episode(id) => format!("episode-{id}"),
            EntityChange::EpisodeProgress(id) => format!("episodeProgress-{id}"),
        }
    }
}

pub trait AppHandleExt {
    fn send_invalidate_cache(&self, key: impl Into<String>) -> AppResult<()>;
}

impl AppHandleExt for AppHandle {
    fn send_invalidate_cache(&self, key: impl Into<String>) -> AppResult<()> {
        self.emit("invalidate-cache", key.into())?;
        Ok(())
    }
}
