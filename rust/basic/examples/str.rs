// String:  String是一个可变的、堆上分配的UTF-8的字节缓冲区
// &String:  &String <--> &str
// &str: 1."Hello World"字面值常量,指向静态内存, 2: 由&String自动转换而来(实现了deref trait) 
// 如果是从String解引用而来的，则指向堆上，如果是字面值，则指向静态内存。
fn main() {

    let mut a = "Hello World".to_string();
    a.push_str("string");

    // *const string 指向的数据不可变更
    // 所以 *const string 不能引用 &mut String
    let raw_p = &a as *const String;

    println!("{:?}", raw_p); // 0x7ffee9ec9410
    unsafe {
        // 只能在unsafe中调用
        println!("{:?}", *raw_p); // 0x7ffee9ec9410
        // (*raw_p).push_str(" add nwe string"); // `raw_p` is a `*const` pointer, so the data it refers to cannot be borrowed as mutable
        println!("{:?}", *raw_p); // 0x7ffee9ec9410
    }
}