use keyring::{Entry, Result};

pub fn api_key_entry() -> Result<Entry> {
    Entry::new("leptos-cloud", "api-key")
}
