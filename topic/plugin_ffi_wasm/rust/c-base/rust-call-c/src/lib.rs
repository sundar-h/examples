// #[no_mangle]
// pub extern "C" fn echo_str(str: String) {
// }

// #[no_mangle]
// pub extern "C" fn echo_bytes() {
// }

// #[no_mangle]
// pub extern "C" fn echo_struct(str: String) {
// }

// #[no_mangle]
// pub extern "C" fn callback_echo() {
// }

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::os::raw::c_void;

/// * 基础类型对应使用std::os::raw::库(或者libc 区别是libc是no std依赖的)对应的类型
/// * 结构体对应需要手动定义
///     rust中结构体默认的内存表示和c并不兼容。如果要将结构体传给ffi函数，请为rust的结构体打上标记：
///     如果使用#[repr(C, packed)]将不为此结构体填充空位用以对齐。
/// * enum对应同结构体
#[repr(C)]
struct RustObject {
    i: c_int,
    s: c_char,
}

/// 外部链接库, 对应libecho.[a/so/dll/dylib]
#[link(name = "echo")]
extern "C" {
    // 传递不可变
    fn echo_int(a: c_int);
    fn echo_str(a: *const c_char);
    fn echo_struct(a: *mut RustObject);
    fn echo_bytes(a: c_int);
    fn echo_void_int(a: c_void);
    fn callback_echo();

    fn return_int() -> c_int;
    fn return_str() -> *const c_char;
    fn return_bytes() -> c_char;
    fn return_struct() -> c_void;
}

fn echo_int_wrapper(a: i32) {
    unsafe { echo_int(a as c_int) }
}

// 传递所有权并且 String 不可变, 如果变更需要使用
fn echo_str_wrapper(s: &str) {
    unsafe { echo_str(CString::new(s).unwrap().as_ptr()) }
}

fn return_str_wrapper() {
    let s = unsafe {
        let s = return_str();
        CStr::from_ptr(s)
    };
    println!("{}", s.to_string_lossy().into_owned())
}

pub fn run() {
    echo_int_wrapper(3);

    echo_str_wrapper("Hello World");
    return_str_wrapper()
}
