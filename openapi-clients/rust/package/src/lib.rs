// Include generated modules directly from the generated/src directory
#[path = "generated/src/apis/mod.rs"]
pub mod apis;
#[path = "generated/src/models/mod.rs"]
pub mod models;

use apis::configuration::Configuration;

/// Create a Bible API client with optional custom base path
/// 
/// # Examples
/// 
/// ```
/// use holy_bible_api::create_bible_api;
/// 
/// // Use default endpoint
/// let config = create_bible_api(None);
/// 
/// // Use custom endpoint
/// let config = create_bible_api(Some("https://holy-bible-api.com".to_string()));
/// ```
pub fn create_bible_api(base_path: Option<String>) -> Configuration {
    let mut config = Configuration::new();
    config.base_path = base_path.unwrap_or_else(|| "https://holy-bible-api.com".to_string());
    config
}

