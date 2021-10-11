#![feature(generator_trait)]
#![feature(generators)]
use std::ops::GeneratorState;

fn main() {
    let a: i32 = 4;

    let mut gen = move || {
        println!("Hello");
        yield a * 2;
        println!("world!");
    };

    if let GeneratorState::Yielded(n) = gen.resume() {
        println!("Got value {}", n);
    }

    if let GeneratorState::Complete(()) = gen.resume() {
        ()
    };
}
