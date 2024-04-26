use serde::{Deserialize, Serialize};

use dimppl_shared::sync::{SyncPodcast, SyncPodcastEpisode};

use crate::models::episode::EpisodeWithProgress;
use crate::models::Podcast;

#[derive(Serialize, Deserialize)]
pub struct CreateDeviceRequest {
    pub user_access_key: String,
    pub device_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateDeviceResponse {
    pub name: String,
    pub access_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub access_key: String,
}

impl From<Podcast> for SyncPodcast {
    fn from(value: Podcast) -> Self {
        let Podcast {
            guid,
            feed_url,
            updated_at,
            ..
        } = value;
        Self {
            guid,
            url: feed_url,
            deleted_at: None,
            updated_at,
        }
    }
}

impl From<EpisodeWithProgress> for SyncPodcastEpisode {
    fn from(value: EpisodeWithProgress) -> Self {
        Self {
            guid: value.episode.guid,
            url: value.episode.content_url,
            listened_seconds: value.progress.listened_seconds,
            completed: value.progress.completed,
            updated_at: value.progress.updated_at,
        }
    }
}
