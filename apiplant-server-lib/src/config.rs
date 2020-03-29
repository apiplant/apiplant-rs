use std::fs;
use std::collections::HashMap;
use toml;
use serde::Deserialize;
use colored::*;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Config {
    pub server: Option<HashMap<String, String>>,
    pub driver_secure_storage: Option<HashMap<String, String>>,
    pub driver_event_storage: Option<HashMap<String, String>>,
    pub driver_model_storage: Option<HashMap<String, String>>,
    pub driver_error_storage: Option<HashMap<String, String>>,
}

impl Config {
    pub fn new() -> Config { Config::default() }
}

pub fn load_config(config_path: &str) -> Config {
    let config_contents = fs::read_to_string(&config_path)
        .unwrap_or_else(|err| {
            println!("{} can't read {}: {}", "warning:".bright_yellow().bold(), &config_path, err);
            "".to_string()
        });
    toml::from_str(&config_contents)
        .unwrap_or_else(|err| {
            println!("{} can't parse {}: {}", "warning:".bright_yellow().bold(), &config_path, err);
            Config::new()
        })
}