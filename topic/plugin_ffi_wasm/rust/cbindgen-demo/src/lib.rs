#[no_mangle]
pub extern "C" fn received_bytes(payload: *const u8) {}

#[no_mangle]
pub extern "C" fn hello_rust() -> *const u8 {
    "Hello World!\0".as_ptr()
}
