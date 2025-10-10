use crate::routes::swagger::ApiDoc;
use serde_json;
use std::fs;
use std::path::Path;
use tracing::{info, warn};
use utoipa::OpenApi;

pub fn generate_and_save_openapi_json(output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let openapi_spec = ApiDoc::openapi();
    let json_string = serde_json::to_string_pretty(&openapi_spec)?;
    
    if let Some(parent_dir) = Path::new(output_path).parent() {
        fs::create_dir_all(parent_dir)?;
    }
    
    fs::write(output_path, json_string)?;
    Ok(())
}

pub fn try_generate_and_save_openapi_json(output_path: &str) {
    match generate_and_save_openapi_json(output_path) {
        Ok(_) => info!("Successfully generated OpenAPI JSON at startup"),
        Err(e) => warn!("Failed to generate OpenAPI JSON at startup: {}", e),
    }
}

pub fn try_generate_and_save_openapi_json_default() {
    try_generate_and_save_openapi_json("openapi.json");
}
