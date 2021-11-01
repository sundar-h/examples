use std::collections::HashMap;

///*
/// Rust 的FFI 原则上谁定义结构体，谁拥有写权限, 另一端只负责读
///

#[repr(C)]
#[derive(Debug, Default)]
pub struct Event {
    name: String,
    length: i32,
}

impl Event {
    fn echo(&mut self) {
        println!("{:?}", self);
    }
}

// 仅仅业务处理逻辑
type RustCallback = extern "C" fn(*mut Event);

// 如果需要再Callback里面引用上下文(如 发送对象到外部)
// 可以有三种写法
// 1. 传递需要的Rust环境变量参数
type RustCallbackWithContext = extern "C" fn(RustContext: *mut Context, dataFromC: *mut Event);
// 对应C的代码 typedef void (*rust_callback)(void*, CEvent*);

// 2. 传递结构体，callback里面调用方法
// type RustCallbackWithContext = extern "C" fn(ptr: *mut Object, dataFromC: *mut Event);
// let ptr = unsafe {
// assert!(!ptr.is_null());
// &mut *ptr
// };
// ptr.populate(event);

// 3. 传递闭包, 需要额外转换

extern "C" {
    pub fn register_callback(arg1: *mut Event, arg2: RustCallback) -> ::std::os::raw::c_int;
    pub fn trigger_callback();
}

extern "C" fn callback(e: *mut Event) {
    unsafe {
        // ** //
        (*e).name = String::from("Hello World");
        (*e).length = 3;
        (*e).echo();
    }
}

// https://doc.rust-lang.org/nomicon/ffi.html
pub fn run() {

    // 这里注意用法
    let mut rust_object = Box::new(Event::default() );
    unsafe {
        // register_callback(Box::into_raw(Box::new(e)), callback);
        // ** //
        register_callback(&mut *rust_object, callback);
        trigger_callback();
    }
}