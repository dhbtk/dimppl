use crate::errors::AppResult;
use crate::models::podcast::NewProgress;
use crate::models::{Episode, EpisodeProgress};
use chrono::Utc;
use diesel::associations::HasTable;
use diesel::insert_into;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeWithProgress {
    pub episode: Episode,
    pub progress: EpisodeProgress,
}

impl EpisodeWithProgress {
    pub fn new(episode: Episode, progress: EpisodeProgress) -> Self {
        Self { episode, progress }
    }
}

pub fn list_for_podcast(
    given_podcast_id: i32,
    conn: &mut SqliteConnection,
) -> AppResult<Vec<EpisodeWithProgress>> {
    fix_missing_progress_entries(given_podcast_id, conn)?;
    let episodes_with_progress = EpisodeProgress::table()
        .inner_join(Episode::table())
        .filter(crate::schema::episodes::dsl::podcast_id.eq(given_podcast_id))
        .order_by(crate::schema::episodes::dsl::episode_date.desc())
        .select((EpisodeProgress::as_select(), Episode::as_select()))
        .load::<(EpisodeProgress, Episode)>(conn)?
        .iter()
        .map(|(progress, episode)| EpisodeWithProgress::new(episode.clone(), progress.clone()))
        .collect::<Vec<_>>();
    Ok(episodes_with_progress)
}

fn fix_missing_progress_entries(
    given_podcast_id: i32,
    conn: &mut SqliteConnection,
) -> AppResult<()> {
    let podcast = super::podcast::find_one(given_podcast_id, conn)?;
    let episodes = Episode::belonging_to(&podcast)
        .select(Episode::as_select())
        .load(conn)?;
    let ids = episodes.iter().map(|it| it.id).collect::<Vec<_>>();
    let ids_with_progress = crate::schema::episode_progresses::dsl::episode_progresses
        .filter(crate::schema::episode_progresses::dsl::episode_id.eq_any(&ids))
        .select(EpisodeProgress::as_select())
        .load(conn)?
        .iter()
        .map(|it| it.episode_id)
        .collect::<Vec<_>>();
    for episode_id in &ids {
        if !ids_with_progress.contains(episode_id) {
            let new_progress = NewProgress {
                episode_id: *episode_id,
                completed: false,
                listened_seconds: 0,
                updated_at: Utc::now().naive_utc(),
            };
            let _ = insert_into(EpisodeProgress::table())
                .values(new_progress)
                .execute(conn)?;
        }
    }
    Ok(())
}
