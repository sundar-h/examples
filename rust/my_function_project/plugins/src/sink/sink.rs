use std::io::Result;

use crate::{PluginError, plugin::Plugin};

// 数据流出的地方
trait Sink: Plugin {
    // 同步操作
    fn send(&self, payload: String) -> Result<PluginError>;
}