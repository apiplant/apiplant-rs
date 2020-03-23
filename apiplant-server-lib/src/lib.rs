mod config;
mod plugins;
pub use plugins::*;

pub fn init_apiplant(config_path: &str, plugins_path: &str, hooks_path: &str) -> () {
    let config = config::load_config(&config_path);
    // Load Plugins
    // Load hooks
}
