use futures::executor::block_on;
use futures::{Future, Stream};
use std::ops::Sub;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

fn main() {
    ticker();

    let r = run();
    block_on(r);
}

async fn run() {
    let f = MyFuture {};
    f.await;
}

struct MyFuture {}

impl MyFuture {
    fn leaf_future(&self) -> u32{
        println!("step into leaf_future of MyFuture");
        std::thread::sleep(Duration::from_secs(3));

        println!("step out {}", Instant::now().elapsed().as_secs());
    }

    // fn leaf_stream(&self) {
    //     let now = Instant::now();
    //     let timeout = Duration::from_secs(10);
    //
    //     loop {
    //         self.leaf_future();
    //         if now.elapsed().sub(timeout).is_zero() {
    //             break;
    //         }
    //         println!("loop stream once");
    //     }
    // }
}

impl Future for MyFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let output = self.leaf_future();
        Poll::Ready(())
    }
}

impl Stream for MyFuture {
    type Item = ();

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.poll(cx) {
            Poll::Pending => ,
            Poll::Ready(()) => ,
        }
        // self.leaf_stream();
        // Poll::Ready(Some(()))
    }
}

fn ticker() {
    std::thread::spawn(|| loop {
        std::thread::sleep(Duration::from_secs(1));
        println!("*********************");
    });
}
