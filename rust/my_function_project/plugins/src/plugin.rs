use anyhow::Result;
use crate::errors::PluginError;

#[repr(C)]
struct Config {
    name: String,
    lang_type: String,
    plugin_type: String,
}

pub trait Plugin {
    fn init(config: &str) -> Result<PluginError>;
    fn finalize() -> Result<PluginError>;

    fn info() -> String;
}



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
