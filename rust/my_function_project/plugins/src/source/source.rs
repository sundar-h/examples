use crate::Plugin;
use errors::*;
use anyhow::Result;

// extern fn rust_callback() -> PluginErr

// Stream 或者 是Iterator之类的

// 数据流入的地方
trait Source: Plugin {
    // 一种是基于回调
    // register_callback(cb: ) -> Result<PluginError>;
    // 一种是直接开放数据给到外部
    fn next(&self) -> Option<String>;
}

// extern "C" fn next() -> Option<*std::os::raw::c_char>;

struct ExternSource {
    source: Box<dyn Source>
}

impl Source for ExternSource {
}