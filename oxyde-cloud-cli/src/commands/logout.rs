use anyhow::{Context, Result};
use keyring::Entry;

pub fn logout() -> Result<()> {
    let keyring_entry = Entry::new("oxyde-cloud", "api-key")
        .context("Failed to create keyring entry for logout")?;

    if let Err(err) = keyring_entry.delete_credential() {
        if let keyring::Error::NoEntry = err {
            // already logged out
            Ok(())
        } else {
            Err(err).context("Failed to delete API key from keyring")
        }
    } else {
        Ok(())
    }
}
