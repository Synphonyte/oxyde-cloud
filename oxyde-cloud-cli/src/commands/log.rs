use crate::api_key::api_key;
use anyhow::{Context, Result};
use oxyde_cloud_client::Client;

pub async fn log(name: &str) -> Result<()> {
    let api_key = api_key().context("Failed to get API key")?;
    let client = Client::new(api_key);

    let logs = client
        .log(name)
        .await
        .with_context(|| format!("Failed to fetch logs for app '{name}'"))?;

    println!("{logs}");

    Ok(())
}
