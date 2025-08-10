use crate::api_key::api_key;
use anyhow::{Context, Result};
use cliclack::{input, select, spinner};
use heck::ToTitleCase;
use oxyde_cloud_client::Client;
use oxyde_cloud_common::config::AppConfig;

pub(super) async fn input_team_slug() -> Result<String> {
    let api_key = api_key().context("Failed to get API key")?;

    let spinner = spinner();
    spinner.start("Loading teams...");

    let client = Client::new(api_key.clone());

    let teams = client
        .teams()
        .await
        .context("Failed to fetch teams from API")?;

    if teams.is_empty() {
        spinner.stop("No teams found.");
        return input_new_team(api_key).await;
    }

    if teams.len() == 1 {
        spinner.stop(format!("Using team: {}", teams[0].name));
        return Ok(teams[0].slug.clone());
    }

    spinner.clear();

    let team_slug = select("Select the team this app should belong to:")
        .items(
            &teams
                .into_iter()
                .map(|t| (t.slug, t.name, ""))
                .collect::<Vec<_>>(),
        )
        .interact()
        .context("Failed to get team selection")?;

    Ok(team_slug)
}

async fn input_new_team(api_key: String) -> Result<String> {
    loop {
        let team_slug: String = input("Creating new team. Enter unique team slug [a-z0-9_-]:")
            .placeholder("your-team-name-42")
            .validate_interactively(|input: &String| {
                if AppConfig::is_valid_slug(input) {
                    Ok(())
                } else {
                    Err(format!("Team slug must be at least {} characters long, lower case alphanumeric and can contain underscores or dashes.", AppConfig::MIN_SLUG_LENGTH))
                }
            })
            .interact()
            .context("Failed to get team slug input")?;

        let spinner = spinner();
        spinner.start(format!(
            r#"Checking availability for team slug "{team_slug}"..."#
        ));

        let client = Client::new(api_key.clone());

        if client
            .new_team(&team_slug)
            .await
            .with_context(|| format!("Failed to check team slug availability: {team_slug}"))?
        {
            spinner.stop("Slug confirmed");

            input_new_team_name(&team_slug, api_key)
                .await
                .context("Failed to set team name")?;

            return Ok(team_slug);
        } else {
            spinner.error(format!(
                r#"Team slug "{team_slug}" is not available. Please try another one."#
            ));
        }
    }
}

async fn input_new_team_name(team_slug: &str, api_key: String) -> Result<()> {
    let default_name = team_slug.to_title_case();

    let mut name: String = input("Enter team display name:")
        .default_input(&default_name)
        .interact()
        .context("Failed to get team name input")?;

    if name.is_empty() {
        name = default_name;
    }

    let spinner = spinner();
    spinner.start(format!(r#"Saving team name "{name}"..."#));

    let client = Client::new(api_key);

    client
        .set_team_name(team_slug, &name)
        .await
        .with_context(|| format!("Failed to save team name: {name}"))?;

    spinner.stop("Saved.");

    Ok(())
}
