use std::os::raw::c_char;

use libloading::{Library, Symbol};

use crate::errors::*;

// used by host language
pub trait Plugin {
    fn name() -> String;
    fn lang_type() -> String;
    fn plugin_type() -> String;
    fn init(config: String) -> Result<()>;
    fn finalize() -> Result<()>;
}

pub struct ExternPlugin {
    library: Library,
    // name_function: Symbol<NameFunction>,
}

type NameFunction = unsafe extern "C" fn() -> *const c_char;
type LangTypeFunction = unsafe extern "C" fn() -> *const c_char;
type PluginTypeFunction = unsafe extern "C" fn() -> *const c_char;
type InitializeFunction = unsafe extern "C" fn(config: *const c_char) -> *const c_char;
type FinalizeFunction = unsafe extern "C" fn() -> *const c_char;

impl ExternPlugin {
    fn load(lib_path: &str) -> Result<Self> {
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

// impl Plugin for ExternPlugin {
//     fn lang_type() -> String {
//         todo!()
//     }

//     fn plugin_type() -> String {
//         todo!()
//     }

//     fn init(config: String) -> Result<()> {
//         todo!()
//     }

//     fn finalize() -> Result<()> {
//         todo!()
//     }
// }