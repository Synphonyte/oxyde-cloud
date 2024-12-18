use crate::api_key::api_key;
use oxyde_cloud_client::{Client, ReqwestJsonError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Reqwest error: {0}")]
    Api(#[from] ReqwestJsonError),
    #[error("Keyring error: {0}")]
    Keyring(#[from] keyring::Error),
}

pub async fn log(name: &str) -> Result<(), Error> {
    let api_key = api_key()?;
    let client = Client::new(api_key);

    println!("{}", client.log(name).await?);

    Ok(())
}
