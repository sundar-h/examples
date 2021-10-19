/* automatically generated by rust-bindgen 0.59.1 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Event {
    pub topic: *const ::std::os::raw::c_char,
    pub body: *const ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_Event() {
    assert_eq!(
        ::std::mem::size_of::<Event>(),
        16usize,
        concat!("Size of: ", stringify!(Event))
    );
    assert_eq!(
        ::std::mem::align_of::<Event>(),
        8usize,
        concat!("Alignment of ", stringify!(Event))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Event>())).topic as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Event),
            "::",
            stringify!(topic)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Event>())).body as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Event),
            "::",
            stringify!(body)
        )
    );
}
pub type RustCallback = ::std::option::Option<unsafe extern "C" fn(e: *mut Event)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Object {
    _unused: [u8; 0],
}
extern "C" {
    pub fn init(arg1: *mut Object) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn error() -> *const ::std::os::raw::c_char;
}
