use crate::api_key::api_key;
use cliclack::log::remark;
use cliclack::{input, intro, outro, select, spinner};
use heck::ToTitleCase;
use lazy_static::lazy_static;
use leptos_cloud_client::{Client, ReqwestJsonError};
use leptos_cloud_common::config::AppConfig;
use std::path::PathBuf;
use tera::{Context, Tera};
use thiserror::Error;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();

        if let Err(e) = tera.add_raw_template(
            "leptos-cloud.toml",
            include_str!("../../templates/leptos-cloud.toml"),
        ) {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }

        tera
    };
}

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
}

pub async fn init(
    app_slug: Option<String>,
    team_slug: Option<String>,
    config_file: PathBuf,
) -> Result<(), Error> {
    intro("Leptos Cloud app init")?;

    let team_slug = match team_slug {
        Some(team_slug) => {
            remark(&format!("Team provided: {}", team_slug))?;
            team_slug
        }
        None => input_team_slug().await?,
    };

    let app_slug = match app_slug {
        Some(slug) => {
            remark(&format!("App slug provided: {}", slug))?;
            slug
        }
        None => input_app_slug(&team_slug).await?,
    };

    let mut context = Context::new();
    context.insert("app_slug", &app_slug);
    let config_str = TEMPLATES.render("leptos-cloud.toml", &context)?;

    std::fs::write(&config_file, config_str)?;

    outro(&format!("Created config file: {}", config_file.display()))?;

    Ok(())
}

async fn input_team_slug() -> Result<String, Error> {
    let api_key = api_key()?;

    let spinner = spinner();
    spinner.start("Loading teams...");

    let client = Client::new(api_key.clone());

    let teams = client.teams().await?;

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
        .interact()?;

    Ok(team_slug)
}

async fn input_new_team(api_key: String) -> Result<String, Error> {
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
            .interact()?;

        let spinner = spinner();
        spinner.start(format!(
            r#"Checking availability for team slug "{team_slug}"..."#
        ));

        let client = Client::new(api_key.clone());

        if client.new_team(&team_slug).await? {
            spinner.stop("Slug confirmed");

            input_new_team_name(&team_slug, api_key).await?;

            return Ok(team_slug);
        } else {
            spinner.error(format!(
                r#"Team slug "{team_slug}" is not available. Please try another one."#
            ));
        }
    }
}

async fn input_new_team_name(team_slug: &str, api_key: String) -> Result<(), Error> {
    let default_name = team_slug.to_title_case();

    let mut name: String = input("Enter team display name:")
        .default_input(&default_name)
        .interact()?;

    if name.is_empty() {
        name = default_name;
    }

    let spinner = spinner();
    spinner.start(format!(r#"Saving team name "{name}"..."#));

    let client = Client::new(api_key);

    client.set_team_name(team_slug, &name).await?;

    spinner.stop("Saved.");

    Ok(())
}

async fn input_app_slug(team_slug: &str) -> Result<String, Error> {
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
