use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum WsMessage {
    ProgressUpdate {
        podcast_guid: String,
        episode_guid: String,
        listened_seconds: i32,
        completed: bool,
        updated_at: NaiveDateTime
    }
}