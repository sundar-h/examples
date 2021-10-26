use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_char;

/// [Rust FFI 实践](https://www.jianshu.com/p/f76631edbbfd)
/// [Rust functions that return allocated strings](http://jakegoulding.com/rust-ffi-omnibus/string_return/)
/// [CString](https://doc.rust-lang.org/std/ffi/struct.CString.html)
/// [CStr](https://doc.rust-lang.org/std/ffi/struct.CStr.html)
///
/// 对象字符串 所有权再Rust和C之间的转移
/// 1. 首先，一个对象如果传递给了调用者，那么所有权会转移到调用者，这个是由 `Box::into_raw(Box::new(....))` 自动完成的。
/// 2. 其次，借出的对象一旦重新返回Rust,那么所有权就转移回Rust了，这个也是由 `*Box::from_raw(...)` 自动完成的。
/// 3. 如果想由调用者决定是否获取所有权和何时释放, 使用`&*tensor`避免获取所有权
///
///
/// ```
/// #include <stdio.h>
/// #include <stdint.h>
///
/// extern char *
/// theme_song_generate(uint8_t length);
///
/// extern void
/// theme_song_free(char *);
///
/// int main(void) {
///   char *song = theme_song_generate(5);
///   printf("%s\n", song);
///   theme_song_free(song);
/// }
/// // There’s not much interesting for the C version: the char * is returned, can be printed, and then is transferred back to be freed
/// ```
///
///
///
/// CString: --> Into<Vec<u8>> 所有类型 (eg: String, &str)

#[no_mangle]
pub extern "C" fn theme_song_generate(length: u8) -> *mut c_char {
    let c_str_song = CString::new("Hello World").expect("To CString");
    // 这里不能这么使用
    // c_str_song.as_ptr()

    // 需要转移所有权,
    // 特别注意:
        // 1. 这个返回的字符串，必须再次被返回给到Rust
        // 以便Rust调用哦CString::from_raw()以正确的释放内存
        // 2. into_raw()返回的字符串 不能使用C的free函数来释放
        // 3. 并且C侧,不能改变into_raw()串的长度
    c_str_song.into_raw()
}

#[no_mangle]
pub extern "C" fn theme_song_generate2(length: u8) -> Option<*mut c_char> {
    None
}

// 接受字符串并接受所有权 Rust负责销毁
#[no_mangle]
pub extern "C" fn theme_song_free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

// pub struct Event {
//     pub id: String,
//     pub source: String,
//     pub protocol: String,
//     pub metadata: HashMap<String, String>,
//     pub payload: Vec<u8>,
//     pub create_at: f64,
//     pub process_at: f64,
// }
