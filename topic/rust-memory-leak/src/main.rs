// 目前默认内存分配器是不确定的，但是确保cdylibs和staticlibs 是System(即: 操作系统提供的malloc和free)
// https://doc.rust-lang.org/std/alloc/index.html
// Box<T> and Vec<T>.
#[allow(dead_code)]
fn forget_string() {
    let s = String::from("ksjflkja");
    std::mem::forget(s);
}

#[allow(dead_code)]
fn forget_vec() {
    // This `Vec` will allocate memory through `GLOBAL` above
    let mut v = Vec::new();
    v.push(1);
    std::mem::forget(v);
}

// valgrind --leak-check=full ./target/debug/rust_memory_leak

#[allow(dead_code)]
fn forget_box_int() {
    let b = Box::new(1);
    std::mem::forget(b);
}

#[allow(dead_code)]
fn ok_int() {
    let b = Box::new(1);
    println!("{}", b);
}

#[allow(dead_code)]
fn forget_string_ok() {
    let mut s = String::from("ksjfklfjalk");
    s.push_str("ksjf");
    s.push_str("ksjf");
    s.push_str("ksjf");
    s.push_str("ksjf");
    println!("{}", s);
}

fn main() {
    // forget_string();
    // forget_vec();
    // forget_box_int();
    // ok_int();
    forget_string_ok();
}
