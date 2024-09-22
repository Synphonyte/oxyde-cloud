use thiserror::Error;
use crate::config::CloudConfig;

#[derive(Debug, Error)]
pub enum Error {}

pub fn build(config: &CloudConfig) -> Result<(), Error> {
    // TODO
    Ok(())
}