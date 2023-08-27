use crate::directories::project_dirs;
use crate::errors::AppResult;
use gethostname::gethostname;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub user_access_key: String,
    pub device_name: String,
    pub access_token: String,
}

impl Config {
    pub fn load() -> AppResult<Self> {
        let contents = fs::read_to_string(config_path())?;
        let config: Self = toml::from_str(contents.as_str())?;
        Ok(config)
    }

    pub fn load_or_save_default() -> AppResult<Self> {
        if let Ok(config) = Config::load() {
            return Ok(config);
        }
        let config = Config::default();
        config.save()?;
        Ok(config)
    }

    pub fn save(&self) -> AppResult<()> {
        let serialized = toml::to_string(self)?;
        fs::write(config_path(), serialized)?;
        Ok(())
    }
}

fn config_path() -> PathBuf {
    let directories = project_dirs();
    directories.data_dir().join("settings.toml")
}

impl Default for Config {
    fn default() -> Self {
        Self {
            user_access_key: "".into(),
            device_name: gethostname().into_string().unwrap(),
            access_token: "".into(),
        }
    }
}

pub struct ConfigWrapper(pub Mutex<Config>);

impl Default for ConfigWrapper {
    fn default() -> Self {
        Self(Mutex::new(
            Config::load_or_save_default().expect("error initializing app state"),
        ))
    }
}
