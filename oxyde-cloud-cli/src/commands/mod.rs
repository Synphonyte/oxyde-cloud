use lazy_static::lazy_static;
use tera::Tera;

pub mod deploy_config;
pub mod init;
pub mod log;
pub mod login;
pub mod logout;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();

        if let Err(e) = tera.add_raw_template(
            "oxyde-cloud.toml",
            include_str!("../../templates/oxyde-cloud.toml"),
        ) {
            println!("Parsing error(s): {e}");
            ::std::process::exit(1);
        }

        if let Err(e) = tera.add_raw_template(
            "github-workflow.yml",
            include_str!("../../templates/github-workflow.yml"),
        ) {
            println!("Parsing error(s): {e}");
            ::std::process::exit(1);
        }

        tera
    };
}
