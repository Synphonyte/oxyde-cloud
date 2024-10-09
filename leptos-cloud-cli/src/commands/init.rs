use crate::api_key::api_key;
use cliclack::log::remark;
use cliclack::{input, intro, outro, spinner};
use leptos_cloud_client::{Client, ReqwestJsonError};
use leptos_cloud_common::config::{AppConfig, CloudConfig};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Keyring error: {0}")]
    Keyring(#[from] keyring::Error),

    #[error("Check Name error: {0}")]
    CheckName(#[from] ReqwestJsonError),

    #[error("TOML error: {0}")]
    Toml(#[from] toml::ser::Error),
}

pub async fn init(name: Option<String>, config_file: PathBuf) -> Result<(), Error> {
    intro("Leptos Cloud app init")?;

    let name = match name {
        Some(name) => {
            remark(&format!("App name provided: {}", name))?;
            name
        }
        None => input_name().await?,
    };

    let config = CloudConfig {
        app: AppConfig { name },
        env: Default::default(),
        leptos_config: Default::default(),
    };

    let config_str = toml::to_string_pretty(&config)?;

    std::fs::write(&config_file, config_str)?;

    outro(&format!("Created config file: {}", config_file.display()))?;

    Ok(())
}

async fn input_name() -> Result<String, Error> {
    let api_key = api_key()?;

    loop {
        let name: String = input("Enter app name:")
            .placeholder("yourappname")
            .validate_interactively(|input: &String| {
                if AppConfig::is_valid_name(input) {
                    Ok(())
                } else {
                    Err("App name must be lower case alphanumeric".to_string())
                }
            })
            .interact()?;

        let spinner = spinner();
        spinner.start(format!(r#"Checking availability for name "{name}"..."#));

        let client = Client::new(api_key.clone());

        if client.check_name(&name).await? {
            spinner.stop("Name confirmed");
            return Ok(name);
        } else {
            spinner.error("App name is not available. Please try again.");
        }
    }
}
