use crate::config::Config;
use crate::errors::AppResult;

pub struct AppState {
    pub config: Config,
}

impl AppState {
    pub fn new() -> AppResult<Self> {
        let config = Config::load_or_save_default()?;
        Ok(Self {
            config
        })
    }
}
