use libloading::{Library, Symbol};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uchar};
// use std::ptr::null_mut;

extern "C" {
    pub fn Echo(arg1: *mut People) -> *mut People;
    pub fn Init();
}

pub struct People {
    pub name: *mut c_char,
    pub payload: *mut c_char,
    pub content: *mut c_uchar, // *const u8 or vec[u8]
    pub age: c_int,
}

impl People {
    fn free(ptr: *mut People) {
        if ptr.is_null() {
            return;
        }
        unsafe {
            Box::from_raw(ptr);
        }
    }
}

type EchoSymbol = extern "C" fn(arg1: *mut People) -> *mut People;
type InitSymbol = extern "C" fn();

fn main() {
    unsafe {
        let lib = Library::new("../go/echo").unwrap();

        // 转移内存管理所有权
        let echo: Symbol<EchoSymbol> = lib.get(b"Echo").unwrap();
        let p = People {
            name: CString::new("Name~~").unwrap().into_raw(),
            age: 99999,
            payload: CString::new("Payload~~").unwrap().into_raw(),
            content: CString::new("Payload~~")
                .unwrap()
                .into_bytes_with_nul()
                .as_mut_ptr(),
        };
        // TODO: 定义 People::from() --> 转换为Rust 有一个库 提供一个宏可以自动转换
        let p2 = echo(Box::into_raw(Box::new(p)));
        let p2 = Box::from_raw(p2);
        println!("{:?}", p2.age);
        println!("{:?}", CStr::from_ptr(p2.name));
        println!("{:?}", CStr::from_ptr(p2.payload));
        // println!("{:?}", CStr::from_ptr(p2.content));
        // println!("{:?}", CStr::from_bytes_with_nul(p2.content2));
    }
}
