use crate::database::AsyncConnection;
use crate::error_handling::AppResult;
use crate::models::{Podcast, User};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

pub async fn create<'a>(
    create_request: &CreatePodcastRequest,
    conn: &mut AsyncConnection<'a>,
) -> AppResult<()> {
    use crate::schema::podcasts::dsl::*;
    let podcast = diesel::insert_into(podcasts)
        .values((
            user_id.eq(create_request.user_id),
            url.eq(&create_request.url),
            guid.eq(&create_request.guid),
            updated_at.eq(NaiveDateTime::from_timestamp_opt(0, 0).unwrap()),
        ))
        .returning(Podcast::as_returning())
        .get_result(conn)
        .await?;
    for episode_request in &create_request.episodes {
        use crate::schema::podcast_episodes::dsl::*;
        diesel::insert_into(podcast_episodes)
            .values((
                podcast_id.eq(podcast.id),
                url.eq(&episode_request.url),
                guid.eq(&episode_request.guid),
                listened_seconds.eq(0),
                completed.eq(false),
                updated_at.eq(NaiveDateTime::from_timestamp_opt(0, 0).unwrap()),
            ))
            .execute(conn)
            .await?;
    }
    Ok(())
}

pub async fn sync_upsert_podcast<'a>(
    user: &User,
    sync_podcast: &SyncPodcast,
    conn: &mut AsyncConnection<'a>,
) -> AppResult<SaveResult> {
    use diesel::query_dsl::methods::FilterDsl;
    let update_count = {
        use crate::schema::podcasts::dsl::*;
        diesel::insert_into(podcasts)
            .values((
                user_id.eq(user.id),
                url.eq(&sync_podcast.url),
                guid.eq(&sync_podcast.guid),
                deleted_at.eq(sync_podcast.deleted_at),
                updated_at.eq(sync_podcast.updated_at),
            ))
            .on_conflict((user_id, guid))
            .do_update()
            .set((
                url.eq(&sync_podcast.url),
                deleted_at.eq(sync_podcast.deleted_at),
                updated_at.eq(sync_podcast.updated_at),
            ))
            .filter(updated_at.lt(sync_podcast.updated_at))
            .execute(conn)
            .await?
    };
    Ok(update_count.into())
}

pub async fn sync_upsert_episodes<'a>(
    user: &User,
    podcast_guid: &str,
    episodes: &[SyncPodcastEpisode],
    conn: &mut AsyncConnection<'a>,
) -> AppResult<()> {
    let podcast_record_id = {
        use crate::schema::podcasts::dsl::*;
        podcasts
            .filter(user_id.eq(user.id).and(guid.eq(podcast_guid)))
            .select(id)
            .first::<i64>(conn)
            .await?
    };
    for episode in episodes {
        use crate::schema::podcast_episodes::dsl::*;
        use diesel::query_dsl::methods::FilterDsl;

        diesel::insert_into(podcast_episodes)
            .values((
                podcast_id.eq(podcast_record_id),
                guid.eq(&episode.guid),
                url.eq(&episode.url),
                listened_seconds.eq(episode.listened_seconds),
                completed.eq(episode.completed),
                updated_at.eq(episode.updated_at),
            ))
            .on_conflict((podcast_id, guid))
            .do_update()
            .set((
                url.eq(&episode.url),
                listened_seconds.eq(episode.listened_seconds),
                completed.eq(episode.completed),
                updated_at.eq(episode.updated_at),
            ))
            .filter(updated_at.lt(episode.updated_at))
            .execute(conn)
            .await?;
    }
    Ok(())
}

#[derive(Eq, PartialEq, Debug)]
pub enum SaveResult {
    Saved,
    NotSaved,
}

impl From<usize> for SaveResult {
    fn from(value: usize) -> Self {
        if value > 0 {
            SaveResult::Saved
        } else {
            SaveResult::NotSaved
        }
    }
}

#[derive(Serialize, Deserialize, Selectable, Insertable)]
#[diesel(table_name = crate::schema::podcasts)]
pub struct SyncPodcast {
    pub guid: String,
    pub url: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Selectable, Insertable)]
#[diesel(table_name = crate::schema::podcast_episodes)]
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
    pub episodes: HashMap<String, SyncPodcastEpisode>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct SyncStateResponse {
    pub podcasts: Vec<SyncPodcast>,
    pub episodes: HashMap<String, SyncPodcastEpisode>,
}

#[cfg(test)]
mod tests {
    use crate::app::create_test_app;
    use crate::models::user_device::test_user_and_device;
    use crate::models::PodcastEpisode;
    use chrono::Local;
    use serial_test::serial;

    use super::*;
    #[tokio::test]
    #[serial]
    async fn test_sync_upsert_podcast_insertion() {
        let (state, _) = create_test_app();
        let mut conn = state.pool.get().await.unwrap();
        let (user, _device) = test_user_and_device(&mut conn).await.unwrap();
        let new_podcast = SyncPodcast {
            url: "https://google.com".into(),
            guid: "guid".into(),
            deleted_at: None,
            updated_at: NaiveDateTime::default(),
        };
        let result = sync_upsert_podcast(&user, &new_podcast, &mut conn).await;
        let query = {
            use crate::schema::podcasts::dsl::*;
            podcasts
                .filter(guid.eq("guid").and(user_id.eq(user.id)))
                .select(Podcast::as_select())
                .load(&mut conn)
                .await
        };
        assert_eq!(Some(SaveResult::Saved), result.ok());
        assert_eq!(Some(true), query.ok().map(|v| v.get(0).is_some()));
    }

