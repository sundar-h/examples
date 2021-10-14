// 探索Rust的胖指针
// &dyn trait 或者 Box<&dyn trait>, slice or Array
// trait 对象指针的布局是这样的：
//
//  * 前 8 字节指向 trait 对象的数据（data）。
//  * 第二个 8 字节指向 trait 对象的 vtable。

use std::mem::align_of;

trait Test {
    fn add(&self) -> i32;
    fn sub(&self) -> i32;
    fn mul(&self) -> i32;
}

// 实际上会调用虚表中对应的函数，并将 data 作为函数的第一个参数传入。
struct FatPointer<'a> {
    /// A reference is a pointer to an instantiated `Data` instance
    /// 引用是指向一个实例化的' Data '实例的指针
    data: &'a mut Data,
    /// Since we need to pass in literal values like length and alignment it's
    /// easiest for us to convert pointers to usize-integers instead of the other way around.
    /// 因为我们需要传入长度和对齐等文字值
    /// 对我们来说，将指针转换为使用整数最简单，而不是相反。
    /// 好像类似这种*const 可以避免类型或者生命周期检查
    vtable: *const usize, // 虚指针: 指向虚函数表; C++中 虚指针存在于对象实例地址的最前面
                          // C++中虚函数表数据类, 对象头部持有一些列类的虚函数表的指针
}

struct Data {
    a: i32,
    b: i32,
}

fn add(s: &Data) -> i32 {
    s.a + s.b
}
fn sub(s: &Data) -> i32 {
    s.a - s.b
}
fn mul(s: &Data) -> i32 {
    s.a * s.b
}

fn main() {
    let mut data = Data { a: 3, b: 2 };

    // vtable is like special purpose array of pointer-length types with a fixed
    // format where the three first values contains some general information like
    // a pointer to drop and the length and data alignment of `data`.
    // Vtable类似于指针长度类型的特殊用途数组
    // 格式，其中第一个值包含一些一般信息，如
    // 一个指针的下落和长度和数据对齐的'数据'。

    let vtable = vec![
        0,                      // pointer to `Drop` (which we're not implementing here)
        size_of::<Data>(),      // length of data 数据域偏移量
        align_of()::<Data>(),   // alignment of data 内存对其  align_of返回ABI要求的类型的最小对齐方式。

        // we need to make sure we add these in the same order as defined in the Trait.
        // 我们需要确保我们按照Trait中定义的顺序添加这些。
        add as usize,       // 函数指针
        sub as usize,       // 函数指针
        mul as usize,       // 函数指针
    ];

    let fat_pointer = FatPointer {
        data: &mut data,
        vtable: vtable.as_ptr(0),
    };

    let test = unsafe { std::mem::transmute::<FatPointer, &dyn Test>(fat_pointer) };

    println!("Add: 3 + 2 = {}", test.add());
    println!("Sub: 3 - 2 = {}", test.sub());
    println!("Mul: 3 * 2 = {}", test.mul());
}
