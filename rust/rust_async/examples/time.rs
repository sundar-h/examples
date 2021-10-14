use std::ops::Sub;
use std::time::{Duration, Instant};

fn main() {
    let now = Instant::now();
    let timeout = Duration::from_secs(3);
    ticker();

    loop {
        if now.elapsed().sub(timeout).is_zero() {
            println!("timeout here");
            break;
        }

        println!("loop again");
    }

    println!("exit main");
}

fn ticker() {
    std::thread::spawn(|| loop {
        println!("***********");
        std::thread::sleep(Duration::from_secs(1));
    });
}
