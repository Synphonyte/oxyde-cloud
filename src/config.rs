use std::path::PathBuf;
use leptos_config::errors::LeptosConfigError;
use thiserror::Error;

#[derive(serde::Deserialize)]
pub struct CloudConfig {
    pub app_name: String,

    #[serde(skip, default)]
    pub leptos_config: leptos_config::ConfFile,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML parse error: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("Leptos config error: {0}")]
    Leptos(#[from] LeptosConfigError),
}

impl CloudConfig {
    pub fn load(path: &PathBuf) -> Result<CloudConfig, Error> {
        let contents = std::fs::read_to_string(path)?;
        let mut config = toml::from_str(&contents)?;

        config.leptos_config = leptos_config::get_configuration(Some("Cargo.toml"))?;

        Ok(config)
    }
}