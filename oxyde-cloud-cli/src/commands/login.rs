use crate::api_key::api_key_entry;
use crate::commands::logout;
use crate::commands::logout::logout;
use cliclack::log::remark;
use cliclack::{input, intro, outro, outro_cancel, spinner};
use oxyde_cloud_client::Client;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Keyring error: {0}")]
    Keyring(#[from] keyring::Error),
    #[error("Logout error: {0}")]
    Logout(#[from] logout::Error),
}

pub async fn login() -> Result<(), Error> {
    intro("Login")?;

    let keyring_entry = api_key_entry()?;

    if keyring_entry.get_password().is_ok() {
        outro("You're already logged in.")?;
        return Ok(());
    }

    remark("Get your API-Key from https://oxyde.cloud/dashboard/profile/api-key")?;

    let api_key: String = input("Paste your API key")
        .placeholder("ABCD-efgh-IJKL-mnop")
        .interact()?;

    let api_key = api_key.trim().to_string();

    keyring_entry.set_password(&api_key)?;

    let spinner = spinner();
    spinner.start("Logging in...");

    match Client::new(api_key).login().await {
        Ok(login_result) => {
            spinner.stop("Done!");
            outro(format!(
                "You're now logged in as {}.",
                login_result.username
            ))?;
        }
        Err(err) => {
            logout()?;

            spinner.error("Failed!");
            outro_cancel(format!("Failed to login: {err}"))?;
        }
    }

    Ok(())
}
