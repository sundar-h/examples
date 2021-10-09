use std::mem::{align_of, size_of};


struct Data {
    a: i32,
    b: i32,
    c: i16,
}

fn main() {
    println!("{}, {}", size_of::<Data>(), align_of::<Data>())
}