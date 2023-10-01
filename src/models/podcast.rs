use crate::database::AsyncConnection;
use crate::error_handling::AppResult;
use crate::models::Podcast;
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, SelectableHelper};
use diesel_async::RunQueryDsl;
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
