use futures::{Stream, StreamExt};
use std::{
    pin::Pin,
    task::{Context, Poll},
};

#[tokio::main]
async fn main() {
    let c = Counter::new();

    let mut w = Wrapper::new(c);
    while let Some(i) = w.next().await {
        println!("{}", i);
    }
}
// #![feature(async_stream)]

// First, the struct:

/// A stream which counts from one to five
struct Counter {
    count: usize,
}

// we want our count to start at one, so let's add a new() method to help.
// This isn't strictly necessary, but is convenient. Note that we start
// `count` at zero, we'll see why in `poll_next()`'s implementation below.
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// Then, we implement `Stream` for our `Counter`:

impl Stream for Counter {
    // we will be counting with usize
    type Item = usize;

    // poll_next() is the only required method
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Increment our count. This is why we started at zero.
        self.count += 1;

        // Check to see if we've finished counting or not.
        if self.count < 6 {
            Poll::Ready(Some(self.count))
        } else {
            Poll::Ready(None)
        }
    }
}

struct Wrapper {
    couter: Counter,
}

impl Wrapper {
    fn new(couter: Counter) -> Self {
        Self { couter }
    }
}

impl Stream for Wrapper {
    type Item = usize;

    // poll_next() is the only required method
    // fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    // self.couter.poll_next_unpin(cx)
    // }

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.couter.poll_next_unpin(cx)
    }
}

struct WrapperToString {
    couter: Counter,
}

impl WrapperToString {
    fn new(couter: Counter) -> Self {
        Self { couter }
    }
}

// impl Stream for WrapperToString {
//     type Item = String;

//     fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
//         // self.couter.poll_next_unpin(cx)
//         match self.couter.poll_next_unpin(cx) {
//             Poll::Ready(msg) => {
//             }
//             Poll::Pending => {
//             }
//         }
//     }
// }
