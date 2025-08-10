mod api_key;
mod commands;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use oxyde_cloud_common::config::CloudConfig;
use std::path::PathBuf;

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

        /// Sets a custom config file. Defaults to `oxyde-cloud.toml`
        #[arg(short, long, value_name = "FILE", default_value = "oxyde-cloud.toml")]
        config: PathBuf,
    },

    /// Configure how the project should be deployed to the cloud
    DeployConfig,

    Log {
        /// The name of the project to get the logs for. Defaults to the name from the config in
        /// the current directory
        #[arg(short, long)]
        name: Option<String>,

        /// Sets a custom config file. Defaults to `oxyde-cloud.toml`
        #[arg(short, long, value_name = "FILE", default_value = "oxyde-cloud.toml")]
        config: PathBuf,
    },
}

#[tokio::main]
async fn main() {
    if let Err(err) = error_wrapper().await {
        eprintln!("Error: {err:?}");
    }
}

async fn error_wrapper() -> Result<()> {
    let args = Args::parse();

    simple_logger::init_with_level(log::Level::Info).unwrap();

    match args.command {
        Commands::Login => {
            commands::login::login().await.context("Login failed")?;
        }
        Commands::Logout => {
            commands::logout::logout().context("Logout failed")?;
        }
        Commands::Init {
            name,
            team_slug,
            config,
        } => {
            commands::init::init(name, team_slug, config)
                .await
                .context("Init failed")?;
        }
        Commands::DeployConfig => {
            commands::deploy_config::init_deploy_config().context("Deploy config failed")?;
        }
        Commands::Log { name, config } => {
            let config = CloudConfig::load(&config).await.ok();

            let name = if let Some(name) = name {
                name
            } else if let Some(config) = config {
                config.app.slug
            } else {
                anyhow::bail!(
                    "If you don't execute this command in a folder with a config you have to provide an app name!"
                );
            };

            commands::log::log(&name)
                .await
                .context("Log command failed")?;
        }
    }

    Ok(())
}
