use crate::models::episode::EpisodeWithProgress;
use crate::models::Podcast;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SyncPodcast {
    pub guid: String,
    pub url: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
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

#[derive(Serialize, Deserialize)]
pub struct SyncPodcastEpisode {
    pub guid: String,
    pub url: String,
    pub listened_seconds: i32,
    pub completed: bool,
    pub updated_at: NaiveDateTime,
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

#[derive(Serialize, Deserialize)]
pub struct SyncStateRequest {
    pub podcasts: Vec<SyncPodcast>,
    pub episodes: HashMap<String, Vec<SyncPodcastEpisode>>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct SyncStateResponse {
    pub podcasts: Vec<SyncPodcast>,
    pub episodes: HashMap<String, Vec<SyncPodcastEpisode>>,
}
