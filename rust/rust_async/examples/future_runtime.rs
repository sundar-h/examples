use std::{collections::HashMap, sync::{Arc, Mutex, mpsc::Sender}, thread::{self, JoinHandle}};

// use futures::executor::block_on;
fn main() {
    // // async 把代码块重写为一个状态机:
    // // 其中每一个await 代表一次Future状态变更
    // let non_leaf_fut = async {
    //     let leaf_fut = Reactor::new_io(...);
    //     let result = leaf_fut.await;
    // };
    // // block_on 传递future给executor,并且阻塞当前线程知道所有子future完成
    // block_on(non_leaf_fut);
}

struct MyWaker {
    thread: thread::Thread,
}

pub struct Task {
    id: usize,
    reactor: Arc<Mutex<Reactor>>,
    data: u64,
}

enum TaskState {
    Ready,
    NotReady(Waker),
    Finished,
}

struct Reactor {
    dispatcher: Sender<Event>,
    handle: Option<JoinHandle<()>>,

    tasks: HashMap<usize, TaskState>,
}

enum Event {
    Close,
    Timeout(u64, usize),
}

impl Reactor {
}

fn block_on<F: Future>(mut future: F) -> F::Output {
    let mywaker = Arc::new(MyWaker{})
}