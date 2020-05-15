use libloading::Library;
use std::{ffi::OsStr, io, rc::Rc};
use bigdecimal::BigDecimal;
use chrono::prelude::*;
use std::net::IpAddr;
use serde_json::Value;
use geoutils::Location;
use crate::plugins::{APIPLANT_VERSION, RUSTC_VERSION, PluginDeclaration};
use crate::config::DriverChoice;

pub enum FieldType {
    bool, String, i32, f32, BigDecimal, DateDateTime<Utc>, IpAddr, Value, Location
}

pub struct ModelField {
    name: String,
    type: FieldType,
    default: FieldType,
    
}

pub trait ModelStorage {
    fn get_definitions(&self) ->  Option<&str> { None }
    fn get_definition(&self) -> Option<&str> { None }
    fn create_definition(&self) -> Option<&str> { None }
    fn update_definition(&self) -> Option<&str> { None }
    fn delete_definition(&self) -> Option<&str> { None }

    fn get_instances(&self) ->  Option<&str> { None }
    fn get_instance(&self) -> Option<&str> { None }
    fn create_instance(&self) -> Option<&str> { None }
    fn update_instance(&self) -> Option<&str> { None }
    fn delete_instance(&self) -> Option<&str> { None }

    fn get_hooks(&self) ->  Option<&str> { None }
    fn get_hook(&self) ->  Option<&str> { None }
    fn update_hook(&self) ->  Option<&str> { None }
}

pub trait EventStorage {
    fn publish_definition(&self) ->  Option<&str> { None }
    fn listen_definition(&self) ->  Option<&str> { None }
    fn publish_instance(&self) ->  Option<&str> { None }
}

pub trait ErrorStorage {
    fn log(&self) ->  Option<&str> { None }
}

#[derive(Default)]
pub struct PluginRegistry {
    pub model_storage: Option<Box<dyn ModelStorage>>,
    pub event_storage: Option<Box<dyn EventStorage>>,
    pub error_storage: Option<Box<dyn ErrorStorage>>,
    drivers: DriverChoice,
    libraries: Vec<Rc<Library>>,
}

pub trait PluginRegistrar {
    fn register_model_storage(&mut self, _: &str, _: Box<dyn ModelStorage>) { () }
    fn register_event_storage(&mut self, _: &str, _: Box<dyn EventStorage>) { () }
    fn register_error_storage(&mut self, _: &str, _: Box<dyn ErrorStorage>) { () }
}

impl PluginRegistrar for PluginRegistry {
    fn register_model_storage(&mut self, name: &str, model_storage: Box<dyn ModelStorage>) {
        if self.drivers.model_storage == name {
            self.model_storage = Some(model_storage)
        }
    }
    fn register_event_storage(&mut self, name: &str, event_storage: Box<dyn EventStorage>) { 
        if self.drivers.event_storage == name {
            self.event_storage = Some(event_storage)
        }
    }
    fn register_error_storage(&mut self, name: &str, error_storage: Box<dyn ErrorStorage>) { 
        if self.drivers.error_storage == name {
            self.error_storage = Some(error_storage)
        }
    }
}

impl PluginRegistry {
    pub fn new() -> PluginRegistry { PluginRegistry::default() }
    pub unsafe fn load<P: AsRef<OsStr>>(
        &mut self,
        library_path: P,
    ) -> io::Result<()> {
        let library = Rc::new(Library::new(library_path)?);
        let decl = library
            .get::<*mut PluginDeclaration>(b"plugin_declaration\0")?
            .read();
        if decl.apiplant_version != APIPLANT_VERSION {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "apiplant version mismatch",
            ));
        }
        if decl.rustc_version != RUSTC_VERSION {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "rustc version mismatch",
            ));
        }
        (decl.register)(self);
        self.libraries.push(library);
        Ok(())
    }
}
