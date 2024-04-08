use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

use crate::backend::models::{SyncPodcast, SyncPodcastEpisode, SyncStateRequest};
use crate::database::db_connect;
use anyhow::Context;
use chrono::{NaiveDateTime, Utc};
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::{insert_into, update, SqliteConnection};
use futures::StreamExt;
use futures_util::stream::FuturesUnordered;
use rfc822_sanitizer::parse_from_rfc2822_with_fallback;
use rss::{Channel, Item};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::directories::images_dir;
use crate::errors::AppResult;
use crate::models::episode::list_for_podcast;
use crate::models::{Episode, Podcast};

pub fn list_all(conn: &mut SqliteConnection) -> AppResult<Vec<Podcast>> {
    use crate::schema::podcasts::dsl::*;
    let results = podcasts.order_by(name.asc()).select(Podcast::as_select()).load(conn)?;
    Ok(results)
}

pub fn find_one(podcast_id: i32, conn: &mut SqliteConnection) -> AppResult<Podcast> {
    use crate::schema::podcasts::dsl::*;
    let results = podcasts.filter(id.eq(podcast_id)).first(conn)?;
    Ok(results)
}

pub async fn import_podcast_from_url(url: String, conn: &mut SqliteConnection) -> AppResult<Podcast> {
    let parsed_podcast = download_rss_feed(url.clone(), None).await?;
    let inserted_podcast = {
        use crate::schema::podcasts::dsl::*;
        insert_into(podcasts::table())
            .values(NewPodcast::from_parsed(&parsed_podcast, url.clone()))
            .returning(Podcast::as_returning())
            .get_result(conn)?
    };
    for episode in &parsed_podcast.episodes {
        use crate::schema::episodes::dsl::*;
        insert_into(episodes::table())
            .values(NewEpisode::from_parsed(episode, inserted_podcast.id))
            .execute(conn)?;
    }
    Ok(inserted_podcast)
}

pub async fn sync_podcasts(conn: &mut SqliteConnection, app_handle: &AppHandle) -> AppResult<()> {
    let podcasts = list_all(conn)?
        .into_iter()
        .map(|podcast| tokio::spawn(sync_single_podcast(app_handle.clone(), podcast)))
        .collect::<FuturesUnordered<_>>();
    let _ = futures::future::join_all(podcasts).await;

    Ok(())
}

async fn sync_single_podcast(app_handle: AppHandle, podcast: Podcast) -> AppResult<()> {
    let mut conn = db_connect();
    let _ = app_handle.emit_all("sync-podcast-start", podcast.id);
    tracing::debug!("Updating podcast: {}", podcast.name.as_str());
    let parsed_podcast = download_rss_feed(podcast.feed_url.clone(), Some(podcast.guid.clone())).await?;
    let updated_podcast = UpdatedPodcast::new(
        podcast.id,
        NewPodcast::from_parsed(&parsed_podcast, podcast.feed_url.clone()),
    );
    diesel::update(Podcast::table().filter(crate::schema::podcasts::dsl::id.eq(podcast.id)))
        .set(updated_podcast)
        .execute(&mut conn)?;
    for episode in &parsed_podcast.episodes {
        let result = {
            use crate::schema::episodes::dsl::*;
            Episode::belonging_to(&podcast)
                .filter(guid.eq(episode.guid.clone()))
                .first::<Episode>(&mut conn)
        };
        if let Ok(episode_record) = result {
            use crate::schema::episodes::dsl::*;
            update(episodes)
                .set(content_url.eq(episode.content_url.clone()))
                .filter(id.eq(episode_record.id))
                .execute(&mut conn)?;
        } else {
            use crate::schema::episodes::dsl::*;
            insert_into(episodes::table())
                .values(NewEpisode::from_parsed(episode, podcast.id))
                .execute(&mut conn)?;
        }
    }
    let _ = app_handle.emit_all("sync-podcast-stop", podcast.id);
    Ok(())
}

pub fn build_backend_sync_request(conn: &mut SqliteConnection) -> AppResult<SyncStateRequest> {
    let mut podcasts: Vec<SyncPodcast> = Vec::new();
    let mut episodes: HashMap<String, Vec<SyncPodcastEpisode>> = HashMap::new();
    let podcast_query = list_all(conn)?;
    for podcast in podcast_query {
        let sync_podcast = podcast.clone().into();
        podcasts.push(sync_podcast);
        let episode_list = list_for_podcast(podcast.id, conn)?;
        episodes.insert(
            podcast.guid.clone(),
            episode_list.into_iter().map(|ep| ep.into()).collect(),
        );
    }

    Ok(SyncStateRequest { podcasts, episodes })
}

pub async fn download_rss_feed(url: String, identifier: Option<String>) -> AppResult<ParsedPodcast> {
    let content = reqwest::get(url).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    let podcast = ParsedPodcast::from_channel(channel, identifier).await?;
    Ok(podcast)
}

#[derive(Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::podcasts)]
struct UpdatedPodcast {
    pub id: i32,
    pub author: String,
    pub local_image_path: String,
    pub image_url: String,
    pub name: String,
    pub description: String,
    pub feed_url: String,
    pub updated_at: NaiveDateTime,
}

