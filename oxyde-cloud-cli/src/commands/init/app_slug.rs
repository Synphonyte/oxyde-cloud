use crate::api_key::api_key;
use crate::commands::init::Error;
use cliclack::{input, spinner};
use oxyde_cloud_client::Client;
use oxyde_cloud_common::config::AppConfig;

pub(super) async fn input_app_slug(team_slug: &str) -> Result<String, Error> {
    let api_key = api_key()?;

    loop {
        let app_slug: String = input("Enter app slug [a-z0-9_-]:")
            .placeholder("your-app-name-42")
            .validate_interactively(|input: &String| {
                if AppConfig::is_valid_slug(input) {
                    Ok(())
                } else {
                    Err(format!("App slug must be at least {} characters long, lower case alphanumeric and can contain underscores or dashes.", AppConfig::MIN_SLUG_LENGTH))
                }
            })
            .interact()?;

        let spinner = spinner();
        spinner.start(format!(r#"Checking availability for slug "{app_slug}"..."#));

        let client = Client::new(api_key.clone());

        if client.new_app(&app_slug, team_slug).await? {
            spinner.stop("Slug confirmed");
            return Ok(app_slug);
        } else {
            spinner.error(format!(
                r#"App slug "{app_slug}" is not available. Please try another one."#
            ));
        }
    }
}
