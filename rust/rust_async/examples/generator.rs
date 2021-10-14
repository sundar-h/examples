// #![feature(generator_trait)]
// #![feature(generators)]
// use std::{ops::GeneratorState, pin::Pin};

// 无栈协程实现

fn main() {
    let mut generator = GeneratorA::start(4);

    if let GeneratorState::Yield(n) = generator.resume() {
        println!("Got value {}", n);
    }
    if let GeneratorState::Complete(_) = generator.resume() {
        ()
    }
}

// fn example() {

//     let a: i32 = 4;

//     let mut gen = move || {
//         println!("Hello");
//         yield a * 2;
//         println!("world!");
//     };

//     if let GeneratorState::Yielded(n) = gen.resume() {
//         println!("Got value {}", n);
//     }

//     if let GeneratorState::Complete(()) = gen.resume() {
//         ()
//     };

//     // if let GeneratorState::Yielded(n) = Pin::new(&mut gen).resume(()) {
//     //     println!("Got value {}", n);
//     // }

//     // if let GeneratorState::Complete(()) = Pin::new(&mut gen).resume(()) {
//     //     ()
//     // };
// }


enum GeneratorState<Y, R> {
    Yield(Y),
    Complete(R)
}

trait Generator {
    type Yield;
    type Return;

    fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return>;
}

enum GeneratorA {
    Enter(i32), // 初始化
    Yield(i32), // 暂停点
    Exit, // 退出状态
}

impl GeneratorA {
    fn start(a: i32) -> Self {
        GeneratorA::Enter(a)
    }
}

impl Generator for GeneratorA {
    type Yield = i32;
    type Return = ();
    fn resume(&mut self) -> GeneratorState<i32,()> {
        match std::mem::replace(self, GeneratorA::Exit) {
            GeneratorA::Enter(a) => {
                println!("Hello");
                let a = 2 * a;

                *self = GeneratorA::Yield(a);
                GeneratorState::Yield(a)
            }
            GeneratorA::Yield(_) => {
                println!("World");

                *self = GeneratorA::Exit;
                GeneratorState::Complete(())
            }
            GeneratorA::Exit => panic!("exited generator")
        }
    }
}