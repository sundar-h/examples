use std::collections::HashMap;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Event {
    name: String,
    len: i32,
}

extern "C" {
    pub fn register_callback(arg1: *mut Event, arg2: rust_callback) -> ::std::os::raw::c_int;
    pub fn trigger_callback();
}

extern "C" fn callback(e: Event) {
}

pub fn run() {
    let mut e = Event::default();
    unsafe {
        register_callback(Box::into_raw(Box::new(e)), callback);
        trigger_callback();
    }

    println!("{}", e);
}