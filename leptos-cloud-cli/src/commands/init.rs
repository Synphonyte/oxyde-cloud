use cliclack::log::remark;
use cliclack::{input, intro, outro};
use leptos_cloud_common::config::{AppConfig, CloudConfig};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML error: {0}")]
    Toml(#[from] toml::ser::Error),
}

pub fn init(name: Option<String>, config_file: PathBuf) -> Result<(), Error> {
    intro("Leptos Cloud app init")?;

    let name = match name {
        Some(name) => {
            remark(&format!("App name provided: {}", name))?;
            name
        }
        None => input("Enter app name:").placeholder("my-app").interact()?,
    };

    let config = CloudConfig {
        app: AppConfig { name },
        env: Default::default(),
        leptos_config: Default::default(),
    };

    let config_str = toml::to_string_pretty(&config)?;

    std::fs::write(&config_file, config_str)?;

    outro(&format!("Created config file: {}", config_file.display()))?;

    // TODO
    Ok(())
}
