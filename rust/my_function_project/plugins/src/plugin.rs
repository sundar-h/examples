use crate::errors::PluginError;
use std::intrinsics::sub_with_overflow;
use std::os::raw::c_char;

// #[repr(C)]
// struct Config {
//     _private: [u8; 0],
// }

pub enum PluginType {
    Sink,
    Source,
    Function,
    Unsupported,
}

impl From(&str) for PluginType {
    fn from(s: &str) -> Self {}
}

// Rust实现的插件使用这个, 需要私用abi_stable 来屏蔽版本差异
pub trait Plugin {
    fn init(&self, config: &str) -> Result<(), PluginError>;
    fn finalize(&self) -> Result<(), PluginError>;

    fn name(&self) -> String;
    fn lang_type(&self) -> String {
        "Rust".to_string()
    }
    fn plugin_type(&self) -> Result<PluginType, PluginError>;
}

pub type ExternName = extern "C" fn() -> c_char;
pub type ExternInit = extern "C" fn(config: *const c_char) -> c_char;
pub type ExternFinalize = extern "C" fn() -> c_char;
pub type ExternLangType = extern "C" fn() -> c_char;
pub type ExternPluginType = extern "C" fn() -> c_char;

// struct ExternPlugin {
//     plugin: Box<dyn Plugin>,
// }

// 外部语言实现的拆件使用这个接口
// 最终再宿主语言上 表现为Plugin一致的特性
// 屏蔽底层语言管理/内存安全方面的差异
// pub trait ExternPlugin: Plugin {
//     fn new() -> Box<dyn ExternPlugin> {
//         todo!();
//     }
// }

// impl<'a> dyn ExternPlugin + 'a {}

// ## 接收字符串
// 保持对外部字符串的借用，而不是直接复制一份。
// 在转换数据类型时最小化unsafe的代码区域。

// let msg_str: &str = unsafe {
//             // SAFETY: accessing raw pointers expected to live for the call,
//             // and creating a shared reference that does not outlive the current
//             // stack frame.
//             match std::ffi::CStr::from_ptr(msg).to_str() {
//                 Ok(s) => s,
//                 Err(e) => {
//                     crate::log_error("FFI string conversion failed");
//                     return;
//                 }
//             }
//         };
// }

// ## 传递字符串

// 让拥有的字符串生命周期尽可能长。
// 在转换时保持最小化unsafe区域代码。
// 如果C语言代码会修改字符串数据，那么使用Vec类型而不是CString。
// 除非外部函数的API需要字符串的所有权，否则不要传给被调用的函数。
