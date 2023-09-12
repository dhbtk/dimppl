use std::ffi::c_void;
use std::sync::Arc;

use serde::Serialize;
use tauri::AppHandle;

use crate::errors::AppResult;
use crate::models::{Episode, Podcast};
use crate::player::new_player::NewPlayer;

mod new_player;
pub mod output;
pub mod resampler;

#[allow(dead_code)]
pub struct Player {
    app_handle: AppHandle,
    new_player: Arc<NewPlayer>,
}

impl Player {
    pub fn new(app_handle: AppHandle) -> Self {
        let new_handle = app_handle.clone();
        Self {
            app_handle,
            new_player: Arc::new(NewPlayer::new(new_handle)),
        }
    }

    pub fn set_up_media_controls(&self, handle: Option<*mut c_void>) {
        self.new_player.set_up_media_controls(handle);
    }

    pub fn play_episode(&self, episode: Episode, starting_seconds: u64) -> AppResult<()> {
        self.new_player.play_episode(episode, starting_seconds as i32)
    }

    pub fn play(&self) {
        self.new_player.play();
    }

    pub fn pause(&self) {
        self.new_player.pause();
    }

    pub fn skip_forwards(&self) {
        self.new_player.skip_forwards();
    }

    pub fn skip_backwards(&self) {
        self.new_player.skip_backwards();
    }

    pub fn seek_to(&self, seconds: i64) {
        self.new_player.seek_to(seconds);
    }

    pub fn set_volume(&self, volume: f32) {
        self.new_player.set_volume(volume);
    }

    pub fn set_playback_speed(&self, speed: f32) {
        self.new_player.set_playback_speed(speed);
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStatus {
    pub is_paused: bool,
    pub episode: Option<Episode>,
    pub podcast: Option<Podcast>,
    pub elapsed: i64,
    pub duration: i64,
    pub loading: bool,
}
