use crate::deploy::deploy;
use cargo_leptos::config::Opts;
use clap::Parser;
use leptos_cloud_common::config::CloudConfig;
use std::path::PathBuf;
use thiserror::Error;

mod build;
mod deploy;

#[derive(Debug, Error)]
enum Error {
    #[error("Deploy error: {0}")]
    Deploy(#[from] deploy::Error),
    #[error("Config loading error: {0}")]
    Config(#[from] leptos_cloud_common::config::Error),
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Sets a custom config file. Defaults to `leptos-cloud.toml`
    #[arg(short, long, value_name = "FILE", default_value = "leptos-cloud.toml")]
    config: PathBuf,

    #[clap(flatten)]
    cargo_leptos_opts: Opts,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let Args {
        config,
        cargo_leptos_opts,
    } = Args::parse();

    let config = CloudConfig::load(&config).await?;
    deploy(&config, cargo_leptos_opts).await?;

    Ok(())
}
