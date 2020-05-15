use apiplant_server_lib::{ModelStorage, PluginRegistrar};

apiplant_server_lib::export_plugin!(register);

extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.register_model_storage("memory", Box::new(MemoryModel))
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemoryModel {
    definitions: 
    instances: 
};

impl ModelStorage for MemoryModel {
    fn get_definitions(&self) -> Option<&str> {
        Some("Hello from the other side")
    }
}