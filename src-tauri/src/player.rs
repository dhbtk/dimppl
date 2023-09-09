use std::io::BufReader;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use anyhow::anyhow;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use send_wrapper::SendWrapper;

use crate::errors::AppResult;
use crate::models::Episode;

#[allow(dead_code)]
pub struct Player {
    stream_handle: OutputStreamHandle,
    stream: SendWrapper<OutputStream>,
    sink: Sink,
    playing_episode: Arc<RwLock<Option<Episode>>>,
    played_millis: Arc<AtomicI64>,
}

impl Player {
    pub fn new() -> Self {
        let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        Self {
            sink: Sink::try_new(&stream_handle).unwrap(),
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
            let mut playing_episode = self.playing_episode.write().unwrap();
            if playing_episode.is_some() {
                *playing_episode = Some(episode.clone());
            }
        }
        let file = std::fs::File::open(episode.content_local_path)?;
        let reader = BufReader::new(file);
        self.played_millis
            .store((starting_seconds as i64) * 1000 - 100, Ordering::Relaxed);
        let cloned = self.played_millis.clone();
        let source = Decoder::new(reader)?
            .skip_duration(Duration::from_secs(starting_seconds))
            .periodic_access(Duration::from_millis(100), move |_src| {
                cloned.fetch_add(100, Ordering::Relaxed);
                // tracing::trace!("playback ms: {}", cloned.load(Ordering::Relaxed));
            });
        self.sink.stop();
        self.sink.append(source);
        // if self.sink.len() > 1 {
        //     self.sink.skip_one();
        // }
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
        Ok(())
    }

    #[allow(dead_code)]
    fn update_status(&mut self) {}
}
