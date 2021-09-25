// use std::sync::Arc;

// 异步编程练习
// 本节目标
// 自定义实现Feature(一次异步值), Stream(可重复的异步值), Sink(一次或多次异步值)

fn main() {
    println!("Hello, world!");

    // std::future::Future;
    // std::stream::Stream;
    // std::io::Sink

    // futures::future::Future;
    // futures::Future;
    // futures::stream::Stream;
    // futures::sink::Sink;
}

fn printA() -> Box<dyn Custom<Item = u32>> {
    let a = A { item: 3u32 };
    Box::new(a)
}

trait Custom {
    type Item;

    fn a(&self) -> Option<Self::Item>;
}

struct A {
    item: u32,
}

impl Custom for A {
    type Item = T;

    fn a(&self) -> Option<Self::Item> {}
}
