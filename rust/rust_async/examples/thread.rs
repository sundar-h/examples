use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// [Rust并发编程之多线程](https://blog.51cto.com/u_15127658/2783191)
/// [Rust 的绅士介绍](http://llever.com/gentle-intro/7-shared-and-networking.zh.html)
fn main() {
    // new_thread_channel_cond();
    thread_park();
}

fn new_thread() {
    // 产生新线程
    let counter = Arc::new(Mutex::new(0));

    // Arc: 引用计数
    let counter2 = counter.clone();
    let handler = thread::spawn(move || {
        let mut i = counter2.lock().unwrap();
        *i = *i + 1;
        let current_thread = thread::current();
        println!(
            "exit sub thread: {:?}, {:?}",
            current_thread.id(),
            current_thread.name()
        );
    });

    // 等待子线程结束
    handler.join().unwrap();
    let counter = counter.lock().unwrap();
    println!("{}", counter);
}

// 原子变量; 类比golang的atomic
fn new_thread_atomic() {
    let counter = Arc::new(AtomicU32::new(0));

    let counter2 = counter.clone();
    let handler = thread::spawn(move || {
        counter2.fetch_add(1, Ordering::SeqCst);
    });

    counter.fetch_add(1, Ordering::SeqCst);
    handler.join();
    counter.load(Ordering::SeqCst);
}

// 管道
fn new_thread_channel() {
    use std::sync::mpsc::channel;
    let (sender, receiver) = channel();
    let sender2 = sender.clone();

    thread::spawn(move || {
        sender2.send(123).unwrap();
    });
    thread::spawn(move || {
        sender.send(456).unwrap();
    });

    // mpsc: Multiple Producer Single Consumer: 多生产者 单接收者
    while let Ok(res) = receiver.recv() {
        println!("{:?}", res);
    }
}

// 条件变量能够阻塞一个线程，让它不消耗 CPU。直到一个事件触发，线程再继续执行。
// 条件变量往往和一个 bool类型及一个 mutex 关联
fn new_thread_condvar() {
    use std::sync::Condvar;

    let mutex_condvar = Arc::new((Mutex::new(false), Condvar::new()));
    let m_c = mutex_condvar.clone();

    thread::spawn(move || {
        println!("sub thread start...");
        let (lock, cvar) = &*m_c;
        let mut started = lock.lock().unwrap();
        *started = true;
        thread::sleep(Duration::from_secs(5));
        cvar.notify_all(); // 唤醒条件变量等待者
        println!("sub thread finished...");
    });
    println!("main thread start...");

    let (lock, cvar) = &*mutex_condvar;
    let mut started = lock.lock().unwrap();
    println!("main thread begin wait...");
    while !*started {
        // 等待条件变量被唤醒，且等待关注的业务参数为真。这里需要注意，要在循环中判断started，因为条件变量被唤醒时，有可能业务条件并未为true
        println!("loop wait...");
        started = cvar.wait(started).unwrap();
    }
}

// 配置线程
fn new_thread_with_builder() {
    let handler = thread::Builder::new()
        .name("child1".to_string())
        .spawn(|| {
            println!("sub thread with builder {:?}", thread::current());
        })
        .unwrap();

    handler.join().unwrap();
}

// 线程的暂停与恢复
// thread::park()
// let sub_thread = thread::current();
// sub_thread.unpark()
fn thread_park() {
    let counter = Arc::new(Mutex::new(0));
    let counter2 = counter.clone();

    let (tx, rx) = std::sync::mpsc::channel();
    ticker();

    let tx2 = tx.clone();
    thread::spawn(move || {
        let sub_thread = thread::current();
        println!(
            "park thread: {:?}, {:?}",
            sub_thread.id(),
            sub_thread.name()
        );
        // send 从不阻塞
        // 对比 mpsc::sync_channel(0) 参数为0为阻塞发送
        // 对比golang的channel
        tx.send(sub_thread);
        thread::park();

        // thread::sleep(Duration::from_secs(3));
        println!("exit sub_thread");
    });

    // 阻塞接收
    let sub_thread = rx.recv().unwrap();
    println!("received sub thread data, sleep 3 second to unpark sub thread");
    thread::sleep(Duration::from_secs(3));

    sub_thread.unpark();
    println!("exit main");
}

fn ticker() {
    thread::spawn(|| loop {
        println!("********************");
        thread::sleep(Duration::from_secs(1));
    });
}
