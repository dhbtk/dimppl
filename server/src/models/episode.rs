use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use dimppl_shared::progress::ProgressUpdateRequest;
use crate::database::AsyncConnection;
use crate::error_handling::AppResult;
use crate::models::Podcast;
use crate::models::podcast::SaveResult;

pub async fn update_progress<'a>(the_user_id: i64, request: ProgressUpdateRequest, conn: &mut AsyncConnection<'a>,) -> AppResult<SaveResult> {
    let podcast = {
        use crate::schema::podcasts::dsl::*;
        podcasts.select(Podcast::as_select()).filter(user_id.eq(the_user_id).and(guid.eq(request.podcast_guid)))
            .limit(1)
            .first(conn).await?
    };
    let count = {
        use crate::schema::podcast_episodes::dsl::*;
        diesel::update(podcast_episodes)
            .set((
                listened_seconds.eq(request.listened_seconds),
                completed.eq(request.completed),
                updated_at.eq(request.updated_at),
                ))
            .filter(
                podcast_id.eq(podcast.id)
                    .and(guid.eq(request.episode_guid))
                    .and(updated_at.lt(request.updated_at))
            )
            .execute(conn).await?
    };
    Ok(count.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Local, TimeDelta};
    use serial_test::serial;
    use dimppl_shared::progress::ProgressUpdateRequest;
    use crate::app::create_test_app;
    use crate::models::episode::update_progress;
    use crate::models::podcast::{test_podcast_with_episodes, SaveResult};
    use crate::models::PodcastEpisode;
    use crate::models::user_device::test_user_and_device;

    #[serial]
    #[tokio::test]
    async fn test_update_progress_successful_update() {
        let (state, _) = create_test_app();
        let mut conn = state.pool.get().await.unwrap();
        let (user, _device) = test_user_and_device(&mut conn).await.unwrap();
        let (existing_podcast, episodes) = test_podcast_with_episodes(&user, &mut conn).await.unwrap();
        
        let request = ProgressUpdateRequest {
            podcast_guid: existing_podcast.guid.clone(),
            episode_guid: episodes[1].guid.clone(),
            listened_seconds: 250,
            completed: true,
            updated_at: Local::now().naive_utc(),
        };
        
        let result = update_progress(user.id, request.clone(), &mut conn).await;
        assert!(result.is_ok());
        assert_eq!(Some(SaveResult::Saved), result.ok());
        
        let episode = {
            use crate::schema::podcast_episodes::dsl::*;
            podcast_episodes
                .filter(guid.eq(&episodes[1].guid))
                .select(PodcastEpisode::as_select())
                .load(&mut conn)
                .await.unwrap()
                .remove(0)
        };
        assert_eq!(250, episode.listened_seconds);
        assert!(episode.completed);
        assert_eq!(request.updated_at, episode.updated_at);
    }

    #[serial]
    #[tokio::test]
    async fn test_update_progress_unsuccessful() {
        let (state, _) = create_test_app();
        let mut conn = state.pool.get().await.unwrap();
        let (user, _device) = test_user_and_device(&mut conn).await.unwrap();
        let (existing_podcast, episodes) = test_podcast_with_episodes(&user, &mut conn).await.unwrap();

        let request = ProgressUpdateRequest {
            podcast_guid: existing_podcast.guid.clone(),
            episode_guid: episodes[0].guid.clone(),
            listened_seconds: 250,
            completed: true,
            updated_at: Local::now().naive_utc() - TimeDelta::days(1),
        };

        let result = update_progress(user.id, request.clone(), &mut conn).await;
        assert!(result.is_ok());
        assert_eq!(Some(SaveResult::NotSaved), result.ok());

        let episode = {
            use crate::schema::podcast_episodes::dsl::*;
            podcast_episodes
                .filter(guid.eq(&episodes[0].guid))
                .select(PodcastEpisode::as_select())
                .load(&mut conn)
                .await.unwrap()
                .remove(0)
        };
        assert_eq!(episodes[0].listened_seconds, episode.listened_seconds);
        assert!(episode.completed);
        assert_ne!(request.updated_at, episode.updated_at);
    }

    #[serial]
    #[tokio::test]
    async fn test_update_progress_no_podcast() {
        let (state, _) = create_test_app();
        let mut conn = state.pool.get().await.unwrap();
        let (user, _device) = test_user_and_device(&mut conn).await.unwrap();
        let (_existing_podcast, _episodes) = test_podcast_with_episodes(&user, &mut conn).await.unwrap();

        let request = ProgressUpdateRequest {
            podcast_guid: String::from("null"),
            episode_guid: String::from("null"),
            listened_seconds: 250,
            completed: true,
            updated_at: Local::now().naive_utc(),
        };

        let result = update_progress(user.id, request.clone(), &mut conn).await;
        assert!(result.is_err());
    }
}