impl UpdatedPodcast {
    pub fn new(id: i32, new_podcast: NewPodcast) -> Self {
        let NewPodcast {
            author,
            local_image_path,
            image_url,
            name,
            description,
            feed_url,
            updated_at,
            ..
        } = new_podcast;
        Self {
            id,
            author,
            local_image_path,
            image_url,
            name,
            description,
            feed_url,
            updated_at,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::podcasts)]
struct NewPodcast {
    pub guid: String,
    pub author: String,
    pub local_image_path: String,
    pub image_url: String,
    pub name: String,
    pub description: String,
    pub feed_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl NewPodcast {
    fn from_parsed(parsed: &ParsedPodcast, url: String) -> Self {
        Self {
            guid: parsed.guid.clone(),
            author: parsed.author.clone(),
            local_image_path: parsed.local_image_path.clone(),
            image_url: parsed.image_url.clone(),
            name: parsed.name.clone(),
            description: parsed.description.clone(),
            created_at: Utc::now().naive_utc(),
            updated_at: parsed.published_at,
            feed_url: url,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::episodes)]
struct NewEpisode {
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

impl NewEpisode {
    pub fn from_parsed(parsed: &ParsedEpisode, podcast_id: i32) -> Self {
        Self {
            guid: parsed.guid.clone(),
            podcast_id,
            content_local_path: "".into(),
            content_url: parsed.content_url.clone(),
            description: parsed.description.clone(),
            image_local_path: "".into(),
            image_url: parsed.image_url.clone(),
            length: parsed.length,
            link: parsed.link.clone(),
            episode_date: parsed.episode_date,
            title: parsed.title.clone(),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::episode_progresses)]
pub struct NewProgress {
    pub episode_id: i32,
    pub completed: bool,
    pub listened_seconds: i32,
    pub updated_at: NaiveDateTime,
}

pub struct ParsedPodcast {
    pub guid: String,
    pub author: String,
    pub local_image_path: String,
    pub image_url: String,
    pub name: String,
    pub description: String,
    pub published_at: NaiveDateTime,
    pub episodes: Vec<ParsedEpisode>,
}

impl ParsedPodcast {
    pub async fn from_channel(channel: Channel, maybe_identifier: Option<String>) -> AppResult<Self> {
        let mut episodes: Vec<ParsedEpisode> = Vec::new();
        for item in &channel.items {
            let episode = ParsedEpisode::from_item(item.clone())?;
            episodes.push(episode);
        }
        let identifier = maybe_identifier.unwrap_or(Uuid::new_v4().to_string());
        let local_image_path = {
            match channel.image.clone() {
                None => "".into(),
                Some(image) => download_image(&image.url, &identifier).await?,
            }
        };
        let instance = Self {
            guid: identifier.clone(),
            author: channel.itunes_ext.and_then(|atom| atom.author).unwrap_or("".into()),
            local_image_path,
            image_url: channel.image.map(|i| i.url).unwrap_or("".into()),
            name: channel.title,
            description: channel.description,
            published_at: rfc822_to_naive_date_time(channel.pub_date),
            episodes,
        };
        Ok(instance)
    }
}

async fn download_image(image_url: &str, identifier: &str) -> AppResult<String> {
    let response = reqwest::get(image_url).await?;
    let extension = response
        .url()
        .path_segments()
        .and_then(|s| s.last())
        .and_then(|i| PathBuf::from(i).extension().map(|i| i.to_os_string()))
        .and_then(|i| i.into_string().ok())
        .unwrap_or("jpg".into());
    let filename = format!("podcastImage-{identifier}.{extension}");
    let file_path = images_dir().join(filename);
    let path_string = file_path.clone().into_os_string().into_string().unwrap();
    let mut tokio_file = tokio::fs::File::from(File::create(file_path)?);
    let mut byte_stream = response.bytes_stream();
    while let Some(data) = byte_stream.next().await {
        tokio::io::copy(&mut data?.as_ref(), &mut tokio_file).await?;
    }
    Ok(path_string)
}

pub struct ParsedEpisode {
    pub guid: String,
    pub content_url: String,
    pub description: String,
    pub image_url: String,
    pub length: i32,
    pub link: String,
    pub episode_date: NaiveDateTime,
    pub title: String,
}

impl ParsedEpisode {
    pub fn from_item(item: Item) -> AppResult<Self> {
        let description = if let Some(text) = item.description {
            text
        } else if let Some(itunes) = item.itunes_ext {
            itunes.summary.unwrap_or("".into())
        } else {
            "".into()
        };
        let enclosure = item.enclosure.context("episode with no enclosure")?;
        let instance = Self {
            guid: item.guid.context("no guid for episode!")?.value,
            content_url: enclosure.url.clone(),
            description,
            image_url: "".into(),
            length: enclosure.length.parse().unwrap_or(0), // TODO: parse from itunes extension
            link: item.link.context("episode with no link")?,
            title: item.title.context("episode with no title")?,
            episode_date: rfc822_to_naive_date_time(item.pub_date),
        };
        Ok(instance)
    }
}

fn rfc822_to_naive_date_time(string: Option<String>) -> NaiveDateTime {
    string
        .and_then(|pub_date_str| parse_from_rfc2822_with_fallback(pub_date_str).ok())
        .and_then(|timestamp| NaiveDateTime::from_timestamp_millis(timestamp.timestamp_millis()))
        .unwrap_or(NaiveDateTime::default())
}
