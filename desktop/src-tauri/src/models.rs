pub mod episode;
pub mod episode_downloads;
pub mod podcast;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Identifiable, Clone, Debug)]
#[diesel(table_name = crate::schema::podcasts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct Podcast {
    pub id: i32,
    pub guid: String,
    pub author: String,
    pub local_image_path: String,
    pub image_url: String,
    pub feed_url: String,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PodcastStats {
    pub podcast: Podcast,
    pub total_episodes: i32,
    pub latest_ep_date: NaiveDateTime,
    pub last_listened_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Associations, Identifiable, Clone, Debug)]
#[diesel(table_name = crate::schema::episodes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Podcast))]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    pub id: i32,
    pub guid: String,
    pub podcast_id: i32,
    pub content_local_path: String,
    pub content_url: String,
    pub description: String,
    pub image_local_path: String,
    pub image_url: String,
    pub length: i32,
    pub link: String,
    pub episode_date: NaiveDateTime,
    pub title: String,
}

#[derive(Queryable, Selectable, Serialize, Associations, Identifiable, Clone, Debug)]
#[diesel(table_name = crate::schema::episode_progresses)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
#[diesel(belongs_to(Episode))]
pub struct EpisodeProgress {
    pub id: i32,
    pub episode_id: i32,
    pub completed: bool,
    pub listened_seconds: i32,
    pub updated_at: NaiveDateTime,
}
