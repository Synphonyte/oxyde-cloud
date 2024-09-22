use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {}

pub fn init() -> Result<(), Error> {
    // TODO
    Ok(())
}