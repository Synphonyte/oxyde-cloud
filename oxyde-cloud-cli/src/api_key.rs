use keyring::{Entry, Result};

#[inline(always)]
pub fn api_key_entry() -> Result<Entry> {
    Entry::new("oxyde-cloud", "api-key")
}

pub fn api_key() -> Result<String> {
    if let Ok(api_key) = std::env::var("LEPTOS_CLOUD_API_KEY") {
        Ok(api_key)
    } else {
        api_key_entry()?.get_password()
    }
}
