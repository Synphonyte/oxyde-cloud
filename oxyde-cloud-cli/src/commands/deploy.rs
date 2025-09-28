use anyhow::{Context, Result};
use cargo_leptos::config::Opts;
use cliclack::{intro, log::remark, outro};
use oxyde_cloud_common::config::CloudConfig;
use std::path::PathBuf;

#[cfg(feature = "with-deploy-test")]
pub async fn deploy(config: PathBuf) -> Result<()> {
    intro("Deploy to Oxyde Cloud").context("Failed to show deploy intro")?;

    // Load and validate config
    let cloud_config = CloudConfig::load(&config)
        .await
        .context("Failed to load config file")?;

    // Use default cargo-leptos options if none provided
    let opts = Opts {
        release: true,
        ..Default::default()
    };

    remark("Building and deploying..").context("Failed to show API key instructions")?;

    oxyde_cloud_deploy::deploy_with_config_file(&config, opts)
        .await
        .context("Failed to deploy")?;
    outro(format!(
        "Your app '{}' has been deployed",
        cloud_config.app.slug,
    ))
    .context("Failed to show deployment success message")?;

    Ok(())
}
