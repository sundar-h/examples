use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn return_str() -> *const c_char {
    CString
}

// #[no_mangle]
// pub extern "C" fn pass_bytes() {
// }

// #[no_mangle]
// pub extern "C" fn pass_struct(str: String) {
// }

// #[no_mangle]
// pub extern "C" fn callback_pass() {
// }
