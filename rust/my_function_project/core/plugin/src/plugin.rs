use std::os::raw::c_char;

use libloading::{Library};

use crate::errors::*;

// used by host language
pub trait Plugin {
    fn name(&self) -> String;
    fn lang_type(&self) -> String;
    fn plugin_type(&self) -> String;
    fn init(&self, config: String) -> Result<()>;
    fn finalize(&self) -> Result<()>;
}

pub struct ExternPlugin {
    pub library: Library,
    // name_function: Symbol<NameFunction>,
}

type NameFunction = unsafe extern "C" fn() -> *const c_char;
type LangTypeFunction = unsafe extern "C" fn() -> *const c_char;
type PluginTypeFunction = unsafe extern "C" fn() -> *const c_char;
type InitializeFunction = unsafe extern "C" fn(config: *const c_char) -> *const c_char;
type FinalizeFunction = unsafe extern "C" fn() -> *const c_char;

impl ExternPlugin {
    pub fn load(lib_path: &str) -> Result<Self> {
        let lib = Library::new(lib_path)?;

        let name  = lib.get::<*mut NameFunction>(b"name\0")?.read();
        let lang_type  = lib.get::<*mut LangTypeFunction>(b"lang_type\0")?.read();
        let plugin_type  = lib.get::<*mut PluginTypeFunction>(b"plugin_type\0")?.read();
        let initialize= lib.get::<*mut InitializeFunction>(b"initialize\0")?.read();
        let finalize  = lib.get::<*mut FinalizeFunction>(b"finalize\0")?.read();

        let plugin = ExternPlugin{
            library: lib,
        };

        Ok(plugin)
    }
}

impl Plugin for ExternPlugin {
    fn name(&self) -> String {
        todo!()
    }

    fn lang_type(&self) -> String {
        todo!()
    }

    fn plugin_type(&self) -> String {
        todo!()
    }

    fn init(&self, config: String) -> Result<()> {
        todo!()
    }

    fn finalize(&self) -> Result<()> {
        todo!()
    }
}

impl Drop for ExternPlugin {
    fn drop(&mut self) {
        self.finalize()
    }
}