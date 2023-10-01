pub mod podcast;
pub mod user;
pub mod user_device;

use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub access_key: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::user_devices)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserDevice {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub last_session_at: chrono::NaiveDateTime,
    pub access_token: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::podcasts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Podcast {
    pub id: i64,
    pub user_id: i64,
    pub guid: String,
    pub url: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::podcast_episodes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PodcastEpisode {
    pub id: i64,
    pub podcast_id: i64,
    pub guid: String,
    pub url: String,
    pub listened_seconds: i32,
    pub completed: bool,
    pub updated_at: chrono::NaiveDateTime,
}
