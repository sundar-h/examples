use std::os::raw::c_char;

use crate::plugin::{ExternPlugin, Plugin};
use crate::errors::Result;

type SendFunction = unsafe extern "C" fn(*const c_char) -> *const c_char;

pub trait Sink: Plugin {
    fn send(&self, payload: String) -> Result<()>;
}

pub struct ExternSink {
    base: ExternPlugin,
    send: SendFunction,
}

impl ExternSink {
    fn load(lib_path: &str) -> Result<Self> {
        let base = ExternPlugin::load(lib_path)?;
        let send= base.library.get::<*mut SendFunction>("send")?.read();

        Ok(ExternSink{
            base,
            send,
        })
    }
}

impl Sink for ExternSink {
    fn send(&self, payload: String) -> Result<()> {
        todo!()
    }
}

impl Plugin for ExternSink {
    fn name(&self) -> String {
        self.base.name()
    }

    fn lang_type(&self) -> String {
        self.base.lang_type()
    }

    fn plugin_type(&self) -> String {
        self.base.plugin_type()
    }

    fn init(&self, config: String) -> Result<()> {
        self.base.init(config)
    }

    fn finalize(&self) -> Result<()> {
        self.base.finalize()
    }
}