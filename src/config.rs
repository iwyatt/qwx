use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use anyhow::{Result, Context};
use directories::ProjectDirs;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub default_location: Option<String>,
    pub last_location: Option<String>,
    pub current_format: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        if !config_path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file at {:?}", config_path))?;
        
        let config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file at {:?}", config_path))?;
        
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory at {:?}", parent))?;
        }

        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config to TOML")?;
        
        fs::write(&config_path, content)
            .with_context(|| format!("Failed to write config file at {:?}", config_path))?;
        
        Ok(())
    }

    fn get_config_path() -> Result<PathBuf> {
        if let Ok(path) = std::env::var("QWX_CONFIG") {
            return Ok(PathBuf::from(path));
        }

        let proj_dirs = ProjectDirs::from("", "", "qwx")
            .context("Failed to determine project directories")?;
        
        Ok(proj_dirs.config_dir().join("config.toml"))
    }
}
