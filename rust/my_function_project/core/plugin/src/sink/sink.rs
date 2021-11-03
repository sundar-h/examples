use crate::plugin::ExternPlugin;
use crate::errors::Result;

// pub struct Sink(ExternPlugin);

pub trait ExternPluginExt {
    fn send(&self, payload: String) -> Result<()>;
}

impl ExternPluginExt for ExternPlugin {
}