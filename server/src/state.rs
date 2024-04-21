use crate::database::{create_database_pool, Pool};
use crate::sync_lock::SyncLock;
use axum::extract::FromRef;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
    pub sync_lock: SyncLock,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            pool: create_database_pool(),
            sync_lock: SyncLock::default(),
        }
    }
}

impl FromRef<AppState> for Pool {
    fn from_ref(input: &AppState) -> Self {
        input.pool.clone()
    }
}
