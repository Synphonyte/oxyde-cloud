use thiserror::Error;
use crate::api_key::api_key_entry;
use crate::commands::build;
use crate::config::CloudConfig;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Build error: {0}")]
    Build(#[from] build::Error),

    #[error("Keyring error: {0}")]
    Keyring(#[from] keyring::Error),
}

pub async fn deploy(config: &CloudConfig) -> Result<(), Error> {
    build::build(config)?;

    // TODO

    let api_key = api_key_entry()?;

    Ok(())
}