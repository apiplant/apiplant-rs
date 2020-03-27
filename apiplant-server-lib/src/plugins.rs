use std::{fs, io, env, path::Path};
use colored::*;
use crate::app_interface::{App, PluginRegistrar};

pub static APIPLANT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

#[derive(Copy, Clone)]
pub struct PluginDeclaration {
    pub rustc_version: &'static str,
    pub apiplant_version: &'static str,
    pub register: unsafe extern "C" fn(&mut dyn PluginRegistrar),
}

#[macro_export]
macro_rules! export_plugin {
    ($register:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static plugin_declaration: $crate::PluginDeclaration =
            $crate::PluginDeclaration {
                apiplant_version: $crate::APIPLANT_VERSION,
                rustc_version: $crate::RUSTC_VERSION,
                register: $register,
            };
    };
}

pub fn load_plugins(plugins_path: &str) -> App {
    let mut app = App::new();
    if !Path::new(plugins_path).is_dir() {
        println!("{} plugins_path {} is not a directory", "warning:".bright_yellow().bold(), plugins_path);
        return app;
    }
    load_plugins_files(plugins_path, &mut app).unwrap_or(());
    app
}

fn load_plugins_files(base_path: &str, app: &mut App) -> io::Result<()> {
    for entry in fs::read_dir(base_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(path_str) = path.to_str() {
                load_plugins_files(path_str, app).unwrap_or(());
            }
            continue;            
        }
        unsafe {
            app.load(&path).unwrap_or(());
        }
    }
    Ok(())
} 
