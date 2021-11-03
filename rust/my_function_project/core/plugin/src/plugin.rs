use libloading::{Library, Symbol};

use crate::errors::*;
use std::{ffi::OsStr, os::raw::c_char};

// used by host language
pub trait Plugin {
    fn lang_type() -> String;
    fn plugin_type() -> String;
    fn init(config: String) -> Result<()>;
    fn finalize() -> Result<()>;
}

pub struct ExternPlugin {
    library: Library,
}

type NameSymbol = extern "C" fn() -> *const c_char;
type LangTypeSymbol = extern "C" fn() -> *const c_char;
type PluginTypeSymbol = extern "C" fn() -> *const c_char;
type InitSymbol = extern "C" fn(config: *const c_char) -> *const c_char;
type FinalizeSymbol = extern "C" fn() -> *const c_char;

impl ExternPlugin {
    fn load<P: AsRef<OsStr>>(filename: P) -> Result<Box<dyn Plugin>> {
        let lib = Library::new(filename)?;

        let name: Symbol<NameSymbol> = lib.get(b"name")?;
        let name = name.into_raw();

        let lang_type: Symbol<LangTypeSymbol> = lib.get(b"lang_type")?;
        let lang_type = lang_type.into_raw();

        let plugin_type: Symbol<PluginTypeSymbol> = lib.get(b"plugin_type")?;
        let plugin_type = plugin_type.into_raw();

        let init: Symbol<InitSymbol> = lib.get(b"init")?;
        let init = init.into_raw();

        let finalize: Symbol<FinalizeSymbol> = lib.get(b"finalize")?;
        let finalize = finalize.into_raw();

        Ok(ExternPlugin{
            library: lib,
        })
    }
}

impl Plugin for ExternPlugin {
    fn lang_type() -> String {
        todo!()
    }

    fn plugin_type() -> String {
        todo!()
    }

    fn init(config: String) -> Result<()> {
        todo!()
    }

    fn finalize() -> Result<()> {
        todo!()
    }
}

// impl Drop for Box<dyn Plugin> {
//     fn drop(&mut self) {
//     }
// }
