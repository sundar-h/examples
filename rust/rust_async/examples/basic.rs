use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    // 调用async函数，立刻返回一个Future
    let future = hello_world(); // Nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed
}
