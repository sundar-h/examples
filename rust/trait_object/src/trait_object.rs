// Base
pub trait Super {
    fn super_call(&self) {
        println!("super_call");
    }
}

// Derived
pub trait Sub: Super {
    fn sub_call(&self) {
        println!("sub_call");
    }
}

impl<'a> dyn Super + 'a {
    fn dyn_super_call(&self) {
        println!("dyn_super_call");
    }
}

struct SSuper;

impl Super for SSuper {}

impl Drop for SSuper {
    fn drop(&mut self) {
        println!("SSuper::Drop");
    }
}

struct SSub;

impl Super for SSub {}

impl Sub for SSub {}

// 标准语法是不行的
fn wants_upcast(s: &dyn Super) {
    s.super_call();
    s.dyn_super_call();
}

fn wants_upcast2(s: Box<dyn Super>) {
    s.super_call();
    s.dyn_super_call();
}

fn call_method() {
    // 对象不可以使用 dyn trait的方法
    let s = SSuper;
    s.super_call();

    {
        // dyn trait对象可以使用对象的方法
        let s = &SSuper as &dyn Super;
        s.super_call();
        s.dyn_super_call();
    }
}

fn return_dyn_super() -> Box<dyn Super> {
    Box::new(SSuper)
}

fn upcast() {
    // 向上类型转换: 需要 #![feature(trait_upcasting)]支持
    // [RFC](https://doc.rust-lang.org/nightly/unstable-book/language-features/trait-upcasting.html)
    wants_upcast(&SSub);
    wants_upcast2(Box::new(SSub));
}

pub fn run() {
    // call_method();
    // upcast();
    // 堆上的指针对象，生命周期结束以后，也会自动调用Drop
    let _d_super = return_dyn_super();
}
