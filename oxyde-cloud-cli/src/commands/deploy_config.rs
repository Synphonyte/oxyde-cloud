use crate::commands::TEMPLATES;
use cliclack::{intro, outro, select};
use std::fmt::Formatter;
use std::fs::create_dir_all;
use tera::Context;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Keyring error: {0}")]
    Keyring(#[from] keyring::Error),

    #[error("TOML error: {0}")]
    Toml(#[from] toml::ser::Error),

    #[error("Tera error: {0}")]
    Tera(#[from] tera::Error),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DeployConfig {
    None,
    GitHub,
    // TODO : GitLab, ...
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum RustToolchain {
    Stable,
    Nightly,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Sqlx {
    Postgres,
    None,
}

impl std::fmt::Display for RustToolchain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RustToolchain::Stable => write!(f, "stable"),
            RustToolchain::Nightly => write!(f, "nightly"),
        }
    }
}

impl std::fmt::Display for Sqlx {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Sqlx::None => write!(f, "none"),
            Sqlx::Postgres => write!(f, "postgres"),
        }
    }
}

pub fn init_deploy_config() -> Result<(), Error> {
    intro("Oxyde Cloud deployment init")?;

    let deploy_config = select("How do you want to deploy?")
        .item(DeployConfig::GitHub, "GitHub Workflow", "")
        .item(DeployConfig::None, "None", "Setup deployment later")
        .initial_value(DeployConfig::GitHub)
        .interact()?;

    match deploy_config {
        DeployConfig::GitHub => {
            let toolchain = select_rust_toolchain()?;
            let sqlx = select_sqlx()?;

            let mut context = Context::new();
            context.insert("toolchain", &toolchain.to_string());
            context.insert("sqlx", &sqlx.to_string());
            let config_str = TEMPLATES.render("github-workflow.yml", &context)?;

            create_dir_all(".github/workflows")?;
            std::fs::write(".github/workflows/oxyde-cloud-deploy.yml", config_str)?;

            outro("Created .github/workflows/oxyde-cloud-deploy.yml")?;
        }
        DeployConfig::None => {
            outro("You can setup deployment yourself manually or call `leco deploy-config` at any time")?;
        }
    }

    Ok(())
}

fn select_rust_toolchain() -> Result<RustToolchain, Error> {
    // TODO : check for rust-toolchain.toml

    let toolchain = select("Select a Rust toolchain:")
        .item(RustToolchain::Stable, "stable", "")
        .item(RustToolchain::Nightly, "nightly", "")
        .initial_value(RustToolchain::Stable)
        .interact()?;

    Ok(toolchain)
}
fn select_sqlx() -> Result<Sqlx, Error> {
    let sqlx = select("Select your sqlx database server:")
        .item(Sqlx::None, "none", "")
        .item(Sqlx::Postgres, "postgres", "")
        .initial_value(Sqlx::None)
        .interact()?;

    Ok(sqlx)
}
