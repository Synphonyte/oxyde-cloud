use leptos_config::errors::LeptosConfigError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;
use toml::Table;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CloudConfig {
    pub app: AppConfig,

    pub env: Table,

    #[serde(skip, default)]
    pub leptos_config: leptos_config::ConfFile,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub slug: String,
}

impl AppConfig {
    pub const ALLOWED_CHARS: [char; 38] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        '-', '_',
    ];

    pub const MIN_SLUG_LENGTH: usize = 5;

    pub fn is_valid_slug(slug: impl AsRef<str>) -> bool {
        slug.as_ref().len() >= Self::MIN_SLUG_LENGTH
            && slug
                .as_ref()
                .chars()
                .next()
                .map(|c| c.is_ascii_lowercase())
                .unwrap_or(false)
            && slug
                .as_ref()
                .chars()
                .all(|c| Self::ALLOWED_CHARS.contains(&c))
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
        format!("https://{}.oxydecloud.com", self.app.slug)
    }
}
