use apiplant_server_lib::{ModelStorage, PluginRegistrar};

apiplant_server_lib::export_plugin!(register);

extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.register_model_storage(Box::new(MemoryModel))
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemoryModel;

impl ModelStorage for MemoryModel {
    fn get_definitions(&self) -> Option<&str> {
        Some("Hello from the other side")
    }
}