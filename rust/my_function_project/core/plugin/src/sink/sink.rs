use std::os::raw::c_char;

use crate::plugin::ExternPlugin;
use crate::errors::Result;

type SendFunction = unsafe extern "C" fn(*const c_char) -> *const c_char;

pub trait Sink: Plugin {
    fn send(&self, payload: String) -> Result<()>;
}

pub struct ExternSink {
    base: ExternPlugin,
    send: SendFunction,
}

impl Sink for ExternSink {
    fn send(&self, payload: String) -> Result<()> {
        todo!()
    }
}