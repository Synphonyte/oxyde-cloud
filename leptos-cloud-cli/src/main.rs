mod api_key;
mod commands;

use clap::{Parser, Subcommand};
use leptos_cloud_common::config::CloudConfig;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Login to the cloud
    Login,
    /// Logout from the cloud
    Logout,

    /// Initialize the necessary files for the project to be deployed to the cloud
    Init {
        /// The name of the project to deploy. Has to be unique within the cloud.
        #[arg(short, long)]
        name: Option<String>,

        /// The identifier name of the team this app should belong to. If none is provided,
        /// the app will belong to the user exclusively.
        #[arg(short, long)]
        team_slug: Option<String>,

        /// Sets a custom config file. Defaults to `leptos-cloud.toml`
        #[arg(short, long, value_name = "FILE", default_value = "leptos-cloud.toml")]
        config: PathBuf,
    },

    /// Configure how the project should be deployed to the cloud
    DeployConfig,

    Log {
        /// The name of the project to get the logs for. Defaults to the name from the config in
        /// the current directory
        #[arg(short, long)]
        name: Option<String>,

        /// Sets a custom config file. Defaults to `leptos-cloud.toml`
        #[arg(short, long, value_name = "FILE", default_value = "leptos-cloud.toml")]
        config: PathBuf,
    },
}

#[derive(Debug, Error)]
enum Error {
    #[error("Login error: {0}")]
    Login(#[from] commands::login::Error),
    #[error("Logout error: {0}")]
    Logout(#[from] commands::logout::Error),
    #[error("Init error: {0}")]
    Init(#[from] commands::init::Error),
    #[error("DeployConfig error: {0}")]
    DeployConfig(#[from] commands::deploy_config::Error),
    #[error("Config loading error: {0}")]
    Config(#[from] leptos_cloud_common::config::Error),
    #[error("Log error: {0}")]
    Name(String),
    #[error("Log error: {0}")]
    Log(#[from] commands::log::Error),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    simple_logger::init_with_level(log::Level::Info).unwrap();

    match args.command {
        Commands::Login => {
            commands::login::login().await?;
        }
        Commands::Logout => {
            commands::logout::logout()?;
        }
        Commands::Init {
            name,
            team_slug,
            config,
        } => {
            commands::init::init(name, team_slug, config).await?;
        }
        Commands::DeployConfig => {
            commands::deploy_config::init_deploy_config()?;
        }
        Commands::Log { name, config } => {
            let config = CloudConfig::load(&config).await.ok();

            let name = if let Some(name) = name {
                name
            } else if let Some(config) = config {
                config.app.slug
            } else {
                return Err(Error::Name("If you don't execute this command in a folder with a config you have to provide an app name!".to_string()));
            };

            commands::log::log(&name).await?;
        }
    }

    Ok(())
}
