/// 探究多态对象 参数和返回值
fn main() {
    // turbo-fish 语法指定T类型
    foo1::<i32>(1); // ---> foo1
                    // 直接使用
                    // foo1(x); // ---> foo1

    // impl Trait 不能使用turbo-fish语法
    // foo2::<i32>(x); // ---> foo2
    foo2(3); // ---> foo2
}

trait Trait {}

impl Trait for i32 {}

/// # 参数多态
/// 写法一
/// 使用方法1: fool::<i32>(x)
/// 使用方法2: fool(x)
fn foo1<T: Trait>(_: T) {
    println!("foo1");
}

/// 写法二
/// 使用方法: fool(x)
/// 使用方法1: 不能使用此方法 fool::<i32>(x)
fn foo2(_: impl Trait) {
    println!("foo2")
}

/// # 返回多态
#[warn(dead_code)]
fn return_a_trait_object1() -> Box<dyn Trait> {
    Box::new(1)
}
#[warn(dead_code)]
fn return_a_trait_object2() -> impl Trait {
    5
}
