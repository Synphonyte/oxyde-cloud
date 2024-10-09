use leptos_config::errors::LeptosConfigError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;
use toml::Table;

#[derive(Serialize, Deserialize, Debug)]
pub struct CloudConfig {
    pub app: AppConfig,

    pub env: Table,

    #[serde(skip, default)]
    pub leptos_config: leptos_config::ConfFile,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub name: String,
}

impl AppConfig {
    pub fn is_valid_name(name: impl AsRef<str>) -> bool {
        name.as_ref().chars().all(|c| c.is_alphanumeric() && c.is_lowercase())
    }
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
    pub async fn load(path: &PathBuf) -> Result<Self, Error> {
        let contents = std::fs::read_to_string(path)?;
        let mut config: Self = toml::from_str(&contents)?;

        config.leptos_config = leptos_config::get_configuration(Some("Cargo.toml")).await?;

        Ok(config)
    }

    pub fn deployed_url(&self) -> String {
        format!("https://{}.leptos.app", self.app.name)
    }
}
