use cliclack::log::remark;
use cliclack::{intro, outro};
use std::path::PathBuf;
use tera::Context;

mod app_slug;
mod error;
mod team;

use crate::commands::deploy_config::init_deploy_config;
use crate::commands::init::app_slug::input_app_slug;
use crate::commands::init::team::input_team_slug;
use crate::commands::TEMPLATES;

pub use error::*;

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

    outro(&format!("Created config file: {}\n", config_file.display()))?;

    init_deploy_config()?;

    Ok(())
}
