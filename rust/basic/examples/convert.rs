// AsRef/AsMut

/// [Rust 中的隐式类型转换](https://juejin.cn/post/6999829181680844831)
/// ****************************************************************//
///   *  自动类型转换
///   * AsRef: 对于一个类型为 T 的对象 foo，如果 T 实现了 AsRef<U>，那么，foo 可执行 .as_ref() 操作，即 foo.as_ref()。操作的结果，我们得到了一个类型为 &U 的新引用。
///   1. 与 Into<T> 不同的是，AsRef<T> 只是类型转换，foo 对象本身没有被消耗；
///   2. T: AsRef<U> 中的 T，可以接受 资源拥有者（owned）类型，共享引用（shared referrence）类型 ，可变引用（mutable referrence）类型。
///
///   *AsMut: 它是 AsRef<T> 的可变（mutable）引用版本。
/// ****************************************************************//

// * AsRef<T> --> &T
// * AsMut<T> --> &mut &T

// 引用和借用: as_ref, borrow
// as_ref 是转引用函数, 将具有所有权对象转换成引用对象,
// 目前: Option, Box, Result 这三种类型默认提供支持as_ref.

// pub trait AsRef<T: ?Sized> {
//     /// Performs the conversion.
//     #[stable(feature = "rust1", since = "1.0.0")]
//     fn as_ref(&self) -> &T;
// }

// pub trait AsMut<T: ?Sized> {
//     /// Performs the conversion.
//     #[stable(feature = "rust1", since = "1.0.0")]
//     fn as_mut(&mut self) -> &mut T;
// }

#[derive(Debug)]
struct Person {
    name: String,
}

// into 对泛型的约束
impl Person {
    fn new<S: Into<String>>(name: S) -> Person {
        Person { name: name.into() }
    }
}

fn main() {
    let name: String = String::from("Hello world");
    let p = Person::new(&name);

    println!("{:?}", p);
}

// From<T>: 对于类型为 U 的对象 foo，如果它实现了 From<T>，那么，可以通过 let foo = U::from(bar) 来生成自己。这里，bar 是类型为 T 的对象。
// Into<T>: 对于一个类型为 U: Into<T> 的对象 foo，Into 提供了一个函数：.into(self) -> T，调用 foo.into() 会消耗自己（转移资源所有权），生成类型为 T 的另一个新对象 bar。
