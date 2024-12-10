use crate::deploy::deploy_with_config_file;
use cargo_leptos::config::Opts;
use clap::Parser;
use leptos_cloud_common::config::CloudConfig;
use std::path::PathBuf;
use thiserror::Error;

mod build;
mod deploy;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Sets a custom config file. Defaults to `leptos-cloud.toml`
    #[arg(short, long, value_name = "FILE", default_value = "leptos-cloud.toml")]
    config: PathBuf,

    #[clap(flatten)]
    cargo_leptos_opts: Opts,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            config: PathBuf::from("leptos-cloud.toml"),
            cargo_leptos_opts: Opts::default(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let Args {
        config,
        cargo_leptos_opts,
    } = Args::parse();

    deploy_with_config_file(&config, cargo_leptos_opts).await?;

    Ok(())
}
