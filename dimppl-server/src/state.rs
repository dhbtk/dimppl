use axum::extract::FromRef;
use crate::database::{create_database_pool, Pool};

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            pool: create_database_pool()
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            pool: create_database_pool()
        }
    }
}

impl FromRef<AppState> for Pool {
    fn from_ref(input: &AppState) -> Self {
        input.pool.clone()
    }
}
