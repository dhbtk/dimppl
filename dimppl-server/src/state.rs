use axum::extract::FromRef;
use deadpool_diesel::postgres::Pool;
use crate::database::create_database_pool;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool
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
