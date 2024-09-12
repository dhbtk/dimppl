use std::f32::consts::E;
use std::ffi::c_void;
use std::fs::File;
use std::ops::Deref;
use std::path::Path;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread::JoinHandle;
use std::time::Duration;

use anyhow::anyhow;
use chrono::Utc;
use diesel::associations::HasTable;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use lofty::AudioFile;
use souvlaki::{
    MediaControlEvent, MediaControls, MediaMetadata, MediaPlayback, MediaPosition, PlatformConfig, SeekDirection,
};

use symphonia::core::audio::{AsAudioBufferRef, AudioBuffer, Signal};
use symphonia::core::codecs::CODEC_TYPE_NULL;
use symphonia::core::errors::Error;
use symphonia::core::formats::{FormatReader, SeekMode, SeekTo, Track};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::probe::Hint;
use symphonia::core::units::{Time, TimeStamp};
use tauri::{AppHandle, Emitter, Manager};
use tokio::time::Instant;

use crate::database::db_connect;
use crate::errors::{AppError, AppResult};
use crate::frontend_change_tracking::{AppHandleExt, EntityChange};
use crate::models::{podcast, Episode, EpisodeProgress, Podcast};
use crate::player::{output, PlayerStatus};

#[derive(Clone)]
pub struct NewPlayer {
    app_handle: AppHandle,
    sender_channel: Arc<Mutex<Option<Sender<PlayerCommand>>>>,
    thread_handle: Arc<Mutex<Option<JoinHandle<AppResult<()>>>>>,
    playing_episode: Arc<RwLock<Option<(Episode, Podcast)>>>,
    played_millis: Arc<AtomicI64>,
    episode_length: Arc<RwLock<i64>>,
    is_paused: Arc<RwLock<bool>>,
    volume: Arc<RwLock<f32>>,
    playback_speed: Arc<RwLock<f32>>,
    media_controls: Arc<RwLock<Option<MediaControls>>>,
    last_artwork_episode_id: Arc<RwLock<i32>>,
    latest_status: Arc<Mutex<Option<PlayerStatus>>>,
}

enum PlayerCommand {
    Pause,
    Resume,
    Seek(u64),
    Stop,
}

