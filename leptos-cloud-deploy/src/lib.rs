mod build;
mod deploy;

pub use cargo_leptos::config::Opts;
pub use deploy::{deploy_with_config_file, Error};
