use anyhow::{Context, Result};
use cargo_leptos::config::Opts;
use oxyde_cloud_client::Client;
use oxyde_cloud_common::config::CloudConfig;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub async fn deploy_with_config_file(config: &PathBuf, cargo_leptos_opts: Opts) -> Result<()> {
    let config = CloudConfig::load(config)
        .await
        .with_context(|| format!("Failed to load config file: {}", config.display()))?;
    deploy(&config, cargo_leptos_opts)
        .await
        .context("Failed to deploy with loaded config")?;
    Ok(())
}

pub async fn deploy(config: &CloudConfig, cargo_leptos_opts: Opts) -> Result<()> {
    crate::build::build(cargo_leptos_opts.clone())
        .await
        .context("Failed to build project")?;

    let target_dir = "target";
    let target_bin_dir = std::env::var("OXYDE_CLOUD_BIN_DIR").unwrap_or_else(|_| "target/x86_64-unknown-linux-musl".to_string());

    let server_bin_dir = if cargo_leptos_opts.release {
        "release"
    } else {
        "debug"
    };
    let frontend_dir = "site";

    let api_key = std::env::var("OXYDE_CLOUD_API_KEY")
        .context("Environment variable OXYDE_CLOUD_API_KEY is required for deployment")?;
    let client = Client::new(api_key.clone());

    let frontend_path = Path::new(target_dir).join(frontend_dir);
    let server_path = Path::new(&target_bin_dir).join(server_bin_dir);

    let mut files = recursive_files_from_dir(frontend_path);
    files.append(&mut server_files(server_path).context("Failed to collect server files")?);

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
) -> Result<()> {
    for file in files {
        let file_path = file.display().to_string();
        log::debug!(target:"cargo_leptos", "Uploading {}...", file_path);
        client
            .clone()
            .upload_file(&config.app.slug, file)
            .await
            .with_context(|| format!("Failed to upload file: {file_path}"))?;
    }

    log::debug!(target:"cargo_leptos", "Deploying app...");
    client
        .upload_done(config)
        .await
        .context("Failed to signal deployment completion")?;

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
    let starts_with_a_dot = |path: &Path| {
        path.file_name()
            .expect("cant read filename")
            .to_str()
            .expect("cant convert filename")
            .starts_with(".")
    };

    Ok(read_dir(dir)?
        .filter_map(|d| {
            d.ok().and_then(|e| {
                let path = e.path();
                if path.is_file() && path.extension().is_none() && !starts_with_a_dot(&path) {
                    Some(path)
                } else {
                    None
                }
            })
        })
        .collect())
}
