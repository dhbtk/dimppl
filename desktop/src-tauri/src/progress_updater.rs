use crate::environment::API_URL;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;
use anyhow::anyhow;
use chrono::{NaiveDateTime, TimeDelta, Utc};
use diesel::row::NamedRow;
use reqwest::{Client, ClientBuilder};
use tauri::{AppHandle, Manager, State};
use dimppl_shared::progress::ProgressUpdateRequest;
use crate::config::ConfigWrapper;
use crate::errors::AppResult;

const PROGRESS_UPDATE_INTERVAL: TimeDelta = TimeDelta::seconds(5);

#[derive(Clone)]
pub struct ProgressUpdater {
    app_handle: AppHandle,
    client: Client,
    in_flight: Arc<Mutex<bool>>,
    last_update_at: Arc<RwLock<NaiveDateTime>>,
    last_update: Arc<Mutex<Option<ProgressUpdateRequest>>>
}

impl ProgressUpdater {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            client: ClientBuilder::new()
                .timeout(Duration::from_secs(5))
                .build().unwrap(),
            in_flight: Arc::new(Mutex::new(false)),
            last_update_at: Arc::new(RwLock::new(Utc::now().naive_utc())),
            last_update: Arc::new(Mutex::new(None))
        }
    }

    pub fn submit_progress(&self, progress: ProgressUpdateRequest) -> AppResult<()> {
        let mut in_flight = self.in_flight.lock().unwrap();
        if *in_flight {
            // request in progress
            return Ok(());
        }
        if !self.should_submit(&progress) {
            return Ok(());
        }
        *in_flight = true;
        tauri::async_runtime::spawn(self.clone().do_submit(progress));
        Ok(())
    }

    async fn do_submit(self, progress: ProgressUpdateRequest) -> AppResult<()> {
        let result = self.submit_inner(progress.clone()).await;
        if result.is_err() {
            tracing::info!("progress submit failed: {:?}", result);
        } else {
            tracing::info!("progress submit success");
            *self.last_update_at.write().unwrap() = Utc::now().naive_utc();
            *self.last_update.lock().unwrap() = Some(progress);
        }
        {
            let mut in_flight = self.in_flight.lock().unwrap();
            *in_flight = false;
        }
        Ok(())
    }

    async fn submit_inner(&self, progress: ProgressUpdateRequest) -> AppResult<()> {
        let token = {
            let config_wrapper: State<ConfigWrapper> = self.app_handle.state();
            let config = config_wrapper.0.lock().unwrap();
            config.access_token.clone()
        };
        let result = self.client
            .post(format!("{API_URL}/submit_progress"))
            .header("Authorization", format!("Bearer {token}"))
            .json(&progress)
            .send()
            .await?;
        if result.status().is_success() {
            return Ok(())
        }
        Err(anyhow!("Request failed with status {}: {}", result.status(), result.text().await?).into())
    }

    fn should_submit(&self, progress: &ProgressUpdateRequest) -> bool {
        let last_update = self.last_update.lock().unwrap().clone();
        if last_update.is_none() {
            return true;
        }
        let last_update_at = *self.last_update_at.read().unwrap();
        if Utc::now().naive_utc() - last_update_at > PROGRESS_UPDATE_INTERVAL {
            return true;
        }
        let last_update = last_update.unwrap();
        if last_update.episode_guid != progress.episode_guid {
            return true;
        }
        if last_update.episode_guid == progress.episode_guid && progress.completed && !last_update.completed {
            return true;
        }
        false
    }
}
