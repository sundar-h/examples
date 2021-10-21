pub mod c_protocols;

use libloading::{Library, Symbol};

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// 这里必须有*mut
type ReturnConstChar = extern "C" fn() -> *mut ::std::os::raw::c_char;
type Add = extern "C" fn(a: ::std::os::raw::c_int, b: ::std::os::raw::c_int) -> ::std::os::raw::c_int;

pub fn run() {
    unsafe {
        let lib = Library::new("./shared/c_protocols.so").unwrap();
        let return_const_char: Symbol<ReturnConstChar> = lib.get(b"ReturnConstChar").unwrap();
        let res = return_const_char();
        let res = CStr::from_ptr(res).to_str().unwrap();
        let res = res.to_string();
        println!("{}, {}, {}, {}", res, res.is_empty(), res.capacity(), res.len());


        // let add: Symbol<Add> = lib.get(b"Add").unwrap();
        // let sum = add(1, 2);
        // println!("Got sum: {}", sum);
    }
}
