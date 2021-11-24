use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::os::raw::c_int;

/// * 基础类型对应使用std::os::raw::库(或者libc 区别是libc是no std依赖的)对应的类型
/// * 结构体对应需要手动定义
///     rust中结构体默认的内存表示和c并不兼容。如果要将结构体传给ffi函数，请为rust的结构体打上标记：
///     如果使用#[repr(C, packed)]将不为此结构体填充空位用以对齐。
/// * enum对应同结构体
#[repr(C)]
struct RustObject {
    i: c_int,
    s: c_char,
}

/// 外部链接库, 对应libpass.[a/so/dll/dylib]
#[link(name = "cdemo")]
extern "C" {
    // 传递不可变
    #[link_name = "pass_int"]
    fn c_pass_int(a: c_int);

    #[link_name = "pass_str_owner"]
    fn c_pass_str_owner(a: *const c_char);
    #[link_name = "pass_str_ref"]
    fn c_pass_str_ref(a: *const c_char);
    // 传递复杂类型; Box::into_raw(Box::new(....))和*Box::from_raw()
    // 完全由调用者来决定是否释放,&*tensor 的方式不获取所有权
    // 使用repr(C) 两种语言直接传递结构体
    // fn pass_struct(a: *mut RustObject);
    // fn pass_bytes(a: c_int);
    // fn pass_void_int(a: c_void);
    // fn callback_pass();
    //
    // fn return_int() -> c_int;
    #[link_name = "return_str_static"]
    fn c_return_str_static() -> *const c_char;
    #[link_name = "return_str_heap"]
    fn c_return_str_heap() -> *const c_char;
    #[link_name = "return_str_stack"]
    fn c_return_str_stack() -> *const c_char;
    // fn return_bytes() -> c_char;
    // fn return_struct() -> c_void;
}

#[allow(dead_code)]
fn pass_int_wrapper(a: i32) {
    unsafe { c_pass_int(a as c_int) }
}

// 传递所有权并且 String 不可变, 如果变更需要使用
#[allow(dead_code)]
fn pass_str_owner_wrapper() {
    let s = CString::new("I'm Rust String").unwrap();
    // CString.as_ptr() 类似strdup, 堆分配空间，并且赋值, 返回新内存
    // https://kaisery.github.io/trpl-zh-cn/ch15-02-deref.html
    // 实现了Deref trait,(*t -->编译器会替换为 *(t.deref()) 只会替换一次,不会无限递归替换)
    // 每次当我们在代码中使用 * 时， * 运算符都被替换成了先调用 deref 方法再接着使用 * 解引用的操作，且只会发生一次，不会对 * 操作符无限递归替换，
    // 函数和方法的隐式 Deref 强制转换 可以使任意多次(发生在编译期 &a --> &target)
    // 其将实现了 Deref 的类型的引用转换为原始类型通过 Deref 所能够转换的类型的引用
    unsafe { c_pass_str_owner(s.as_ptr()) }
}

#[allow(dead_code)]
fn return_str_wrapper() {
    let s = unsafe {
        let s = c_return_str_heap();
        CStr::from_ptr(s)
    };
    // 检测并转换utf8字符 (Rust的string是utf8的，但C不是)
    println!("received from C: {}", s.to_string_lossy().into_owned())
}

#[allow(dead_code)]
fn leak_return_str_wrapper() {
    let s = unsafe {
        let s = c_return_str_heap();
        CStr::from_ptr(s)
    };
    // 检测并转换utf8字符 (Rust的string是utf8的，但C不是)
    println!("received from C: {:?}", s);
}

pub fn run() {
    // pass_int_wrapper(3);

    // pass_str_owner_wrapper();
    // return_str_wrapper();
    // for i in 1..10 {
    leak_return_str_wrapper();
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
