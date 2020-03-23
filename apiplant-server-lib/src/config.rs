use std::fs;
use std::collections::HashMap;
use toml;
use serde::Deserialize;
use colored::*;

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Config {
    server: Option<HashMap<String, String>>,
    driver_secure_storage: Option<HashMap<String, String>>,
    driver_event_storage: Option<HashMap<String, String>>,
    driver_model_storage: Option<HashMap<String, String>>,
    driver_error_storage: Option<HashMap<String, String>>,
}

pub fn load_config(config_path: &str) -> Config {
    let config_contents = fs::read_to_string(&config_path)
        .unwrap_or_else(|err| {
            println!("{} Can't read {}: {}", "warning:".bright_yellow().bold(), &config_path, err);
            "".to_string()
        });
    toml::from_str(&config_contents).unwrap()
}