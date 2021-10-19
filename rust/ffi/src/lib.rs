pub mod c_protocols;

pub use crate::c_protocols::*;
use libloading::{Library, Symbol};

use std::ffi::{c_void, CStr, CString};
use std::os::raw::c_char;

unsafe extern "C" fn callback(e: *mut Event) {
    println!("I'm called from C with value {0}", arg1);
}

struct Vtable {
}

type Init = extern "C" fn(*mut Object) -> ::std::os::raw::c_int;

struct Plugin {
    #[allow(dead_code)]
    library: libloading::Library,
    object: *mut Object,
    vtable: Vtable,
}

pub fn run() {
    unsafe {
        let lib = Library::new("./shared/shared.so").unwrap();
        let trigger_callback: Symbol<SimpleCallback> = lib.get(b"trigger_callback_simple").unwrap();

        trigger_callback(Some(callback));


        // let trigger_callback: Symbol<StructCallback> = lib.get(b"trigger_callback_struct").unwrap();
        // let name= CString::new("[my-awesome-shell] $").unwrap();
        //
        // let mut ctx = Context{
        //     name: name.as_ptr(),
        //     year: 32
        // };
        // trigger_callback(&mut ctx, Some(callback));
    }
}
