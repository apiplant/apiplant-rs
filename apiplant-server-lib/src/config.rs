use std::fs;
use std::collections::HashMap;
use toml;
use serde::Deserialize;
use colored::*;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct ServerOptions {
    #[serde(default = "default_mount_address")]
    pub mount_address: String,
    #[serde(default = "default_mount_path")]
    pub mount_path: String,
}
fn default_mount_address() -> String {
    "0.0.0.0:1337".to_string()
}
fn default_mount_path() -> String {
    "/".to_string()
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct DriverChoice {
    #[serde(default = "default_model_storage")]
    pub model_storage: String,
    #[serde(default = "default_event_storage")]
    pub event_storage: String,
    #[serde(default = "default_error_storage")]
    pub error_storage: String,
}
fn default_model_storage() -> String {
    "memory".to_string()
}
fn default_event_storage() -> String {
    "memory".to_string()
}
fn default_error_storage() -> String {
    "stdout".to_string()
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Config {
    #[serde(default = "default_server")]
    pub server: ServerOptions,
    #[serde(default = "default_drivers")]
    pub drivers: DriverChoice,
    pub driver_event_storage: Option<HashMap<String, String>>,
    pub driver_model_storage: Option<HashMap<String, String>>,
    pub driver_error_storage: Option<HashMap<String, String>>,
}
fn default_server() -> ServerOptions {
    let mount_address = default_mount_address();
    let mount_path = default_mount_path();
    ServerOptions { mount_address, mount_path }
}
fn default_drivers() -> DriverChoice {
    let model_storage = default_model_storage();
    let event_storage = default_event_storage();
    let error_storage = default_error_storage();
    DriverChoice { model_storage, event_storage, error_storage }
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