use std::collections::HashMap;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CreatePodcastRequest {
    pub user_id: i64,
    pub url: String,
    pub guid: String,
    pub episodes: Vec<CreatePodcastEpisodeRequest>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreatePodcastEpisodeRequest {
    pub url: String,
    pub guid: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SyncPodcast {
    pub guid: String,
    pub url: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct SyncPodcastEpisode {
    pub guid: String,
    pub url: String,
    pub listened_seconds: i32,
    pub completed: bool,
    pub updated_at: NaiveDateTime,
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
