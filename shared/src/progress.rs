use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgressUpdateRequest {
    pub podcast_guid: String,
    pub episode_guid: String,
    pub listened_seconds: i32,
    pub completed: bool,
    pub updated_at: NaiveDateTime
}