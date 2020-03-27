mod config;
mod plugins;
pub use plugins::{PluginDeclaration, APIPLANT_VERSION, RUSTC_VERSION};
mod app_interface;
pub use app_interface::{ModelStorage, EventStorage, SecureStorage, ErrorStorage, PluginRegistrar};

pub fn init_apiplant(config_path: &str, plugins_path: &str) -> () {
    let config = config::load_config(&config_path);
    let plugins = plugins::load_plugins(&plugins_path);

    if let Some(model_storage) = plugins.model_storage {
        if let Some(result) = model_storage.get_definitions() {
            println!(
                "{}",
                result
            );
        }
        
    }
    print!("{:#?}", config);
}
