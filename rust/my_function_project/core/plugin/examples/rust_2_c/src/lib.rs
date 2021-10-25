#[no_mangle]
pub extern "C" fn send_str(_: Vec<String>) {}
// #[no_mangle]
// pub extern "C" fn send_str2(_: &[&str]) {}
#[no_mangle]
pub extern "C" fn send_byte2(_: &Vec<u8>) {}

// slice 类型不支持
// #[no_mangle]
// pub extern "C" fn send_byte(_: &[u8]) {}


// #include <stdarg.h>
// #include <stdbool.h>
// #include <stdint.h>
// #include <stdlib.h>

// typedef struct Vec_String Vec_String;

// typedef struct Vec_u8 Vec_u8;

// void send_str(struct Vec_String);

// void send_byte2(const struct Vec_u8*);


#[repr(C)]
pub struct config {
    name: String,
    lang_type: String,
    plugin_type: String,
}