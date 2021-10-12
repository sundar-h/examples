pub mod ffi;

pub use crate::ffi::*;
use libloading::{Library, Symbol};

use std::ffi::{c_void, CStr, CString};
use std::os::raw::c_char;

// type FreeObject = extern "C" fn(*mut Object);

// type NameObject = extern "C" fn() -> *mut ::std::os::raw::c_char;
// type NullObject = extern "C" fn() -> *mut ::std::os::raw::c_char;
// type EmptyObject = extern "C" fn() -> *mut ::std::os::raw::c_char;
// type InitializeObject = extern "C" fn();
// type NextMessageObject = extern "C" fn() -> *mut ::std::os::raw::c_char;
// type PublishObject = extern "C" fn(*mut ::std::os::raw::c_char);
// type FinalizeObject = extern "C" fn();

// pub fn run() {
//     unsafe {
//         let lib = Library::new("./libgo.so").unwrap();

//         // type FunctionName = unsafe extern "C" fn() -> *mut ::std::os::raw::c_char;
//         // let func_name: Symbol<FunctionName> = lib.get(b"name").unwrap();

//         let name_function: Symbol<NameObject> = lib.get(b"Name").unwrap();
//         let null_function: Symbol<NullObject> = lib.get(b"Null").unwrap();
//         let empty_function: Symbol<EmptyObject> = lib.get(b"Empty").unwrap();
//         let initialize_function: Symbol<InitializeObject> = lib.get(b"Initialize").unwrap();
//         let next_message_function: Symbol<NextMessageObject> = lib.get(b"NextMessage").unwrap();
//         let publish_function: Symbol<PublishObject> = lib.get(b"Publish").unwrap();
//         let finalize_function: Symbol<FinalizeObject> = lib.get(b"Finalize").unwrap();

//         // let c_str = CStr::from_ptr(name_function());
//         // println!("lalalallalala {}", c_str.to_str().expect("name() "));

//         // let c_str = CStr::from_ptr(null_function());
//         // // println!("lalalallalala {}", c_str.to_str().expect("name() "));
//         // if c_str.to_str().is_err() {
//         //     println!("ERROR")
//         // }

//         let c_str = CStr::from_ptr(empty_function());
//         println!("lalalallalala {}", c_str.to_str().expect("name() "));

//         // initialize_function();

//         // let c_str = CString::new("hello world").unwrap();
//         // let c_world: *mut c_char = c_str.as_ptr() as *mut c_char;
//         // publish_function(c_world);
//         // publish_function(c_world);
//         // publish_function(c_world);
//         // publish_function(c_world);

//         // loop {
//         //     let msg = CStr::from_ptr(next_message_function());
//         //     // if msg.to_str()
//         //     println!("got message: {}", msg.to_str().expect("msg"));
//         // }

//         // finalize_function();
//     }
// }

// extern fn callback(name: *mut ::std::os::raw::c_char) {
//     unsafe {
//         let c_str = CStr::from_ptr(name);
//         println!("my name is: {:?}", c_str.to_str());
//     }
// }

extern fn callback() {
        println!("call back from rust");
}

// #[link(name = "extlib")]
// extern {
//    fn register_callback(cb: extern fn(::std::os::raw::c_char)) -> i32;
//    fn trigger_callback();
// }

type SetNameObject = extern "C" fn(*mut ::std::os::raw::c_char);
type RegisterCallbackObject = extern "C" fn(callback: *mut ::std::os::raw::c_void);
type TriggerCallbackObject = extern "C" fn();
pub fn run() {
    unsafe {
        let lib = Library::new("./libgo.so").unwrap();


        let set_name: Symbol<SetNameObject> = lib.get(b"SetName").unwrap();
        let register_callback: Symbol<RegisterCallbackObject> = lib.get(b"RegisterCallback").unwrap();
        let trigger_callback: Symbol<TriggerCallbackObject> = lib.get(b"TriggerCallback").unwrap();

        // *mut i8
        let c_str = CString::new("hello world").unwrap();
        let c_world: *mut c_char = c_str.as_ptr() as *mut c_char;
        set_name(c_world);

        // let cb: *mut c_void = &mut callback;
        register_callback(callback as *mut c_void);
        trigger_callback(); // Triggers the callback.
    }
}