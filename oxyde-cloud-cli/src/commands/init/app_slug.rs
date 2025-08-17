use crate::api_key::api_key;
use anyhow::{Context, Result};
use cliclack::{input, spinner};
use oxyde_cloud_client::Client;
use oxyde_cloud_common::config::AppConfig;

pub(super) async fn input_app_slug(team_slug: &str) -> Result<String> {
    let api_key = api_key().context("Failed to get API key")?;

    loop {
        let app_slug: String = input("Enter app slug [a-z0-9_-]:")
            .placeholder("your-app-name-42")
            .validate_interactively(|input: &String| {
                if AppConfig::is_valid_slug(input) {
                    Ok(())
                } else {
                    Err(AppConfig::slug_requirements())
                }
            })
            .interact()
            .context("Failed to get app slug input")?;

        let spinner = spinner();
        spinner.start(format!(r#"Checking availability for slug "{app_slug}"..."#));

        let client = Client::new(api_key.clone());

        if client
            .new_app(&app_slug, team_slug)
            .await
            .with_context(|| format!("Failed to check app slug availability: {app_slug}"))?
        {
            spinner.stop("Slug confirmed");
            return Ok(app_slug);
        } else {
            spinner.error(format!(
                r#"App slug "{app_slug}" is not available. Please try another one."#
            ));
        }
    }
}
