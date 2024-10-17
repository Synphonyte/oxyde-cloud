use crate::api_key::api_key;
use cliclack::log::remark;
use cliclack::{input, intro, outro, select, spinner};
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

pub async fn init(
    name: Option<String>,
    team_id: Option<i64>,
    config_file: PathBuf,
) -> Result<(), Error> {
    intro("Leptos Cloud app init")?;

    let team_id = match team_id {
        Some(team_id) => {
            remark(&format!("Team ID provided: {}", team_id))?;
            Some(team_id)
        }
        None => input_team_id().await?,
    };

    let name = match name {
        Some(name) => {
            remark(&format!("App name provided: {}", name))?;
            name
        }
        None => input_name(team_id).await?,
    };

    let config = CloudConfig {
        app: AppConfig { name, team_id },
        env: Default::default(),
        leptos_config: Default::default(),
    };

    let config_str = toml::to_string_pretty(&config)?;

    std::fs::write(&config_file, config_str)?;

    outro(&format!("Created config file: {}", config_file.display()))?;

    Ok(())
}

async fn input_team_id() -> Result<Option<i64>, Error> {
    let api_key = api_key()?;

    let spinner = spinner();
    spinner.start("Loading teams...");

    let client = Client::new(api_key.clone());

    let teams = client.teams().await?;

    if teams.is_empty() {
        spinner.stop("No teams found. Creating a personal app.");
        return Ok(None);
    }

    spinner.clear();

    let team_id = select("Select the team this app should belong to:")
        .item(None, "Personal", "Not part of a team")
        .items(
            &teams
                .into_iter()
                .map(|t| (Some(t.id), t.name, ""))
                .collect::<Vec<_>>(),
        )
        .interact()?;

    Ok(team_id)
}

async fn input_name(team_id: Option<i64>) -> Result<String, Error> {
    let api_key = api_key()?;

    loop {
        let name: String = input("Enter app name [a-z0-9_-]:")
            .placeholder("your-app-name-42")
            .validate_interactively(|input: &String| {
                if AppConfig::is_valid_name(input) {
                    Ok(())
                } else {
                    Err(format!("App name must be at least {} characters long, lower case alphanumeric and can contain underscores or dashes.", AppConfig::MIN_LENGTH))
                }
            })
            .interact()?;

        let spinner = spinner();
        spinner.start(format!(r#"Checking availability for name "{name}"..."#));

        let client = Client::new(api_key.clone());

        if client.check_name(&name, team_id).await? {
            spinner.stop("Name confirmed");
            return Ok(name);
        } else {
            spinner.error("App name is not available. Please try again.");
        }
    }
}
