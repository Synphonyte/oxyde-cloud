use anyhow::{Context, Result};

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use toml::Table;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CloudConfig {
    pub app: AppConfig,

    pub env: Table,

    #[serde(skip)]
    pub leptos_config: Option<leptos_config::ConfFile>,
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

    pub fn slug_requirements() -> String {
        format!("App slug must be at least {} characters long, lower case alphanumeric and can contain underscores or dashes.", Self::MIN_SLUG_LENGTH)
    }

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

// Error type replaced with anyhow::Result for better error handling

impl CloudConfig {
    pub async fn load(path: &PathBuf) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let mut config: Self = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse TOML config file: {}", path.display()))?;

        config.leptos_config = Some(
            leptos_config::get_configuration(Some("Cargo.toml"))
                .context("Failed to load Leptos configuration")?,
        );

        Ok(config)
    }

    pub fn deployed_url(&self) -> String {
        format!("https://{}.oxydecloud.com", self.app.slug)
    }
}
