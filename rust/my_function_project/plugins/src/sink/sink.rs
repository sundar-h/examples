use crate::errors::PluginError;
use crate::{plugin, ExternFinalize, ExternInit, ExternLangType, ExternName, ExternPluginType};
use std::ffi::{CStr, CString, OsStr};
use std::os::raw::{c_char, c_int};

use crate::plugin::{Plugin, PluginType};
use libloading::os::unix::Symbol as RawSymbol;
use libloading::{Library, Symbol};

// 数据流出的地方
trait Sink: Plugin {
    // 同步操作
    fn send(&self, payload: String) -> Result<(), PluginError>;
}

pub struct Vtable {
    extern_name: RawSymbol<ExternName>,
    extern_init: RawSymbol<ExternInit>,
    extern_finalize: RawSymbol<ExternFinalize>,
    extern_lang_type: RawSymbol<ExternLangType>,
    extern_plugin_type: RawSymbol<ExternPluginType>,
    extern_send: RawSymbol<ExternSend>,
}

// 加载外部插件
// 查找外部符号表
// 转换为Rust的
pub struct ExternSink {
    library: Library,
    vtable: Vtable,
}

type ExternSend = extern "C" fn(payload: *const c_char) -> c_char;

impl ExternSink {
    unsafe fn new<P: AsRef<OsStr>>(filename: P) -> Result<Box<dyn Plugin>, PluginError> {
        let lib = Library::new(filename)?;
        let extern_name: Symbol<ExternName> = lib.get(b"name")?;
        let extern_init: Symbol<ExternInit> = lib.get(b"init")?;
        let extern_finalize: Symbol<ExternFinalize> = lib.get(b"finalize")?;
        let extern_lang_type: Symbol<ExternLangType> = lib.get(b"lang_type")?;
        let extern_plugin_type: Symbol<ExternPluginType> = lib.get(b"plugin_type")?;
        let extern_send: Symbol<ExternSend> = lib.get(b"send")?;

        let plugin: Box<dyn Plugin> = Box::new(&Self {
            library: lib,
            vtable: Vtable {
                extern_init: extern_init.into_raw(),
                extern_name: extern_name.into_raw(),
                extern_finalize: extern_finalize.into_raw(),
                extern_lang_type: extern_lang_type.into_raw(),
                extern_plugin_type: extern_plugin_type.into_raw(),
                extern_send: extern_send.into_raw(),
            },
        });
        Ok(plugin)
    }
}

impl Plugin for ExternSink {
    fn init(&self, config: &str) -> Result<(), PluginError> {
        let config = CString::new(config.into())?;
        let err = unsafe {
            let err = self.vtable.extern_init(config.as_ptr());
            CStr::from_ptr(err).to_str()?.to_string()
        };
        if !err.is_empty() {
            return Err(PluginError::Return(err));
        }
        Ok(())
    }

    fn finalize(&self) -> Result<(), PluginError> {
        let err = unsafe {
            let err = self.vtable.extern_finalize();
            CStr::from_ptr(err).to_str()?.to_string()
        };
        if !err.is_empty() {
            return Err(PluginError::Return(err));
        }
        Ok(())
    }

    fn name(&self) -> String {
        unsafe {
            let name = self.vtable.extern_name();
            CStr::from_ptr(name).to_str()?.to_string()
        }
    }

    fn lang_type(&self) -> String {
        unsafe {
            let res = self.vtable.extern_lang_type();
            CStr::from_ptr(res).to_str()?.to_string()
        }
    }

    fn plugin_type(&self) -> Result<PluginType, PluginError> {
        unsafe {
            let res = self.vtable.extern_plugin_type();
            let res = CStr::from_ptr(res).to_str()?;
            PluginType::from(res)
        }
    }
}

impl Sink for ExternSink {
    // 同步
    fn send(&self, payload: String) -> Result<(), PluginError> {
        let payload = CString::new(payload)?;

        let err = unsafe {
            let err = self.vtable.extern_send(payload.as_ptr());
            // 接受字符串 保持对字符串的额借用, 谁生产的 谁负责销毁
            // 如果需要所有权的话, 使用.to_string()方法
            CStr::from_ptr(err).to_str()?.to_string()
        };
        if !err.is_empty() {
            return Err(PluginError::Return(err));
        }

        Ok(())
    }
}

impl Drop for ExternSink {
    fn drop(&mut self) {
        self.vtable.extern_finalize();
    }
}
