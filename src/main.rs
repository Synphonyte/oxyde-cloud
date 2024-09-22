mod commands;
mod client;
mod config;
mod api_key;

use std::path::PathBuf;
use reqwest::multipart;
use clap::{Parser, Subcommand};
use thiserror::Error;
use client::upload::upload_dir;
use crate::config::CloudConfig;

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
    },
    /// Build the project locally
    Build {
        /// Sets a custom config file. Defaults to `leptos-cloud.toml`
        #[arg(short, long, value_name = "FILE", default_value = "leptos-cloud.toml")]
        config: PathBuf,

    },
    /// Build the project locally and deploy it to the cloud
    Deploy {
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
    #[error("Build error: {0}")]
    Build(#[from] commands::build::Error),
    #[error("Deploy error: {0}")]
    Deploy(#[from] commands::deploy::Error),
    #[error("Config loading error: {0}")]
    Config(#[from] config::Error),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    match args.command {
        Commands::Login => {
            commands::login::login().await?;
        }
        Commands::Logout => {
            commands::logout::logout()?;
        }
        Commands::Init { name } => {
            commands::init::init(name)?;
        }
        Commands::Build { config } => {
            let config = CloudConfig::load(&config)?;
            commands::build::build(&config)?;
        }
        Commands::Deploy { config } => {
            let config = CloudConfig::load(&config)?;
            commands::deploy::deploy(&config).await?;
        }
    }

    let mut form = multipart::Form::new();

    upload_dir("test", &mut form).await?;

    Ok(())
}
