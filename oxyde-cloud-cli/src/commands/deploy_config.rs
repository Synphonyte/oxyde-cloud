use crate::commands::TEMPLATES;
use anyhow::{Context as AnyhowContext, Result};
use cliclack::{intro, outro, select};
use std::fmt::Formatter;
use std::fs::create_dir_all;
use tera::Context;

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

impl std::fmt::Display for RustToolchain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RustToolchain::Stable => write!(f, "stable"),
            RustToolchain::Nightly => write!(f, "nightly"),
        }
    }
}

pub fn init_deploy_config() -> Result<()> {
    intro("Oxyde Cloud deployment init").context("Failed to show deployment config intro")?;

    let deploy_config = select("How do you want to deploy?")
        .item(DeployConfig::GitHub, "GitHub Workflow", "")
        .item(DeployConfig::None, "None", "Setup deployment later")
        .initial_value(DeployConfig::GitHub)
        .interact()
        .context("Failed to get deployment config selection")?;

    match deploy_config {
        DeployConfig::GitHub => {
            let toolchain = select_rust_toolchain().context("Failed to select Rust toolchain")?;
            // let sqlx = select_sqlx()?;

            let mut context = Context::new();
            context.insert("toolchain", &toolchain.to_string());
            // context.insert("sqlx", &sqlx.to_string());
            let config_str = TEMPLATES
                .render("github-workflow.yml", &context)
                .context("Failed to render GitHub workflow template")?;

            create_dir_all(".github/workflows")
                .context("Failed to create .github/workflows directory")?;
            std::fs::write(".github/workflows/oxyde-cloud-deploy.yml", config_str)
                .context("Failed to write GitHub workflow file")?;

            outro("Created .github/workflows/oxyde-cloud-deploy.yml")
                .context("Failed to show success message")?;
        }
        DeployConfig::None => {
            outro("You can setup deployment yourself manually or call `leco deploy-config` at any time")
                .context("Failed to show manual setup message")?;
        }
    }

    Ok(())
}

fn select_rust_toolchain() -> Result<RustToolchain> {
    // TODO : check for rust-toolchain.toml

    let toolchain = select("Select a Rust toolchain:")
        .item(RustToolchain::Stable, "stable", "")
        .item(RustToolchain::Nightly, "nightly", "")
        .initial_value(RustToolchain::Stable)
        .interact()
        .context("Failed to get Rust toolchain selection")?;

    Ok(toolchain)
}
