mod config;
mod plugins;
mod app_interface;
mod server;
use colored::*;

pub use plugins::{PluginDeclaration, APIPLANT_VERSION, RUSTC_VERSION};
pub use app_interface::{ModelStorage, EventStorage, SecureStorage, ErrorStorage, PluginRegistrar};

pub fn init_apiplant(config_path: &str, plugins_path: &str) -> () {
    let config = config::load_config(&config_path);
    let plugins = plugins::load_plugins(&plugins_path);
    server::start_server(config, plugins).unwrap_or_else(|err| {
        println!("{} can't start server: {}", "error:".bright_red().bold(), err);
        ()
    });
}
