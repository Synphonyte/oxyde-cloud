use anyhow::{Context, Result};
use keyring::Entry;

#[inline(always)]
pub fn api_key_entry() -> Result<Entry> {
    Entry::new("oxyde-cloud", "api-key").context("Failed to create keyring entry for API key")
}

pub fn api_key() -> Result<String> {
    if let Ok(api_key) = std::env::var("OXYDE_CLOUD_API_KEY") {
        Ok(api_key)
    } else {
        api_key_entry()?.get_password().context(
            "Failed to get API key from keyring. Make sure you're logged in with 'oxy login'",
        )
    }
}