impl NewPlayer {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            sender_channel: Arc::new(Mutex::new(None)),
            thread_handle: Arc::new(Mutex::new(None)),
            playing_episode: Arc::new(RwLock::new(None)),
            played_millis: Arc::new(AtomicI64::new(0)),
            episode_length: Arc::new(RwLock::new(0)),
            is_paused: Arc::new(RwLock::new(false)),
            volume: Arc::new(RwLock::new(1.0)),
            playback_speed: Arc::new(RwLock::new(1.0)),
            media_controls: Arc::new(RwLock::new(None)),
            last_artwork_episode_id: Arc::new(RwLock::new(0)),
            latest_status: Default::default(),
        }
    }

    pub fn latest_status(&self) -> Option<PlayerStatus> {
        self.latest_status.clone().lock().unwrap().clone()
    }

    pub fn set_up_media_controls(&self, handle: Option<*mut c_void>) {
        let config = PlatformConfig {
            dbus_name: "dimppl-desktop",
            display_name: "Dimppl",
            hwnd: handle,
            podcast_controls: true,
        };
        let mut controls = MediaControls::new(config).unwrap();
        let cloned_self = self.clone();
        controls
            .attach(move |event| {
                tracing::info!("MediaControlEvent: {:?}", &event);
                match event {
                    MediaControlEvent::Play => {
                        cloned_self.play();
                    }
                    MediaControlEvent::Pause => {
                        cloned_self.pause();
                    }
                    MediaControlEvent::Toggle => {
                        if *cloned_self.is_paused.read().unwrap() {
                            cloned_self.play();
                        } else {
                            cloned_self.pause();
                        }
                    }
                    MediaControlEvent::Next => {
                        // TODO: queue?
                        cloned_self.skip_forwards();
                    }
                    MediaControlEvent::Previous => {
                        cloned_self.skip_backwards();
                    }
                    MediaControlEvent::Stop => {
                        // TODO
                    }
                    MediaControlEvent::SkipBackward(duration) => {
                        let elapsed_seconds = cloned_self.played_millis.load(Ordering::Relaxed) / 1000;
                        let pos = elapsed_seconds - duration.as_secs() as i64;
                        cloned_self.seek_to(pos);
                    }
                    MediaControlEvent::SkipForward(duration) => {
                        let elapsed_seconds = cloned_self.played_millis.load(Ordering::Relaxed) / 1000;
                        let pos = elapsed_seconds + duration.as_secs() as i64;
                        cloned_self.seek_to(pos);
                    }
                    MediaControlEvent::Seek(direction) => match direction {
                        SeekDirection::Forward => cloned_self.skip_forwards(),
                        SeekDirection::Backward => cloned_self.skip_backwards(),
                    },
                    MediaControlEvent::SeekBy(direction, duration) => {
                        let elapsed_seconds = cloned_self.played_millis.load(Ordering::Relaxed) / 1000;
                        let delta = if direction == SeekDirection::Forward {
                            duration.as_secs() as i64
                        } else {
                            -(duration.as_secs() as i64)
                        };
                        cloned_self.seek_to(elapsed_seconds + delta);
                    }
                    MediaControlEvent::SetPosition(pos) => {
                        cloned_self.seek_to(pos.0.as_secs() as i64);
                    }
                    MediaControlEvent::OpenUri(_) => {}
                    MediaControlEvent::Raise => {
                        #[cfg(not(target_os = "ios"))]
                        if let Some(window) = cloned_self.app_handle.get_webview_window("main") {
                            let _ = window.unminimize();
                        }
                    }
                    MediaControlEvent::Quit => {
                        cloned_self.app_handle.exit(0);
                    }
                }
            })
            .unwrap();

        *self.media_controls.write().unwrap() = Some(controls);
    }

    pub fn play_episode(&self, episode: Episode, starting_at: i32) -> AppResult<()> {
        if episode.content_local_path.is_empty() {
            return Err(anyhow!("no content_local_path").into());
        }
        {
            let mut conn = db_connect();
            let podcast = podcast::find_one(episode.podcast_id, &mut conn)?;
            let mut playing_episode = self.playing_episode.write().unwrap();
            *playing_episode = Some((episode.clone(), podcast));
        }
        let tagged_file = lofty::read_from_path(episode.content_local_path.as_str())?;
        let file_duration = tagged_file.properties().duration().as_secs();
        *self.episode_length.write().unwrap() = file_duration as i64;
        self.played_millis.store((starting_at as i64) * 1000, Ordering::Relaxed);
        *self.is_paused.write().unwrap() = false;
        self.broadcast_status_self(true);
        let path_str = episode.content_local_path.as_str();
        let mut hint = Hint::new();
        let source = {
            let path = Path::new(path_str);
            if let Some(extension) = path.extension() {
                if let Some(extension_str) = extension.to_str() {
                    hint.with_extension(extension_str);
                }
            }
            Box::new(File::open(path)?)
        };
        let mss = MediaSourceStream::new(source, Default::default());
        let probed = symphonia::default::get_probe().format(&hint, mss, &Default::default(), &Default::default())?;
        self.play_track(probed.format, starting_at)?;
        Ok(())
    }

    pub fn play(&self) {
        if self.playing_episode.read().unwrap().is_none() {
            return;
        }
        if let Some(channel) = self.sender_channel.lock().unwrap().deref() {
            let _ = channel.send(PlayerCommand::Resume);
        }
        *self.is_paused.write().unwrap() = false;
        self.broadcast_status_self(true);
    }

    pub fn pause(&self) {
        if self.playing_episode.read().unwrap().is_none() {
            return;
        }
        if let Some(channel) = self.sender_channel.lock().unwrap().deref() {
            let _ = channel.send(PlayerCommand::Pause);
        }
        *self.is_paused.write().unwrap() = true;
        self.broadcast_status_self(true);
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
        if let Some(channel) = self.sender_channel.lock().unwrap().deref() {
            let _ = channel.send(PlayerCommand::Seek(seconds.unsigned_abs()));
        }
    }

    pub fn set_volume(&self, volume: f32) {
        *self.volume.write().unwrap() = (volume.exp() - 1.0) / (E - 1.0);
    }

    pub fn set_playback_speed(&self, speed: f32) {
        *self.playback_speed.write().unwrap() = speed;
    }

    fn broadcast_status_self(&self, save_progress: bool) {
        let episode = self.playing_episode.read().unwrap();
        let elapsed = self.played_millis.load(Ordering::Relaxed);
        let status = Self::broadcast_status(
            &self.app_handle,
            episode.clone(),
            *self.is_paused.read().unwrap(),
            elapsed,
            *self.episode_length.read().unwrap(),
            false,
            save_progress,
            &mut self.media_controls.write().unwrap(),
            &mut self.last_artwork_episode_id.write().unwrap(),
        );
        *self.latest_status.lock().unwrap() = Some(status);
    }

    #[allow(clippy::too_many_arguments)]
    fn broadcast_status(
        app_handle: &AppHandle,
        episode_container: Option<(Episode, Podcast)>,
        paused: bool,
        elapsed: i64,
        duration: i64,
        loading: bool,
        save_progress: bool,
        maybe_controls: &mut Option<MediaControls>,
        last_seen_episode_id: &mut i32,
    ) -> PlayerStatus {
        if save_progress && episode_container.is_some() {
            use crate::schema::episode_progresses::dsl::*;
            let (episode, _) = episode_container.clone().unwrap();
            let elapsed_seconds = elapsed / 1000;
            let completed_listening = (episode.length as i64) - elapsed_seconds < 300; // 5 minutes
            let mut conn = db_connect();
            tracing::trace!(
                "{} seconds elapsed, saving progress for episode id {}",
                elapsed_seconds,
                episode.id
            );
            let _ = diesel::update(EpisodeProgress::table())
                .filter(episode_id.eq(episode.id))
                .set((
                    listened_seconds.eq(elapsed_seconds as i32),
                    completed.eq(completed_listening),
                    updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(&mut conn);
            let progress = episode_progresses
                .select(EpisodeProgress::as_select())
                .filter(episode_id.eq(episode.id))
                .first(&mut conn)
                .unwrap();
            let _ = app_handle.send_invalidate_cache(EntityChange::EpisodeProgress(progress.id));
        }
        let status = PlayerStatus {
            is_paused: paused,
            episode: episode_container.as_ref().map(|(ep, _)| ep.clone()),
            podcast: episode_container.as_ref().map(|(_, podcast)| podcast.clone()),
            elapsed: elapsed / 1000,
            duration,
            loading,
        };
        let _ = app_handle.emit("player-status", status.clone());
        if save_progress {
            if let Some(controls) = maybe_controls {
                if let Some((episode, podcast)) = episode_container {
                    if *last_seen_episode_id != episode.id {
                        *last_seen_episode_id = episode.id;
                        let cover_url = Some(podcast.image_url.as_str());
                        let _ = controls.set_metadata(MediaMetadata {
                            title: Some(episode.title.as_str()),
                            album: None,
                            artist: Some(podcast.name.as_str()),
                            cover_url,
                            duration: Some(Duration::from_secs(episode.length.unsigned_abs() as u64)),
                        });
                    }
                    let progress = Some(MediaPosition(Duration::from_millis(elapsed.unsigned_abs())));
                    let _ = controls.set_playback(if paused {
                        MediaPlayback::Paused { progress }
                    } else {
                        MediaPlayback::Playing { progress }
                    });
                } else {
                    let _ = controls.set_metadata(Default::default());
                    let _ = controls.set_playback(MediaPlayback::Stopped);
                }
            }
        }
        status
    }

    fn play_track(&self, mut reader: Box<dyn FormatReader>, starting_at: i32) -> AppResult<()> {
        let track = first_supported_track(reader.tracks());
        let track_id = match track {
            Some(track) => track.id,
            _ => return Ok(()),
        };
        let seek_ts = if starting_at != 0 {
            Self::calculate_seek_timestamp(&mut reader, starting_at, track_id)
        } else {
            // If not seeking, the seek timestamp is 0.
            0
        };

        let (tx, rx) = mpsc::channel::<PlayerCommand>();

        let mut sender_mutex = self.sender_channel.lock().unwrap();

        if let Some(sender) = sender_mutex.deref() {
            let _ = sender.send(PlayerCommand::Stop);
        }
        {
            let mut handle_mutex = self.thread_handle.lock().unwrap();
            let handle = handle_mutex.take();
            if let Some(handle) = handle {
                let _ = handle.join();
            }
            *handle_mutex = None;
        }
        *sender_mutex = Some(tx);

        let cloned_self = self.clone();

        let handle = Self::spawn_player_thread(reader, track_id, seek_ts, rx, cloned_self).unwrap();
        *self.thread_handle.lock().unwrap() = Some(handle);
        Ok(())
    }

    fn spawn_player_thread(
        reader: Box<dyn FormatReader>,
        track_id: u32,
        seek_ts: TimeStamp,
        rx: Receiver<PlayerCommand>,
        cloned_self: NewPlayer,
    ) -> Result<JoinHandle<Result<(), AppError>>, AppError> {
        let handle = std::thread::Builder::new()
            .name("new player thread".into())
            .spawn(move || Self::player_thread_loop(reader, track_id, seek_ts, rx, &cloned_self))?;
        Ok(handle)
    }

    fn player_thread_loop(
        mut reader: Box<dyn FormatReader>,
        track_id: u32,
        mut seek_ts: TimeStamp,
        rx: Receiver<PlayerCommand>,
        cloned_self: &NewPlayer,
    ) -> Result<(), AppError> {
        let mut save_timer = Instant::now();
        let mut update_timer = Instant::now();
        let mut is_paused = false;
        let mut interrupted = false;
        let track = match reader.tracks().iter().find(|track| track.id == track_id) {
            Some(track) => track,
            _ => return Ok(()),
        };

        let mut decoder = symphonia::default::get_codecs().make(&track.codec_params, &Default::default())?;
        let track_time_base = track.codec_params.time_base;
        let mut audio_output: Option<Box<dyn output::AudioOutput>> = None;
        let mut sample_buffer: Option<AudioBuffer<f32>> = None;
        let _: Result<(), Error> = loop {
            if let Ok(command) = rx.try_recv() {
                match command {
                    PlayerCommand::Pause => is_paused = true,
                    PlayerCommand::Resume => is_paused = false,
                    PlayerCommand::Seek(new_position) => {
                        let old_ts = seek_ts;
                        seek_ts = {
                            let seek_to = SeekTo::Time {
                                time: Time::from(new_position),
                                track_id: Some(track_id),
                            };
                            match reader.seek(SeekMode::Accurate, seek_to) {
                                Ok(seeked_to) => seeked_to.required_ts,
                                Err(_) => old_ts,
                            }
                        };
                    }
                    PlayerCommand::Stop => {
                        interrupted = true;
                        break Ok(());
                    }
                }
            }
            if is_paused {
                std::thread::sleep(Duration::from_millis(10));
                continue;
            }

            let packet = match reader.next_packet() {
                Ok(packet) => packet,
                Err(err) => break Err(err),
            };

            if packet.track_id() != track_id {
                continue;
            }

            match decoder.decode(&packet) {
                Ok(decoded) => {
                    if audio_output.is_none() {
                        let spec = *decoded.spec();
                        let duration = decoded.capacity() as u64;
                        audio_output.replace(output::try_open(spec, duration).unwrap());
                    } else {
                        // TODO: Check the audio spec. and duration hasn't changed.
                    }

                    if sample_buffer.is_none() {
                        sample_buffer = Some(AudioBuffer::<f32>::new(decoded.capacity() as u64, *decoded.spec()));
                    }
                    if let Some(buffer) = &mut sample_buffer {
                        if packet.ts() >= seek_ts {
                            if let Some(time_base) = track_time_base {
                                let time = time_base.calc_time(packet.ts);
                                cloned_self
                                    .played_millis
                                    .store((time.seconds * 1000) as i64, Ordering::Relaxed);
                            }
                            if let Some(audio_output) = &mut audio_output {
                                decoded.convert(buffer);
                                let volume = *cloned_self.volume.read().unwrap();
                                buffer.transform(|sample| sample * volume);
                                audio_output.write(buffer.as_audio_buffer_ref()).unwrap()
                            }
                        }
                    }
                    if save_timer.elapsed().as_millis() > 1000 {
                        save_timer = Instant::now();
                        cloned_self.broadcast_status_self(true);
                    } else if update_timer.elapsed().as_millis() > 100 {
                        update_timer = Instant::now();
                        cloned_self.broadcast_status_self(true);
                    }
                }
                Err(Error::DecodeError(err)) => {
                    // Decode errors are not fatal. Print the error message and try to decode the next
                    // packet as usual.
                    tracing::warn!("decode error: {}", err);
                }
                Err(err) => break Err(err),
            }
        };
        if !interrupted {
            // TODO: mark episode as done
            cloned_self.broadcast_status_self(true);
            *cloned_self.playing_episode.write().unwrap() = None;
            cloned_self.played_millis.store(0, Ordering::Relaxed);
            *cloned_self.episode_length.write().unwrap() = 0;
            cloned_self.broadcast_status_self(true);
        }
        tracing::debug!("thread is now dying");
        Ok(())
    }

    fn calculate_seek_timestamp(reader: &mut Box<dyn FormatReader>, starting_at: i32, track_id: u32) -> TimeStamp {
        let seek_to = SeekTo::Time {
            time: Time::from(starting_at.unsigned_abs()),
            track_id: Some(track_id),
        };

        // Attempt the seek. If the seek fails, ignore the error and return a seek timestamp of 0 so
        // that no samples are trimmed.
        match reader.seek(SeekMode::Accurate, seek_to) {
            Ok(seeked_to) => seeked_to.required_ts,
            Err(Error::ResetRequired) => 0,
            Err(err) => {
                // Don't give-up on a seek error.
                tracing::warn!("seek error: {}", err);
                0
            }
        }
    }
}

fn first_supported_track(tracks: &[Track]) -> Option<&Track> {
    tracks.iter().find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
}