    #[tokio::test]
    #[serial]
    async fn test_sync_upsert_podcast_update() {
        use crate::schema::podcasts::dsl::*;
        let (state, _) = create_test_app();
        let mut conn = state.pool.get().await.unwrap();
        let (user, _device) = test_user_and_device(&mut conn).await.unwrap();
        let _existing_podcast = diesel::insert_into(podcasts)
            .values((
                user_id.eq(user.id),
                url.eq("https://google.com"),
                guid.eq("guid"),
                updated_at.eq(NaiveDateTime::from_timestamp_opt(0, 0).unwrap()),
            ))
            .returning(Podcast::as_returning())
            .get_result(&mut conn)
            .await
            .unwrap();
        let new_podcast = SyncPodcast {
            url: "https://google2.com".into(),
            guid: "guid".into(),
            deleted_at: None,
            updated_at: Local::now().naive_utc(),
        };
        let result = sync_upsert_podcast(&user, &new_podcast, &mut conn).await;
        let query = {
            use crate::schema::podcasts::dsl::*;
            podcasts
                .filter(guid.eq("guid").and(user_id.eq(user.id)))
                .select(Podcast::as_select())
                .load(&mut conn)
                .await
        }
        .unwrap();
        let updated_podcast = query.get(0).expect("no podcast!");
        assert_eq!(Some(SaveResult::Saved), result.ok());
        assert_eq!(new_podcast.url, updated_podcast.url);
        assert_eq!(new_podcast.updated_at, updated_podcast.updated_at);
    }

    #[tokio::test]
    #[serial]
    async fn test_sync_upsert_podcast_no_update() {
        use crate::schema::podcasts::dsl::*;
        let (state, _) = create_test_app();
        let mut conn = state.pool.get().await.unwrap();
        let (user, _device) = test_user_and_device(&mut conn).await.unwrap();
        let existing_podcast = diesel::insert_into(podcasts)
            .values((
                user_id.eq(user.id),
                url.eq("https://google.com"),
                guid.eq("guid"),
                updated_at.eq(Local::now().naive_utc()),
            ))
            .returning(Podcast::as_returning())
            .get_result(&mut conn)
            .await
            .unwrap();
        let new_podcast = SyncPodcast {
            url: "https://google2.com".into(),
            guid: "guid".into(),
            deleted_at: None,
            updated_at: NaiveDateTime::default(),
        };
        let result = sync_upsert_podcast(&user, &new_podcast, &mut conn).await;
        let query = {
            use crate::schema::podcasts::dsl::*;
            podcasts
                .filter(guid.eq("guid").and(user_id.eq(user.id)))
                .select(Podcast::as_select())
                .load(&mut conn)
                .await
        }
        .unwrap();
        let updated_podcast = query.get(0).expect("no podcast!");
        assert_eq!(Some(SaveResult::NotSaved), result.ok());
        assert_eq!(existing_podcast.url, updated_podcast.url);
        assert_eq!(existing_podcast.updated_at, updated_podcast.updated_at);
    }

    #[tokio::test]
    #[serial]
    async fn test_sync_upsert_episodes() {
        let (state, _) = create_test_app();
        let mut conn = state.pool.get().await.unwrap();
        let (user, _device) = test_user_and_device(&mut conn).await.unwrap();
        let existing_podcast: Podcast = {
            use crate::schema::podcasts::dsl::*;
            diesel::insert_into(podcasts)
                .values((
                    user_id.eq(user.id),
                    url.eq("https://google.com"),
                    guid.eq("guid"),
                    updated_at.eq(Local::now().naive_utc()),
                ))
                .returning(Podcast::as_returning())
                .get_result(&mut conn)
                .await
                .unwrap()
        };
        let _episodes = {
            use crate::schema::podcast_episodes::dsl::*;
            diesel::insert_into(podcast_episodes)
                .values(&[
                    (
                        podcast_id.eq(existing_podcast.id),
                        guid.eq("ep1"),
                        url.eq("https://ep1"),
                        listened_seconds.eq(300),
                        completed.eq(true),
                        updated_at.eq(Local::now().naive_utc()),
                    ),
                    (
                        podcast_id.eq(existing_podcast.id),
                        guid.eq("ep2"),
                        url.eq("https://ep2"),
                        listened_seconds.eq(0),
                        completed.eq(false),
                        updated_at.eq(NaiveDateTime::default()),
                    ),
                ])
                .returning(PodcastEpisode::as_returning())
                .execute(&mut conn)
                .await
                .unwrap()
        };
        let episodes = vec![
            SyncPodcastEpisode {
                guid: "ep1".into(),
                url: "https://ep1.changed".into(),
                listened_seconds: 0,
                completed: false,
                updated_at: NaiveDateTime::default(),
            },
            SyncPodcastEpisode {
                guid: "ep2".into(),
                url: "https://ep2.changed".into(),
                listened_seconds: 500,
                completed: true,
                updated_at: Local::now().naive_utc(),
            },
            SyncPodcastEpisode {
                guid: "ep3".into(),
                url: "https://ep3".into(),
                listened_seconds: 350,
                completed: false,
                updated_at: Local::now().naive_utc(),
            },
        ];
        sync_upsert_episodes(&user, &existing_podcast.guid, &episodes, &mut conn)
            .await
            .unwrap();
        let query = {
            use crate::schema::podcast_episodes::dsl::*;
            podcast_episodes
                .filter(podcast_id.eq(existing_podcast.id))
                .select(PodcastEpisode::as_select())
                .order(guid.asc())
                .load(&mut conn)
                .await
        }
        .unwrap();
        assert_eq!(3, query.len());
        assert_eq!("https://ep1", query[0].url);
        assert_eq!("https://ep2.changed", query[1].url);
        assert_eq!(500, query[1].listened_seconds);
        assert!(query[1].completed);
        assert_eq!("https://ep3", query[2].url);
    }
}
