#[derive(Debug)]
struct A {
    name: String,
}

fn main() {
    let a = A {
        name: String::from("lalala"),
    };
    let a = Some(a);

    // -------------------------a--------------------
    // 这里 if let Some a的所有权已经转移
    // 之后的代码块 已经不能再使用a了
    // if let Some(i) = a {
    //     println!("{:?}", i);
    // }

    // 错误: value borrowed here after partial move
    // if a.is_none() {
    //     println!("YES");
    // }
    // ----------------------------------------------

    // 正确用法 应该是引用而非借用
    // if let Some(i) = &mut a
    if let Some(i) = &a {
        println!("{:?}", i);
    }

    // 上面是引用，所以之后a还可以使用 切Option不为None
    if a.is_none() {
        println!("YES");
    }

    // 同上 用法
    // if let Some(i) = a.as_ref() {
    //     println!("{:?}", i);
    // }

    // println!("{:?}", a);
}
