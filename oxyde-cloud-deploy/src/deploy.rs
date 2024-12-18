use cargo_leptos::config::Opts;
use oxyde_cloud_client::{Client, ReqwestJsonError, UploadFileError};
use oxyde_cloud_common::config::CloudConfig;
use std::env::VarError;
use std::ffi::OsStr;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Build error: {0}")]
    Build(#[from] anyhow::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Check Name error: {0}")]
    CheckName(#[from] ReqwestJsonError),

    #[error("Upload error: {0}")]
    Upload(#[from] UploadFileError),

    #[error("Deployment done error: {0}")]
    Done(#[from] reqwest::Error),

    #[error("Error reading variable `LEPTOS_CLOUD_API_KEY`: {0}")]
    ApiKeyEnv(#[from] VarError),

    #[error("Config loading error: {0}")]
    Config(#[from] oxyde_cloud_common::config::Error),
}

pub async fn deploy_with_config_file(
    config: &PathBuf,
    cargo_leptos_opts: Opts,
) -> Result<(), Error> {
    let config = CloudConfig::load(&config).await?;
    deploy(&config, cargo_leptos_opts).await?;
    Ok(())
}

pub async fn deploy(config: &CloudConfig, cargo_leptos_opts: Opts) -> Result<(), Error> {
    crate::build::build(cargo_leptos_opts.clone()).await?;

    let target_dir = "target";
    let server_bin_dir = if cargo_leptos_opts.release {
        "release"
    } else {
        "debug"
    };
    let frontend_dir = "site";

    let api_key = std::env::var("LEPTOS_CLOUD_API_KEY")?;
    let client = Client::new(api_key.clone());

    let frontend_path = Path::new(target_dir).join(frontend_dir);
    let server_path = Path::new(target_dir).join(server_bin_dir);

    let mut files = recursive_files_from_dir(frontend_path);
    files.append(&mut server_files(server_path)?);

    log::debug!(target:"cargo_leptos", "Found files: {:#?}", files);

    log::info!(target:"cargo_leptos", "Deploying app {}", config.app.slug);

    if let Err(err) = deploy_inner(config, client, &mut files).await {
        log::error!(target:"cargo_leptos", "Deploy failed: {:?}", err);
        return Err(err);
    }

    log::info!(target:"cargo_leptos", "Deployed app to {}", config.deployed_url());

    Ok(())
}

async fn deploy_inner(
    config: &CloudConfig,
    client: Client,
    files: &mut Vec<PathBuf>,
) -> Result<(), Error> {
    for file in files {
        log::debug!(target:"cargo_leptos", "Uploading {}...", file.display());
        client.clone().upload_file(&config.app.slug, file).await?;
    }

    log::debug!(target:"cargo_leptos", "Deploying app...");
    client.upload_done(config).await?;

    Ok(())
}

fn recursive_files_from_dir(dir: impl AsRef<Path>) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok().map(|e| e.into_path()))
        .filter_map(|e| if e.is_file() { Some(e) } else { None })
        .collect()
}

fn server_files(dir: impl AsRef<Path>) -> std::io::Result<Vec<PathBuf>> {
    Ok(read_dir(dir)?
        .filter_map(|d| {
            d.ok().and_then(|e| {
                let path = e.path();
                if path.is_file() && path.extension() != Some(OsStr::new("d")) {
                    Some(path)
                } else {
                    None
                }
            })
        })
        .collect())
}
