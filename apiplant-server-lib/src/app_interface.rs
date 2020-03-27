use libloading::Library;
use std::{ffi::OsStr, io, rc::Rc};
use crate::plugins::{APIPLANT_VERSION, RUSTC_VERSION, PluginDeclaration};

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

pub trait SecureStorage {
    fn get(&self) ->  Option<&str> { None }
    fn set(&self) ->  Option<&str> { None }
    fn delete(&self) ->  Option<&str> { None }
}

pub trait ErrorStorage {
    fn log(&self) ->  Option<&str> { None }
}

#[derive(Default)]
pub struct App {
    pub model_storage: Option<Box<dyn ModelStorage>>,
    pub event_storage: Option<Box<dyn EventStorage>>,
    pub secure_storage: Option<Box<dyn SecureStorage>>,
    pub error_storage: Option<Box<dyn ErrorStorage>>,
    libraries: Vec<Rc<Library>>,
}

pub trait PluginRegistrar {
    fn register_model_storage(&mut self, _: Box<dyn ModelStorage>) { () }
    fn register_event_storage(&mut self, _: Box<dyn EventStorage>) { () }
    fn register_secure_storage(&mut self, _: Box<dyn SecureStorage>) { () }
    fn register_error_storage(&mut self, _: Box<dyn ErrorStorage>) { () }
}

impl PluginRegistrar for App {
    fn register_model_storage(&mut self, model_storage: Box<dyn ModelStorage>) { 
        self.model_storage = Some(model_storage)
    }
    fn register_event_storage(&mut self, event_storage: Box<dyn EventStorage>) { 
        self.event_storage = Some(event_storage)
    }
    fn register_secure_storage(&mut self, secure_storage: Box<dyn SecureStorage>) { 
        self.secure_storage = Some(secure_storage)
    }
    fn register_error_storage(&mut self, error_storage: Box<dyn ErrorStorage>) { 
        self.error_storage = Some(error_storage)
    }
}

impl App {
    pub fn new() -> App { App::default() }
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
