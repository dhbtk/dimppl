use std::fs::File;
use std::ops::Deref;
use std::path::Path;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread::JoinHandle;
use std::time::Duration;

use anyhow::anyhow;
use chrono::Utc;
use diesel::associations::HasTable;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use lofty::AudioFile;

use symphonia::core::audio::{AsAudioBufferRef, AudioBuffer, Signal};
use symphonia::core::codecs::CODEC_TYPE_NULL;
use symphonia::core::errors::Error;
use symphonia::core::formats::{FormatReader, SeekMode, SeekTo, Track};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::probe::Hint;
use symphonia::core::units::Time;
use tauri::{AppHandle, Manager};
use tokio::time::Instant;

use crate::database::db_connect;
use crate::errors::AppResult;
use crate::frontend_change_tracking::{AppHandleExt, EntityChange};
use crate::models::{podcast, Episode, EpisodeProgress, Podcast};
use crate::player::{output, PlayerStatus};

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
        }
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
        self.played_millis
            .store((starting_at as i64) * 1000, Ordering::Relaxed);
        *self.is_paused.write().unwrap() = false;
        self.broadcast_status_self();
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
        let probed = symphonia::default::get_probe().format(
            &hint,
            mss,
            &Default::default(),
            &Default::default(),
        )?;
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
        self.broadcast_status_self();
    }

    pub fn pause(&self) {
        if self.playing_episode.read().unwrap().is_none() {
            return;
        }
        if let Some(channel) = self.sender_channel.lock().unwrap().deref() {
            let _ = channel.send(PlayerCommand::Pause);
        }
        *self.is_paused.write().unwrap() = true;
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
        if let Some(channel) = self.sender_channel.lock().unwrap().deref() {
            let _ = channel.send(PlayerCommand::Seek(seconds.unsigned_abs()));
        }
    }

    pub fn set_volume(&self, volume: f32) {
        *self.volume.write().unwrap() = volume;
    }

    pub fn set_playback_speed(&self, speed: f32) {
        *self.playback_speed.write().unwrap() = speed;
    }

    fn broadcast_status_self(&self) {
        let episode = self.playing_episode.read().unwrap();
        let elapsed = self.played_millis.load(Ordering::Relaxed);
        Self::broadcast_status(
            &self.app_handle,
            episode.clone(),
            *self.is_paused.read().unwrap(),
            elapsed,
            *self.episode_length.read().unwrap(),
            false,
            false,
        );
    }

    fn broadcast_status(
        app_handle: &AppHandle,
        episode_container: Option<(Episode, Podcast)>,
        paused: bool,
        elapsed: i64,
        duration: i64,
        loading: bool,
        save_progress: bool,
    ) {
        if save_progress && episode_container.is_some() {
            use crate::schema::episode_progresses::dsl::*;
            let (episode, _podcast) = episode_container.clone().unwrap();
            let elapsed_seconds = elapsed / 1000;
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
        let _ = app_handle.emit_all(
            "player-status",
            PlayerStatus {
                is_paused: paused,
                episode: episode_container.as_ref().map(|(ep, _)| ep.clone()),
                podcast: episode_container.map(|(_, podcast)| podcast),
                elapsed: elapsed / 1000,
                duration,
                loading,
            },
        );
    }

    fn play_track(&self, mut reader: Box<dyn FormatReader>, starting_at: i32) -> AppResult<()> {
        let track = first_supported_track(reader.tracks());
        let mut track_id = match track {
            Some(track) => track.id,
            _ => return Ok(()),
        };
        let mut seek_ts = if starting_at != 0 {
            let seek_to = SeekTo::Time {
                time: Time::from(starting_at.unsigned_abs()),
                track_id: Some(track_id),
            };

            // Attempt the seek. If the seek fails, ignore the error and return a seek timestamp of 0 so
            // that no samples are trimmed.
            match reader.seek(SeekMode::Accurate, seek_to) {
                Ok(seeked_to) => seeked_to.required_ts,
                Err(Error::ResetRequired) => {
                    track_id = first_supported_track(reader.tracks()).unwrap().id;
                    0
                }
                Err(err) => {
                    // Don't give-up on a seek error.
                    tracing::warn!("seek error: {}", err);
                    0
                }
            }
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

        let cloned_atomic = self.played_millis.clone();
        let cloned_handle = self.app_handle.clone();
        let cloned_episode = self.playing_episode.clone();
        let cloned_duration = self.episode_length.clone();
        let cloned_pause = self.is_paused.clone();
        let cloned_volume = self.volume.clone();

        let handle = std::thread::Builder::new()
            .name("new player thread".into())
            .spawn(move || {
                let mut save_timer = Instant::now();
                let mut update_timer = Instant::now();
                let mut is_paused = false;
                let track = match reader.tracks().iter().find(|track| track.id == track_id) {
                    Some(track) => track,
                    _ => return Ok(()),
                };

                // Create a decoder for the track.
                let mut decoder = symphonia::default::get_codecs()
                    .make(&track.codec_params, &Default::default())?;

                // Get the selected track's timebase and duration.
                let tb = track.codec_params.time_base;

                let mut audio_output: Option<Box<dyn output::AudioOutput>> = None;
                let mut sample_buffer: Option<AudioBuffer<f32>> = None;
                // Decode and play the packets belonging to the selected track.
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
                                break Ok(());
                            }
                        }
                    }
                    if is_paused {
                        std::thread::sleep(Duration::from_millis(10));
                        continue;
                    }

                    // Get the next packet from the format reader.
                    let packet = match reader.next_packet() {
                        Ok(packet) => packet,
                        Err(err) => break Err(err),
                    };

                    // If the packet does not belong to the selected track, skip it.
                    if packet.track_id() != track_id {
                        continue;
                    }

                    // Decode the packet into audio samples.
                    match decoder.decode(&packet) {
                        Ok(decoded) => {
                            // If the audio output is not open, try to open it.
                            if audio_output.is_none() {
                                // Get the audio buffer specification. This is a description of the decoded
                                // audio buffer's sample format and sample rate.
                                let spec = *decoded.spec();

                                // Get the capacity of the decoded buffer. Note that this is capacity, not
                                // length! The capacity of the decoded buffer is constant for the life of the
                                // decoder, but the length is not.
                                let duration = decoded.capacity() as u64;

                                // Try to open the audio output.
                                audio_output.replace(output::try_open(spec, duration).unwrap());
                            } else {
                                // TODO: Check the audio spec. and duration hasn't changed.
                            }

                            if sample_buffer.is_none() {
                                sample_buffer = Some(AudioBuffer::<f32>::new(
                                    decoded.capacity() as u64,
                                    *decoded.spec(),
                                ));
                            }
                            if let Some(buffer) = &mut sample_buffer {
                                // Write the decoded audio samples to the audio output if the presentation timestamp
                                // for the packet is >= the seeked position (0 if not seeking).
                                if packet.ts() >= seek_ts {
                                    if let Some(time_base) = tb {
                                        let time = time_base.calc_time(packet.ts);
                                        cloned_atomic
                                            .store((time.seconds * 1000) as i64, Ordering::Relaxed);
                                    }
                                    if let Some(audio_output) = &mut audio_output {
                                        decoded.convert(buffer);
                                        let volume = *cloned_volume.read().unwrap();
                                        buffer.transform(|sample| sample * volume);
                                        audio_output.write(buffer.as_audio_buffer_ref()).unwrap()
                                    }
                                }
                            }
                            if save_timer.elapsed().as_millis() > 1000 {
                                save_timer = Instant::now();
                                NewPlayer::broadcast_status(
                                    &cloned_handle,
                                    cloned_episode.read().unwrap().clone(),
                                    *cloned_pause.read().unwrap(),
                                    cloned_atomic.load(Ordering::Relaxed),
                                    *cloned_duration.read().unwrap(),
                                    false,
                                    true,
                                );
                            } else if update_timer.elapsed().as_millis() > 100 {
                                update_timer = Instant::now();
                                NewPlayer::broadcast_status(
                                    &cloned_handle,
                                    cloned_episode.read().unwrap().clone(),
                                    *cloned_pause.read().unwrap(),
                                    cloned_atomic.load(Ordering::Relaxed),
                                    *cloned_duration.read().unwrap(),
                                    false,
                                    false,
                                );
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
                tracing::debug!("thread is now dying");
                Ok(())
            })
            .unwrap();
        *self.thread_handle.lock().unwrap() = Some(handle);
        Ok(())
    }
}

fn first_supported_track(tracks: &[Track]) -> Option<&Track> {
    tracks
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
}
