pub mod podcast;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::podcasts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
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
    pub updated_at: NaiveDateTime
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::episodes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
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
    pub title: String
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::episode_progresses)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct EpisodeProgress {
    pub id: i32,
    pub episode_id: i32,
    pub completed: bool,
    pub listened_seconds: i32,
    pub updated_at: NaiveDateTime
}
