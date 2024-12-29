use crate::errors::AppResult;
use tauri::{AppHandle, Emitter};

#[derive(Copy, Debug, Clone)]
pub enum EntityChange {
    AllPodcasts,
    Podcast(i32),
    PodcastEpisodes(i32),
    Episode(i32),
    EpisodeProgress(i32),
    AllDownloads,
    AllEpisodes,
}

impl EntityChange {
    pub fn cache_strings(&self) -> Vec<String> {
        match self {
            EntityChange::AllPodcasts => {
                vec![String::from("allPodcasts"), String::from("podcastStats")]
            }
            EntityChange::Podcast(id) => {
                vec![format!("podcast-{id}"), String::from("podcastStats")]
            }
            EntityChange::PodcastEpisodes(id) => {
                vec![format!("podcast-{id}"), String::from("podcastStats")]
            }
            EntityChange::Episode(id) => {
                vec![format!("episode-{id}"), String::from("podcastStats")]
            }
            EntityChange::EpisodeProgress(id) => {
                vec![format!("episodeProgress-{id}")]
            }
            EntityChange::AllDownloads => {
                vec![String::from("allDownloads")]
            }
            EntityChange::AllEpisodes => {
                vec![String::from("allEpisodes")]
            }
        }
    }
}

pub trait AppHandleExt {
    fn send_invalidate_cache(&self, key: EntityChange) -> AppResult<()>;
}

impl AppHandleExt for AppHandle {
    fn send_invalidate_cache(&self, key: EntityChange) -> AppResult<()> {
        for key in key.cache_strings() {
            self.emit("invalidate-cache", key)?;
        }
        Ok(())
    }
}
