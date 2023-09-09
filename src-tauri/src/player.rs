use std::io::BufReader;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use crate::database::db_connect;
use anyhow::anyhow;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use send_wrapper::SendWrapper;
use serde::Serialize;
use tauri::{AppHandle, Manager};

use crate::errors::AppResult;
use crate::models::{podcast, Episode, Podcast};

#[allow(dead_code)]
pub struct Player {
    app_handle: AppHandle,
    stream_handle: OutputStreamHandle,
    stream: SendWrapper<OutputStream>,
    sink: Arc<Sink>,
    playing_episode: Arc<RwLock<Option<(Episode, Podcast)>>>,
    played_millis: Arc<AtomicI64>,
}

impl Player {
    pub fn new(app_handle: AppHandle) -> Self {
        let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        Self {
            app_handle,
            sink: Arc::new(Sink::try_new(&stream_handle).unwrap()),
            stream_handle,
            stream: SendWrapper::new(stream),
            playing_episode: Arc::new(RwLock::new(None)),
            played_millis: Arc::new(AtomicI64::new(0)),
        }
    }

    pub fn play_episode(&self, episode: Episode, starting_seconds: u64) -> AppResult<()> {
        if episode.content_local_path.is_empty() {
            return Err(anyhow!("no content_local_path").into());
        }
        {
            let mut conn = db_connect();
            let podcast = podcast::find_one(episode.podcast_id, &mut conn)?;
            let mut playing_episode = self.playing_episode.write().unwrap();
            *playing_episode = Some((episode.clone(), podcast));
        }
        let file = std::fs::File::open(episode.content_local_path)?;
        let reader = BufReader::new(file);
        self.played_millis
            .store((starting_seconds as i64) * 1000 - 100, Ordering::Relaxed);
        let cloned_atomic = self.played_millis.clone();
        let cloned_sink = self.sink.clone();
        let cloned_handle = self.app_handle.clone();
        let cloned_episode = self.playing_episode.clone();
        let source = Decoder::new(reader)?
            .skip_duration(Duration::from_secs(starting_seconds))
            .periodic_access(Duration::from_millis(100), move |_src| {
                cloned_atomic.fetch_add(100, Ordering::Relaxed);
                let maybe_episode = cloned_episode.read().unwrap();
                Self::broadcast_status(
                    &cloned_handle,
                    maybe_episode.clone(),
                    &cloned_sink,
                    cloned_atomic.load(Ordering::Relaxed),
                );
                tracing::debug!(
                    "playback ms: {} paused: {} episode: {:?}",
                    cloned_atomic.load(Ordering::Relaxed),
                    cloned_sink.is_paused(),
                    maybe_episode.as_ref().map(|(it, _)| it.title.clone())
                );
            });
        self.sink.stop();
        self.sink.append(source);
        tracing::debug!(
            "queue size: {} is paused: {} volume: {} speed: {}",
            self.sink.len(),
            self.sink.is_paused(),
            self.sink.volume(),
            self.sink.speed()
        );
        self.sink.play();
        self.sink.sleep_until_end();
        tracing::info!("finished playback");
        self.broadcast_status_self();
        Ok(())
    }

    pub fn play(&self) {
        if self.playing_episode.read().unwrap().is_none() {
            return;
        }
        self.sink.play();
        self.broadcast_status_self();
    }

    pub fn pause(&self) {
        if self.playing_episode.read().unwrap().is_none() {
            return;
        }
        self.sink.pause();
        self.broadcast_status_self();
    }

    pub fn skip_forwards(&self) {
        self.seek_to(self.played_millis.load(Ordering::Relaxed) / 1000 + 30);
    }

    pub fn skip_backwards(&self) {
        self.seek_to(self.played_millis.load(Ordering::Relaxed) / 1000 - 15);
    }

    pub fn seek_to(&self, seconds: i64) {
        if self.playing_episode.read().unwrap().is_none() || seconds < 0 {
            return;
        }
        let episode = self
            .playing_episode
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .0
            .clone();
        let _ = self.play_episode(episode, seconds.unsigned_abs());
    }

    fn broadcast_status_self(&self) {
        let episode = self.playing_episode.read().unwrap();
        let elapsed = self.played_millis.load(Ordering::Relaxed);
        let sink_ref = self.sink.clone();
        Self::broadcast_status(
            &self.app_handle,
            episode.clone(),
            sink_ref.as_ref(),
            elapsed,
        );
    }

    fn broadcast_status(
        app_handle: &AppHandle,
        episode_container: Option<(Episode, Podcast)>,
        sink: &Sink,
        elapsed: i64,
    ) {
        let _ = app_handle.emit_all(
            "player-status",
            PlayerStatus {
                is_paused: sink.is_paused(),
                episode: episode_container.as_ref().map(|(ep, _)| ep.clone()),
                podcast: episode_container.map(|(_, podcast)| podcast),
                elapsed,
            },
        );
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStatus {
    pub is_paused: bool,
    pub episode: Option<Episode>,
    pub podcast: Option<Podcast>,
    pub elapsed: i64,
}
