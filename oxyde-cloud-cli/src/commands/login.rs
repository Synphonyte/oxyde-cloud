use crate::api_key::api_key_entry;
use crate::commands::logout::logout;
use cliclack::log::remark;
use cliclack::{input, intro, outro, outro_cancel, spinner};
use oxyde_cloud_client::Client;
use anyhow::{Context, Result};

pub async fn login() -> Result<()> {
    intro("Login").context("Failed to show login intro")?;

    let keyring_entry = api_key_entry().context("Failed to access keyring")?;

    if keyring_entry.get_password().is_ok() {
        outro("You're already logged in.").context("Failed to show outro message")?;
        return Ok(());
    }

    remark("Get your API-Key from https://oxyde.cloud/dashboard/profile/api-key")
        .context("Failed to show API key instructions")?;

    let api_key: String = input("Paste your API key")
        .placeholder("ABCD-efgh-IJKL-mnop")
        .interact()
        .context("Failed to get API key input")?;

    let api_key = api_key.trim().to_string();

    keyring_entry.set_password(&api_key)
        .context("Failed to store API key in keyring")?;

    let spinner = spinner();
    spinner.start("Logging in...");

    match Client::new(api_key).login().await {
        Ok(login_result) => {
            spinner.stop("Done!");
            outro(format!(
                "You're now logged in as {}.",
                login_result.username
            )).context("Failed to show login success message")?;
        }
        Err(err) => {
            logout().context("Failed to logout after login error")?;

            spinner.error("Failed!");
            outro_cancel(format!("Failed to login: {err}"))
                .context("Failed to show login error message")?;
        }
    }

    Ok(())
}
