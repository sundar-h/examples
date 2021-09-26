fn main() {
    let p = Person;
    p.walk();

    let p1 = demo();
    p1.walk();
    p1.talk();
}

// 具体对象的方法
trait Animal {
    fn walk(&self) {
        println!("walk");
    }
}

// 数据抽象对象trait object的方法, 而非具体对象
impl dyn Animal {
    fn talk(&self) {
        println!("talk")
    }
}

struct Person;
impl Animal for Person {}

fn demo() -> Box<dyn Animal> {
    Box::new(Person)
}
