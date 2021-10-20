use std::mem::size_of;

trait SomeTrait{}

// &dyn trait VS Box<dyn trait>
// &dyn trait VS impl trait --> 一个是静态派发 一个是动态派发

// 类型系统
// 1. 什么使用使用 Box<T> VS 什么时候使用引用 &T
//     * 理解的关键是 生命周期和所有权系统;
//         * 引用没有所有权
//         * Box 数据存储再堆上

struct A;

// 案例一
// 考虑到这个案例, 返回函数内部的局部变量的引用
// 这里不设置 'static A
// 不通过 可以改为 Box<A>
// fn return_a_reference() -> &A{
//     &A
// }

pub fn run() {
    // println!("&dyn trait: {}", size_of::<&dyn SomeTrait>());
    // println!("{}", return_a_reference());

    println!("======== The size of different pointers in Rust: ========");
    println!("&dyn Trait:------{}", size_of::<&dyn SomeTrait>());
    println!("&[&dyn Trait]:---{}", size_of::<&[&dyn SomeTrait]>());
    println!("Box<Trait>:------{}", size_of::<Box<dyn SomeTrait>>());
    println!("Box<Box<Trait>>:-{}", size_of::<Box<Box<dyn SomeTrait>>>());
    println!("&i32:------------{}", size_of::<&i32>());
    println!("&[i32]:----------{}", size_of::<&[i32]>());
    println!("Box<i32>:--------{}", size_of::<Box<i32>>());
    println!("&Box<i32>:-------{}", size_of::<&Box<i32>>());
    println!("[&dyn Trait;4]:--{}", size_of::<[&dyn SomeTrait; 4]>());
    println!("[i32;4]:---------{}", size_of::<[i32; 4]>());
}

// ======== The size of different pointers in Rust: ========
// &dyn Trait:------16
// &[&dyn Trait]:---16
// Box<Trait>:------16
// Box<Box<Trait>>:-8
// &i32:------------8
// &[i32]:----------16
// Box<i32>:--------8
// &Box<i32>:-------8
// [&dyn Trait;4]:--64
// [i32;4]:---------16