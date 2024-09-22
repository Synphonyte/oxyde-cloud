use cliclack::{input, intro, note, outro, outro_cancel, spinner};
use thiserror::Error;
use crate::client::Client;
use keyring::{Entry};
use crate::api_key::api_key_entry;
use crate::commands::logout::logout;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Keyring error: {0}")]
    Keyring(#[from] keyring::Error),
}

pub async fn login() -> Result<(), Error> {
    intro("Login")?;


    let keyring_entry = api_key_entry()?;

    if keyring_entry.get_password().is_ok() {
        outro("You're already logged in.")?;
        return Ok(());
    }

    note("API-Key", "Get your API-Key from https://leptos.cloud/api-keys")?;

    let api_key: String = input("Paste your API key")
        .placeholder("ABCD-efgh-IJKL-mnop")
        .interact()?;

    keyring_entry.set_password(&api_key)?;

    let spinner = spinner();
    spinner.start("Logging in...");

    match Client::new(api_key).login().await {
        Ok(login_result) => {
            spinner.stop("Done!");
            outro(format!("You're now logged in as {}.", login_result.username))?;
        }
        Err(err) => {
            logout()?;

            spinner.error("Failed!");
            outro_cancel(format!("Failed to login: {err}"))?;
        }
    }

    Ok(())
}
