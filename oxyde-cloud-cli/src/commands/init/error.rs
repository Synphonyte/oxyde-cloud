use oxyde_cloud_client::ReqwestJsonError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Keyring error: {0}")]
    Keyring(#[from] keyring::Error),

    #[error("Check Name error: {0}")]
    CheckName(#[from] ReqwestJsonError),

    #[error("TOML error: {0}")]
    Toml(#[from] toml::ser::Error),

    #[error("Tera error: {0}")]
    Tera(#[from] tera::Error),

    #[error("Deploy config error: {0}")]
    DeployConfig(#[from] crate::commands::deploy_config::Error),
}
