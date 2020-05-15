use apiplant_server_lib::{EventStorage, PluginRegistrar};

apiplant_server_lib::export_plugin!(register);

extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.register_event_storage("file", Box::new(FileModel))
}

#[derive(Debug, Clone, PartialEq)]
pub struct FileModel;

impl EventStorage for FileModel {
    fn publish_definition(&self) -> Option<&str> {
        Some("Hello from the other side")
    }
}