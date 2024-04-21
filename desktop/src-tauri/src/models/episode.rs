use std::cmp::{min, Reverse};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

use anyhow::Context;
use chrono::Utc;
use diesel::associations::HasTable;
use diesel::insert_into;
use diesel::prelude::*;
use futures_util::StreamExt;
use lofty::AudioFile;
use mime2ext::mime2ext;
use reqwest::header::CONTENT_TYPE;
use reqwest::Response;
use serde::Serialize;
use url::Url;

use crate::directories::podcast_downloads_dir;
use crate::errors::AppResult;
use crate::extensions::{ResponseExt, StrOptionExt};
use crate::models::episode_downloads::{EpisodeDownloadProgress, EpisodeDownloads};
use crate::models::podcast::NewProgress;
use crate::models::{Episode, EpisodeProgress, Podcast};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeWithProgress {
    pub episode: Episode,
    pub progress: EpisodeProgress,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeWithPodcast {
    pub episode: Episode,
    pub progress: EpisodeProgress,
    pub podcast: Podcast,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeWithFileSize {
    pub episode: Episode,
    pub progress: EpisodeProgress,
    pub podcast: Podcast,
    pub file_size: u64,
}

impl EpisodeWithProgress {
    pub fn new(episode: Episode, progress: EpisodeProgress) -> Self {
        Self { episode, progress }
    }
}

pub fn list_for_podcast(given_podcast_id: i32, conn: &mut SqliteConnection) -> AppResult<Vec<EpisodeWithProgress>> {
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

pub fn find_one(episode_id: i32, conn: &mut SqliteConnection) -> AppResult<Episode> {
    use crate::schema::episodes::dsl::*;
    let results = episodes.filter(id.eq(episode_id)).first(conn)?;
    Ok(results)
}

pub fn find_one_progress(the_episode_id: i32, conn: &mut SqliteConnection) -> AppResult<EpisodeProgress> {
    use crate::schema::episode_progresses::dsl::*;
    let results = episode_progresses.filter(episode_id.eq(the_episode_id)).first(conn)?;
    Ok(results)
}

pub fn find_one_full(the_episode_id: i32, conn: &mut SqliteConnection) -> AppResult<EpisodeWithPodcast> {
    use crate::schema::episode_progresses::dsl::episode_progresses;
    use crate::schema::episodes::dsl::*;
    use crate::schema::podcasts::dsl::podcasts;
    let results = episodes
        .inner_join(episode_progresses)
        .inner_join(podcasts)
        .filter(id.eq(the_episode_id))
        .select((EpisodeProgress::as_select(), Episode::as_select(), Podcast::as_select()))
        .first(conn)
        .map(|(progress, episode, podcast)| EpisodeWithPodcast {
            episode,
            progress,
            podcast,
        })?;
    Ok(results)
}

pub fn find_all_downloaded(conn: &mut SqliteConnection) -> AppResult<Vec<EpisodeWithFileSize>> {
    use crate::schema::episode_progresses::dsl::episode_progresses;
    use crate::schema::episodes::dsl::*;
    use crate::schema::podcasts::dsl::podcasts;
    let mut results = episodes
        .inner_join(episode_progresses)
        .inner_join(podcasts)
        .filter(content_local_path.ne(""))
        .select((EpisodeProgress::as_select(), Episode::as_select(), Podcast::as_select()))
        .load(conn)?
        .into_iter()
        .map(|(progress, episode, podcast)| EpisodeWithPodcast {
            episode,
            progress,
            podcast,
        })
        .filter_map(|data| {
            let Ok(file) = File::open(&data.episode.content_local_path) else {
                return None;
            };
            let Ok(metadata) = file.metadata() else { return None };
            Some(EpisodeWithFileSize {
                episode: data.episode,
                progress: data.progress,
                podcast: data.podcast,
                file_size: metadata.len(),
            })
        })
        .collect::<Vec<_>>();
    results.sort_by_key(|k| Reverse(k.file_size));
    Ok(results)
}

pub fn find_last_played(conn: &mut SqliteConnection) -> Option<EpisodeWithPodcast> {
    use crate::schema::episode_progresses::dsl::*;
    use crate::schema::episodes::dsl::*;
    use crate::schema::podcasts::dsl::podcasts;
    episodes
        .inner_join(episode_progresses)
        .inner_join(podcasts)
        .filter(listened_seconds.gt(0))
        .order_by(updated_at.desc())
        .select((EpisodeProgress::as_select(), Episode::as_select(), Podcast::as_select()))
        .first(conn)
        .map(|(progress, episode, podcast)| EpisodeWithPodcast {
            episode,
            progress,
            podcast,
        })
        .ok()
        .filter(|ep| !ep.progress.completed)
}

pub fn list_listen_history(conn: &mut SqliteConnection) -> AppResult<Vec<EpisodeWithPodcast>> {
    use crate::schema::episode_progresses::dsl::*;
    use crate::schema::episodes::dsl::*;
    use crate::schema::podcasts::dsl::podcasts;
    let results = episodes
        .inner_join(episode_progresses)
        .inner_join(podcasts)
        .filter(listened_seconds.gt(0))
        .order_by(updated_at.desc())
        .select((EpisodeProgress::as_select(), Episode::as_select(), Podcast::as_select()))
        .limit(100)
        .load::<(EpisodeProgress, Episode, Podcast)>(conn)?
        .into_iter()
        .map(|(progress, episode, podcast)| EpisodeWithPodcast {
            episode,
            progress,
            podcast,
        })
        .collect::<Vec<_>>();
    Ok(results)
}

pub fn list_latest_episodes(conn: &mut SqliteConnection) -> AppResult<Vec<EpisodeWithPodcast>> {
    use crate::schema::episode_progresses::dsl::*;
    use crate::schema::episodes::dsl::*;
    use crate::schema::podcasts::dsl::podcasts;
    let results = episodes
        .inner_join(episode_progresses)
        .inner_join(podcasts)
        .order_by(episode_date.desc())
        .select((EpisodeProgress::as_select(), Episode::as_select(), Podcast::as_select()))
        .limit(100)
        .load::<(EpisodeProgress, Episode, Podcast)>(conn)?
        .into_iter()
        .map(|(progress, episode, podcast)| EpisodeWithPodcast {
            episode,
            progress,
            podcast,
        })
        .collect::<Vec<_>>();
    Ok(results)
}

pub fn mark_as_complete(the_episode_id: i32, conn: &mut SqliteConnection) -> AppResult<i32> {
    use crate::schema::episode_progresses::dsl::*;
    diesel::update(episode_progresses)
        .set((completed.eq(true), updated_at.eq(Utc::now().naive_utc())))
        .filter(episode_id.eq(the_episode_id))
        .execute(conn)?;
    let progress = find_one_progress(the_episode_id, conn)?;
    Ok(progress.id)
}

pub fn mark_as_not_complete(the_episode_id: i32, conn: &mut SqliteConnection) -> AppResult<i32> {
    use crate::schema::episode_progresses::dsl::*;
    diesel::update(episode_progresses)
        .set((
            completed.eq(false),
            listened_seconds.eq(0),
            updated_at.eq(Utc::now().naive_utc()),
        ))
        .filter(episode_id.eq(the_episode_id))
        .execute(conn)?;
    let progress = find_one_progress(the_episode_id, conn)?;
    Ok(progress.id)
}

pub fn erase_downloaded_file(the_episode_id: i32, conn: &mut SqliteConnection) -> AppResult<()> {
    let episode = find_one(the_episode_id, conn)?;
    if episode.content_local_path.is_empty() {
        return Ok(());
    }
    let path = Path::new(&episode.content_local_path);
    if path.exists() {
        fs::remove_file(path)?;
    }
    {
        use crate::schema::episodes::dsl::*;
        diesel::update(episodes)
            .set(content_local_path.eq(""))
            .filter(id.eq(the_episode_id))
            .execute(conn)?;
    }
    Ok(())
}

pub async fn start_download(
    episode_id: i32,
    progress_indicator: &EpisodeDownloads,
    conn: &mut SqliteConnection,
) -> AppResult<()> {
    let episode = find_one(episode_id, conn)?;
    tracing::debug!("progress_indicator.set_progress");
    progress_indicator
        .set_progress(&episode, EpisodeDownloadProgress::default())
        .await;

    let response = reqwest::get(&episode.content_url).await?;
    if !response.status().is_success() {
        progress_indicator.mark_done(episode_id).await;
        return Ok(()); // this is a lie tho
    }

    let mut downloaded: u64 = 0;
    let total_length = response.content_length().unwrap_or(0);
    tracing::debug!("progress_indicator.set_progress total_length {total_length}");
    progress_indicator
        .set_progress(&episode, EpisodeDownloadProgress::new(downloaded, total_length))
        .await;

    let extension = extract_episode_filename_extension(&episode, &response);
    let file_name = format!("{}-{}.{}", episode.podcast_id, episode.id, extension);
    let path = podcast_downloads_dir().join(file_name);
    let mut file = File::create(&path)?;
    let mut event_emit_ts = Instant::now();
    let mut stream = response.bytes_stream();
    let mut chunk_count = 0;

    while let Some(item) = stream.next().await {
        chunk_count += 1;
        let chunk = item?;
        file.write_all(&chunk)?;
        let new = min(downloaded + (chunk.len() as u64), total_length);
        downloaded = new;
        if event_emit_ts.elapsed().as_millis() > 100 {
            progress_indicator
                .set_progress(&episode, EpisodeDownloadProgress::new(downloaded, total_length))
                .await;
            event_emit_ts = Instant::now();
        }
    }
    tracing::debug!("total chunks: {chunk_count}");
    let tagged_file = lofty::read_from_path(path.clone())?;
    let file_duration = tagged_file.properties().duration().as_secs();
    tracing::debug!(
        "file duration: {} previously known duration: {}",
        file_duration,
        episode.length
    );
    diesel::update(Episode::table())
        .filter(crate::schema::episodes::dsl::id.eq(episode_id))
        .set((
            crate::schema::episodes::dsl::content_local_path.eq(path.to_str().context("to_str")?.to_string()),
            crate::schema::episodes::dsl::length.eq(file_duration as i32),
        ))
        .execute(conn)?;
    progress_indicator.mark_done(episode_id).await;

    Ok(())
}

fn extract_episode_filename_extension(episode: &Episode, response: &Response) -> String {
    let response_extension = response
        .content_disposition_file_name()
        .ok()
        .and_then(|i| i.split('.').last().to_maybe_string());
    let url_extension = Url::parse(&episode.content_url)
        .ok()
        .and_then(|url| url.path().split('.').last().to_maybe_string());
    let mime_type_extension = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok().to_maybe_string())
        .and_then(|value| mime2ext(&value).to_maybe_string());
    let extension_possibilities = [response_extension, url_extension, mime_type_extension];
    let fallback_extension = Some("mp3".to_string());
    extension_possibilities
        .iter()
        .find(|i| i.is_some())
        .unwrap_or(&fallback_extension)
        .clone()
        .unwrap()
}

fn fix_missing_progress_entries(given_podcast_id: i32, conn: &mut SqliteConnection) -> AppResult<()> {
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
