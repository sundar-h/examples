use std::ops::DerefMut;



// ****************************************************************//
// {} --> std::fmt::Display
// {:?} --> std::fmt::Debug
// {:#?} --> 美化打印

/// [Formatting traits](https://doc.rust-lang.org/std/fmt/#formatting-traits)
/// * nothing ⇒ Display
/// * ? ⇒ Debug
/// * x? ⇒ Debug with lower-case hexadecimal integers
/// * X? ⇒ Debug with upper-case hexadecimal integers
/// * o ⇒ Octal
/// * x ⇒ LowerHex
/// * X ⇒ UpperHex
/// * p ⇒ Pointer
/// * b ⇒ Binary
/// * e ⇒ LowerExp
/// * E ⇒ UpperExp
///
/// todo: 验证
/// & VS as_ref()  
/// 1. &Option<_> -->  &Option<_>
/// 2. Option.as_ref() --> Option<&_>
///
/// 其它的实现了 impl Deref 的类型
/// 使用 & --> eg: &String to &str
/// ****************************************************************//


/// ****************************************************************//
///   * 裸指针(raw pointer) * 和 引用&T 异同
///   * 1. 引用总是安全的, 因为借用检查器保证了它指向一个有效的数据
///   * 2. 解引用一个裸指针只能通过不安全 代码块执行
/// ****************************************************************//

fn print_addr() {
    println!("********* 静态存储区 ***********");
    let s = "hello world";
    println!("静态存储区地址: {:p}", s); // 0x10edc4369 --> 静态存储区
    println!("静态存储区地址: {:p}", s.as_ptr()); // 0x10edc4369 --> 静态存储区
    println!("指向静态存储区地址的变量的地址: {:p}", &s); // 0x7ffee0e75168 --> 指向静态存储区的栈变量地址

    println!("\n******** String: 堆内存 ************");

    let a = String::from(s);
    // {:p}: std::fmt::Pointer: 所有的引用类型（&T和&mut T）都实现了这个trait。
    println!("内部堆地址: {:p}", a.as_ptr()); // 0x7fc861405ba0 --> 指向堆的栈变量地址
    println!("指向堆内存地址的变量(栈上)的地址: {:p}", &a); // 0x7ffee0e75290 -> --> 指向存储堆的*栈*变量地址
    println!("指向堆内存地址的变量(栈上)的地址: {:p}", &a as *const String); // 0x7ffee0e75290 --> 指向存储堆的*栈*变量地址

    println!("\n********* Move String ***********");
    // 移动是复用，内部堆地址还是同一个 证明没有发生clone或者重新分配内存
    let b = a;
    println!("{:p}", b.as_ptr()); // 0x7fc861405ba0 --> 指向堆的栈变量地址
    println!("{:p}", &b); // 0x7ffee0e75378 --> 指向存储堆的栈变量地址
}

fn main() {
    print_addr()
}
