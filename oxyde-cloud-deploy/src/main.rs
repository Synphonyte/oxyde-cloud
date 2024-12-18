use crate::deploy::{deploy_with_config_file, Error};
use cargo_leptos::config::Opts;
use clap::Parser;
use std::path::PathBuf;

mod build;
mod deploy;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Sets a custom config file. Defaults to `oxyde-cloud.toml`
    #[arg(short, long, value_name = "FILE", default_value = "oxyde-cloud.toml")]
    config: PathBuf,

    #[clap(flatten)]
    cargo_leptos_opts: Opts,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            config: PathBuf::from("oxyde-cloud.toml"),
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
