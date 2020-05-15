use apiplant_server_lib::{ModelStorage, PluginRegistrar};

apiplant_server_lib::export_plugin!(register);

extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.register_model_storage("sqlx", Box::new(SQLModel))
}

#[derive(Debug, Clone, PartialEq)]
pub struct SQLModel;

impl ModelStorage for SQLModel {
    fn get_definitions(&self) -> Option<&str> {
        Some("Hello from the other side")
    }
}