use cargo_leptos::config::Commands::Build;
use cargo_leptos::config::{Cli, Opts};
use cargo_leptos::run;

const BUILD_TARGET: &'static str = "x86_64-unknown-linux-gnu";

pub async fn build(mut cargo_leptos_opts: Opts) -> anyhow::Result<()> {
    // let mut cargo_args = cargo_leptos_opts.bin_cargo_args.unwrap_or_default();
    // cargo_args.push("--target".to_string());
    // cargo_args.push(BUILD_TARGET.to_string());
    // cargo_leptos_opts.bin_cargo_args = Some(cargo_args);
    
    run(Cli {
        manifest_path: None,
        log: vec![],
        command: Build(cargo_leptos_opts),
    })
    .await
    .map_err(|e| anyhow::anyhow!(e))
}
