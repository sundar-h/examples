trait Animal {
    fn walk(&self) {
        println!("walk");
    }
}

impl<'a> dyn Animal + 'a {
    fn eat(&self) {
        println!("eat");
    }
}

trait People: Animal {
    fn think(&self) {
        println!("think");
    }
}

struct A;

impl People for A {
    fn think(&self) {
        println!("A::think");
    }
}

impl Animal for A {
    fn walk(&self) {
        println!("A::walk");
    }
}


fn demo() -> Box<dyn People> {
    Box::new(A)
}

pub fn run() {
    let a = demo();

    a.walk();
    a.think();
    a.eat();

    // let a = a as Box<dyn Animal>;

    // let b = a as Box<dyn Animal>;
    // b.eat();
}