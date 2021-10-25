use crate::{BasePlugin, PluginError};

// 数据流出的地方
pub trait Sink: BasePlugin {
    // 同步操作
    fn send(&self, payload: String) -> Result<(), PluginError>;
}
