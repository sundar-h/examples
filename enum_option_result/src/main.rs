/// [Rust版本指南](https://rustwiki.org/zh-CN/edition-guide/rust-2018/trait-system/impl-trait-for-returning-complex-types-with-ease.html)
fn main() {
    // 如何方便的揭开Option和Result
    // if let
    // while let
    // 组合子 .and() .or()
    let x = Some(3i32);
    // x.and_then(f)
    println!("{:?}", x.and::<i32>(Some(9i32)));
    println!("{:?}", x.and::<i32>(x));
    // println!("Hello, world!");
}
