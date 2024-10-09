use crate::api_key::api_key;
use crate::commands::build;
use cargo_leptos::config::Opts;
use cliclack::{intro, outro, progress_bar, ProgressBar};
use leptos_cloud_client::{Client, ReqwestJsonError, UploadFileError};
use leptos_cloud_common::config::CloudConfig;
use log::debug;
use std::ffi::OsStr;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Build error: {0}")]
    Build(#[from] anyhow::Error),

    #[error("Keyring error: {0}")]
    Keyring(#[from] keyring::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Check Name error: {0}")]
    CheckName(#[from] ReqwestJsonError),

    #[error("Upload error: {0}")]
    Upload(#[from] UploadFileError),

    #[error("Deployment done error: {0}")]
    Done(#[from] reqwest::Error),

    #[error("App name '{0}' not available")]
    Name(String),
}

pub async fn deploy(config: &CloudConfig, cargo_leptos_opts: Opts) -> Result<(), Error> {
    build::build(cargo_leptos_opts.clone()).await?;

    let target_dir = "target";
    let server_bin_dir = if cargo_leptos_opts.release {
        "release"
    } else {
        "debug"
    };
    let frontend_dir = "site";

    let api_key = api_key()?;
    let client = Client::new(api_key.clone());

    let frontend_path = Path::new(target_dir).join(frontend_dir);
    let server_path = Path::new(target_dir).join(server_bin_dir);

    let mut files = recursive_files_from_dir(frontend_path);
    files.append(&mut server_files(server_path)?);

    debug!("Found files: {:#?}", files);

    intro(format!("Deploying app {}", config.app.name))?;

    let file_count = files.len() as u64;

    let progress = progress_bar(file_count + 1);
    progress.start(format!(r#"Checking app name "{}"..."#, config.app.name));

    if let Err(err) = deploy_inner(&config, client, &mut files, &progress).await {
        progress.error(format!("Deploy failed: {:?}", err));
        return Err(err);
    }

    progress.stop("Deployed");
    outro(format!("App deployed to {}", config.deployed_url()))?;

    Ok(())
}

async fn deploy_inner(
    config: &&CloudConfig,
    client: Client,
    files: &mut Vec<PathBuf>,
    progress: &ProgressBar,
) -> Result<(), Error> {
    for file in files {
        progress.set_message(format!("Uploading {}...", file.display()));
        client.clone().upload_file(file).await?;
        progress.inc(1);
    }

    progress.set_message("Deploying app...");
    client.upload_done(config).await?;

    progress.inc(1);

    Ok(())
}

fn recursive_files_from_dir(dir: impl AsRef<Path>) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok().map(|e| e.into_path()))
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
