use anyhow::{Context as AnyhowContext, Result};
use cliclack::log::remark;
use cliclack::{intro, outro};
use std::path::PathBuf;
use tera::Context;

mod app_slug;
mod team;

use crate::commands::deploy_config::init_deploy_config;
use crate::commands::init::app_slug::input_app_slug;
use crate::commands::init::team::input_team_slug;
use crate::commands::TEMPLATES;

pub async fn init(
    app_slug: Option<String>,
    team_slug: Option<String>,
    config_file: PathBuf,
) -> Result<()> {
    intro("Oxyde Cloud app init").context("Failed to show init intro")?;

    let team_slug = match team_slug {
        Some(team_slug) => {
            remark(format!("Team provided: {team_slug}"))
                .context("Failed to show team slug remark")?;
            team_slug
        }
        None => input_team_slug().await.context("Failed to get team slug")?,
    };

    let app_slug = match app_slug {
        Some(slug) => {
            remark(format!("App slug provided: {slug}"))
                .context("Failed to show app slug remark")?;
            slug
        }
        None => input_app_slug(&team_slug).await.context("Failed to get app slug")?,
    };

    let mut context = Context::new();
    context.insert("app_slug", &app_slug);
    let config_str = TEMPLATES
        .render("oxyde-cloud.toml", &context)
        .context("Failed to render config template")?;

    std::fs::write(&config_file, config_str)
        .with_context(|| format!("Failed to write config file: {}", config_file.display()))?;

    outro(format!("Created config file: {}\n", config_file.display()))
        .context("Failed to show config creation message")?;

    init_deploy_config().context("Failed to initialize deploy config")?;

    Ok(())
}
