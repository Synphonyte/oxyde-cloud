use cargo_leptos::config::Commands::Build;
use cargo_leptos::config::{Cli, Opts};
use cargo_leptos::run;

pub async fn build(cargo_leptos_opts: Opts) -> anyhow::Result<()> {
    run(Cli {
        manifest_path: None,
        log: vec![],
        command: Build(cargo_leptos_opts),
    })
    .await
}
