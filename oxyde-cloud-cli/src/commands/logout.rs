use keyring::Entry;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Keyring error: {0}")]
    Keyring(#[from] keyring::Error),
}

pub fn logout() -> Result<(), Error> {
    let keyring_entry = Entry::new("oxyde-cloud", "api-key")?;

    if let Err(err) = keyring_entry.delete_credential() {
        if let keyring::Error::NoEntry = err {
            // already logged out
            Ok(())
        } else {
            Err(err.into())
        }
    } else {
        Ok(())
    }
}
